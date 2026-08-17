//! The fake instance: the second of the two observed boundaries.
//!
//! It is not the real instance and must never become one. It exists to be
//! driven — to accept, refuse, stall, and drop connections on command — and to
//! record what arrived, in what order, carrying what identities.
//!
//! # The control surface, in full
//!
//! One setter and one getter, which is the whole point:
//!
//! - [`FakeInstance::set_behaviour`] takes one [`Behaviour`], a flat enum. The
//!   instance answers every subsequent call according to it.
//! - [`FakeInstance::log`] returns everything that has arrived.
//!
//! Everything else on the type is a convenience over those two. A Kotlin
//! transcription is the same enum, the same two entry points, a gRPC server,
//! and the same byte copying proxy; there is deliberately no builder, no
//! generic, and no callback for a transcriber to have to interpret.
//!
//! # Why there is a proxy
//!
//! Two conditions cannot be expressed inside a gRPC handler. "Unreachable"
//! (A1, F2) and "the connection drops after the instance committed" (D1) are
//! both facts about the socket, so the instance listens behind a byte copying
//! proxy that can refuse a connection outright or tear a live one down.

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use altair_proto::v1;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;
use tonic::{Request, Response, Status, Streaming};

/// How the fake instance answers. One value at a time; set it, then act.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Behaviour {
    /// Every intent applies.
    Accept,
    /// Nothing connects. Connections are closed the moment they are accepted,
    /// which a client sees as a transport failure exactly as it would see a
    /// refused connection.
    Unreachable,
    /// The connection works and the token does not. Per DR-005 this is a wait,
    /// not a fault, and A2 and F3 turn on the client agreeing.
    Unauthenticated,
    /// The submission arrives and no answer ever comes. E1.
    Stall,
    /// The submission arrives, nothing is committed, and the connection is
    /// torn down. An ordinary wait; used to make a client retry.
    DropBeforeCommit,
    /// The submission arrives, the instance commits and records the
    /// acknowledgement it would have sent, and then the connection is torn
    /// down before that acknowledgement can reach the client. D1's condition.
    DropAfterCommit,
    /// Refuse any intent whose content carries one of these titles; apply
    /// everything else. E2, E3, E4, F4, F7.
    RefuseTitled(Vec<String>),
    /// Refuse the nth intent to arrive, counting from one across the whole run;
    /// apply every other. G1.
    RefuseNth(usize),
    /// Refuse everything as not available, with this free text detail. G4 runs
    /// twice with different detail and requires the person to see no
    /// difference.
    RefuseNotAvailable { detail: String },
    /// Apply edits, and say a concurrent change was retained. G2.
    ApplyWithConflict,
    /// Answer an edit with a recreation under a new identity. G3.
    RecreateOnEdit,
    /// Answer with a status a client has no way to classify as self clearing.
    /// F6, F8, F9.
    UnrecognisedCondition,
}

/// One submission, as it arrived.
#[derive(Debug, Clone)]
pub struct Submission {
    /// Position in the single arrival order shared with body streams.
    pub seq: u64,
    /// Whatever was in the `authorization` metadata, if anything.
    pub token: Option<String>,
    pub intents: Vec<v1::Intent>,
    /// What the instance answered, where it answered. One per intent, in the
    /// order submitted.
    pub acknowledgements: Vec<v1::Acknowledgement>,
    /// Whether the client could have received the answer. False for a stall
    /// and for either drop.
    pub answered: bool,
}

/// One completed body stream.
#[derive(Debug, Clone)]
pub struct BodyArrival {
    /// Position in the arrival order, taken when the stream *completed*, which
    /// is what C3 compares against.
    pub seq: u64,
    pub body_id: Vec<u8>,
    pub bytes: Vec<u8>,
}

/// One intent, flattened out of its submission into the global arrival order.
#[derive(Debug, Clone)]
pub struct IntentArrival {
    /// Position among intents, counting from zero. Ordering assertions use it.
    pub order: usize,
    /// Position of the carrying submission in the shared arrival order.
    pub seq: u64,
    pub token: Option<String>,
    pub intent: v1::Intent,
    pub acknowledgement: Option<v1::Acknowledgement>,
    /// Whether the client could have received the answer. False for a stall
    /// and for either drop, which is exactly the distinction D1 turns on.
    pub answered: bool,
}

impl IntentArrival {
    /// Whether the instance applied this intent.
    #[must_use]
    pub fn applied(&self) -> bool {
        matches!(
            self.acknowledgement
                .as_ref()
                .and_then(|ack| ack.outcome.as_ref()),
            Some(v1::acknowledgement::Outcome::Applied(_))
        )
    }

    /// Whether the instance refused this intent.
    #[must_use]
    pub fn refused(&self) -> bool {
        matches!(
            self.acknowledgement
                .as_ref()
                .and_then(|ack| ack.outcome.as_ref()),
            Some(v1::acknowledgement::Outcome::Refused(_))
        )
    }

    /// The recreation the instance answered with, where it answered with one.
    #[must_use]
    pub fn recreated(&self) -> Option<v1::Recreated> {
        match self
            .acknowledgement
            .as_ref()
            .and_then(|ack| ack.outcome.as_ref())
        {
            Some(v1::acknowledgement::Outcome::Recreated(recreated)) => Some(recreated.clone()),
            _ => None,
        }
    }

    /// The retained conflict the acknowledgement carries, where it carries one.
    #[must_use]
    pub fn conflict(&self) -> Option<v1::ConflictRetained> {
        match self
            .acknowledgement
            .as_ref()
            .and_then(|ack| ack.outcome.as_ref())
        {
            Some(v1::acknowledgement::Outcome::Applied(applied)) => applied.conflict.clone(),
            _ => None,
        }
    }
}

/// Everything that reached the instance.
#[derive(Debug, Clone, Default)]
pub struct Log {
    pub submissions: Vec<Submission>,
    pub bodies: Vec<BodyArrival>,
}

impl Log {
    /// Every intent in arrival order.
    #[must_use]
    pub fn intents(&self) -> Vec<IntentArrival> {
        let mut out = Vec::new();
        for submission in &self.submissions {
            for (index, intent) in submission.intents.iter().enumerate() {
                out.push(IntentArrival {
                    order: out.len(),
                    seq: submission.seq,
                    token: submission.token.clone(),
                    intent: intent.clone(),
                    acknowledgement: submission.acknowledgements.get(index).cloned(),
                    answered: submission.answered,
                });
            }
        }
        out
    }
}

#[derive(Default)]
struct Inner {
    behaviour: Option<Behaviour>,
    log: Log,
    /// Intent identity to the acknowledgement already issued for it. D3: a
    /// replay returns the original, unchanged.
    issued: HashMap<Vec<u8>, v1::Acknowledgement>,
    /// Entity identities the instance has actually committed. D1 and D4 count
    /// these to prove one entity rather than two.
    committed: HashSet<Vec<u8>>,
    /// How many intents have been decided, ever. `RefuseNth` counts here.
    decided: usize,
}

struct State {
    inner: Mutex<Inner>,
    seq: AtomicU64,
    generation: watch::Sender<u64>,
}

impl State {
    fn behaviour(&self) -> Behaviour {
        self.inner
            .lock()
            .unwrap()
            .behaviour
            .clone()
            .unwrap_or(Behaviour::Accept)
    }

    fn next_seq(&self) -> u64 {
        self.seq.fetch_add(1, Ordering::SeqCst)
    }

    /// Tear down every live connection. The generation is what the proxy tasks
    /// watch.
    fn drop_connections(&self) {
        self.generation.send_modify(|value| *value += 1);
    }
}

/// A controllable instance, listening on a local port.
pub struct FakeInstance {
    state: Arc<State>,
    address: SocketAddr,
}

impl FakeInstance {
    /// Start listening. The returned instance answers [`Behaviour::Accept`]
    /// until told otherwise.
    ///
    /// # Panics
    ///
    /// Panics if a local port cannot be bound, which is a broken harness
    /// rather than a conformance verdict.
    pub async fn start() -> Self {
        let (generation, generation_rx) = watch::channel(0_u64);
        let state = Arc::new(State {
            inner: Mutex::new(Inner::default()),
            seq: AtomicU64::new(0),
            generation,
        });

        let backend = TcpListener::bind(("127.0.0.1", 0))
            .await
            .expect("bind backend");
        let backend_address = backend.local_addr().expect("backend address");
        let service = v1::altair_server::AltairServer::new(Service {
            state: Arc::clone(&state),
        });
        tokio::spawn(async move {
            let incoming = tokio_stream::wrappers::TcpListenerStream::new(backend);
            let _ = tonic::transport::Server::builder()
                .add_service(service)
                .serve_with_incoming(incoming)
                .await;
        });

        let front = TcpListener::bind(("127.0.0.1", 0))
            .await
            .expect("bind front");
        let address = front.local_addr().expect("front address");
        let proxy_state = Arc::clone(&state);
        tokio::spawn(async move {
            proxy(front, backend_address, proxy_state, generation_rx).await;
        });

        Self { state, address }
    }

    /// Where clients should be pointed.
    #[must_use]
    pub fn url(&self) -> String {
        format!("http://{}", self.address)
    }

    /// The whole control surface, half one.
    pub fn set_behaviour(&self, behaviour: Behaviour) {
        let mut inner = self.state.inner.lock().unwrap();
        inner.behaviour = Some(behaviour);
    }

    /// The whole control surface, half two.
    #[must_use]
    pub fn log(&self) -> Log {
        self.state.inner.lock().unwrap().log.clone()
    }

    /// Every intent that has arrived, in order.
    #[must_use]
    pub fn intents(&self) -> Vec<IntentArrival> {
        self.log().intents()
    }

    /// Every body stream that completed, in order.
    #[must_use]
    pub fn bodies(&self) -> Vec<BodyArrival> {
        self.log().bodies
    }

    /// Entity identities the instance actually committed. Counting these is
    /// how "one entity, not two" is checked.
    #[must_use]
    pub fn committed_entities(&self) -> HashSet<Vec<u8>> {
        self.state.inner.lock().unwrap().committed.clone()
    }

    /// Tear down every live connection, without changing the behaviour.
    pub fn drop_connections(&self) {
        self.state.drop_connections();
    }

    /// Wait until at least `count` intents have arrived. Answers whether they
    /// did.
    pub async fn wait_for_intents(&self, count: usize, patience: Duration) -> bool {
        self.wait_until(patience, |instance| instance.intents().len() >= count)
            .await
    }

    /// Wait until `condition` holds of the instance. Answers whether it did.
    pub async fn wait_until<F>(&self, patience: Duration, condition: F) -> bool
    where
        F: Fn(&Self) -> bool,
    {
        let deadline = tokio::time::Instant::now() + patience;
        loop {
            if condition(self) {
                return true;
            }
            if tokio::time::Instant::now() >= deadline {
                return false;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    /// Let `window` pass, then answer whether `condition` held throughout.
    /// The shape every "and nothing else happens" assertion needs.
    pub async fn stayed_true<F>(&self, window: Duration, condition: F) -> bool
    where
        F: Fn(&Self) -> bool,
    {
        let deadline = tokio::time::Instant::now() + window;
        loop {
            if !condition(self) {
                return false;
            }
            if tokio::time::Instant::now() >= deadline {
                return true;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }
}

// ---------------------------------------------------------------------------
// Reading an intent, at the boundary
// ---------------------------------------------------------------------------

/// The entity identity an intent names, where it names one.
#[must_use]
pub fn entity_id_of(intent: &v1::Intent) -> Option<Vec<u8>> {
    match intent.action.as_ref()? {
        v1::intent::Action::Create(create) => match create.subject.as_ref()? {
            v1::create::Subject::Entity(entity) => Some(entity.entity_id.clone()),
            v1::create::Subject::Relation(_) => None,
        },
        v1::intent::Action::Edit(edit) => match edit.subject.as_ref()? {
            v1::edit::Subject::Entity(entity) => Some(entity.entity_id.clone()),
            v1::edit::Subject::Relation(_) => None,
        },
        _ => None,
    }
}

/// The entity creation an intent carries, where it creates an entity.
#[must_use]
pub fn create_entity_of(intent: &v1::Intent) -> Option<&v1::CreateEntity> {
    match intent.action.as_ref()? {
        v1::intent::Action::Create(create) => match create.subject.as_ref()? {
            v1::create::Subject::Entity(entity) => Some(entity),
            v1::create::Subject::Relation(_) => None,
        },
        _ => None,
    }
}

/// The content an intent carries, where it carries any.
#[must_use]
pub fn content_of(intent: &v1::Intent) -> Option<&v1::EntityContent> {
    match intent.action.as_ref()? {
        v1::intent::Action::Create(create) => match create.subject.as_ref()? {
            v1::create::Subject::Entity(entity) => entity.content.as_ref(),
            v1::create::Subject::Relation(_) => None,
        },
        v1::intent::Action::Edit(edit) => match edit.subject.as_ref()? {
            v1::edit::Subject::Entity(entity) => entity.content.as_ref(),
            v1::edit::Subject::Relation(_) => None,
        },
        _ => None,
    }
}

/// The title an intent carries, where it carries one.
#[must_use]
pub fn title_of(intent: &v1::Intent) -> Option<String> {
    content_of(intent).and_then(|content| content.title.clone())
}

/// The body identity a file intent names, where it names one.
#[must_use]
pub fn body_id_of(intent: &v1::Intent) -> Option<Vec<u8>> {
    match content_of(intent)?.specific.as_ref()? {
        v1::entity_content::Specific::File(file) => file.body_id.clone(),
        _ => None,
    }
}

/// Whether the intent creates an entity.
#[must_use]
pub fn is_create(intent: &v1::Intent) -> bool {
    matches!(
        intent.action.as_ref(),
        Some(v1::intent::Action::Create(v1::Create {
            subject: Some(v1::create::Subject::Entity(_))
        }))
    )
}

/// Whether the intent edits an entity.
#[must_use]
pub fn is_edit(intent: &v1::Intent) -> bool {
    matches!(
        intent.action.as_ref(),
        Some(v1::intent::Action::Edit(v1::Edit {
            subject: Some(v1::edit::Subject::Entity(_))
        }))
    )
}

// ---------------------------------------------------------------------------
// The service
// ---------------------------------------------------------------------------

struct Service {
    state: Arc<State>,
}

impl Service {
    /// Decide one intent, honouring the idempotence rule first: an intent the
    /// instance already holds gets its original acknowledgement back,
    /// unchanged, and is not decided a second time.
    fn decide(&self, intent: &v1::Intent, behaviour: &Behaviour) -> v1::Acknowledgement {
        let mut inner = self.state.inner.lock().unwrap();
        if let Some(held) = inner.issued.get(&intent.intent_id) {
            return held.clone();
        }
        inner.decided += 1;
        let nth = inner.decided;
        drop(inner);

        let refused = |reason: v1::RefusalReason, detail: String| v1::Acknowledgement {
            intent_id: intent.intent_id.clone(),
            outcome: Some(v1::acknowledgement::Outcome::Refused(v1::Refused {
                reason: reason as i32,
                detail,
            })),
        };
        let applied = |conflict: Option<v1::ConflictRetained>| v1::Acknowledgement {
            intent_id: intent.intent_id.clone(),
            outcome: Some(v1::acknowledgement::Outcome::Applied(v1::Applied {
                entity_ids: entity_id_of(intent).into_iter().collect(),
                relation_ids: Vec::new(),
                counter: 1,
                conflict,
            })),
        };

        let acknowledgement = match behaviour {
            Behaviour::RefuseNth(k) if nth == *k => {
                refused(v1::RefusalReason::NotAvailable, String::new())
            }
            Behaviour::RefuseTitled(titles)
                if title_of(intent).is_some_and(|title| titles.contains(&title)) =>
            {
                refused(v1::RefusalReason::NotAvailable, String::new())
            }
            Behaviour::RefuseNotAvailable { detail } => {
                refused(v1::RefusalReason::NotAvailable, detail.clone())
            }
            Behaviour::RecreateOnEdit if is_edit(intent) => v1::Acknowledgement {
                intent_id: intent.intent_id.clone(),
                outcome: Some(v1::acknowledgement::Outcome::Recreated(v1::Recreated {
                    original_entity_id: entity_id_of(intent).unwrap_or_default(),
                    new_entity_id: uuid::Uuid::new_v4().as_bytes().to_vec(),
                    counter: 1,
                })),
            },
            Behaviour::ApplyWithConflict if is_edit(intent) => {
                applied(Some(v1::ConflictRetained {
                    // Field 1 of EntityContent is the title, which is what the
                    // scenarios edit.
                    content_fields: vec![1],
                    specific_fields: Vec::new(),
                    block_ids: Vec::new(),
                }))
            }
            _ => applied(None),
        };

        let mut inner = self.state.inner.lock().unwrap();
        inner
            .issued
            .insert(intent.intent_id.clone(), acknowledgement.clone());
        match acknowledgement.outcome.as_ref() {
            Some(v1::acknowledgement::Outcome::Applied(applied)) => {
                for id in &applied.entity_ids {
                    inner.committed.insert(id.clone());
                }
            }
            Some(v1::acknowledgement::Outcome::Recreated(recreated)) => {
                inner.committed.insert(recreated.new_entity_id.clone());
            }
            _ => {}
        }
        acknowledgement
    }

    fn record(
        &self,
        seq: u64,
        token: Option<String>,
        intents: Vec<v1::Intent>,
        acknowledgements: Vec<v1::Acknowledgement>,
        answered: bool,
    ) {
        let mut inner = self.state.inner.lock().unwrap();
        inner.log.submissions.push(Submission {
            seq,
            token,
            intents,
            acknowledgements,
            answered,
        });
    }
}

fn token_of<T>(request: &Request<T>) -> Option<String> {
    request
        .metadata()
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .map(std::string::ToString::to_string)
}

#[tonic::async_trait]
impl v1::altair_server::Altair for Service {
    async fn submit(
        &self,
        request: Request<v1::SubmitRequest>,
    ) -> Result<Response<v1::SubmitResponse>, Status> {
        let seq = self.state.next_seq();
        let token = token_of(&request);
        let behaviour = self.state.behaviour();
        let intents = request.into_inner().intents;

        match behaviour {
            Behaviour::Unauthenticated => {
                self.record(seq, token, intents, Vec::new(), false);
                return Err(Status::unauthenticated(""));
            }
            Behaviour::UnrecognisedCondition => {
                self.record(seq, token, intents, Vec::new(), false);
                return Err(Status::unknown(""));
            }
            Behaviour::Stall => {
                self.record(seq, token, intents, Vec::new(), false);
                std::future::pending::<()>().await;
                unreachable!()
            }
            Behaviour::DropBeforeCommit => {
                self.record(seq, token, intents, Vec::new(), false);
                self.state.drop_connections();
                std::future::pending::<()>().await;
                unreachable!()
            }
            _ => {}
        }

        let acknowledgements: Vec<_> = intents
            .iter()
            .map(|intent| self.decide(intent, &behaviour))
            .collect();

        if behaviour == Behaviour::DropAfterCommit {
            self.record(seq, token, intents, acknowledgements, false);
            self.state.drop_connections();
            std::future::pending::<()>().await;
            unreachable!()
        }

        self.record(seq, token, intents, acknowledgements.clone(), true);
        Ok(Response::new(v1::SubmitResponse { acknowledgements }))
    }

    async fn put_body(
        &self,
        request: Request<Streaming<v1::BodyChunk>>,
    ) -> Result<Response<v1::PutBodyAck>, Status> {
        let behaviour = self.state.behaviour();
        if behaviour == Behaviour::Unauthenticated {
            return Err(Status::unauthenticated(""));
        }
        let mut stream = request.into_inner();
        let mut body_id = Vec::new();
        let mut bytes = Vec::new();
        while let Some(chunk) = stream.message().await? {
            if !chunk.body_id.is_empty() {
                body_id = chunk.body_id;
            }
            bytes.extend_from_slice(&chunk.data);
        }
        // The sequence is taken at completion, because C3 asks whether the
        // stream *completed* before the intent naming it arrived.
        let seq = self.state.next_seq();
        let received = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        let mut inner = self.state.inner.lock().unwrap();
        inner.log.bodies.push(BodyArrival {
            seq,
            body_id: body_id.clone(),
            bytes,
        });
        drop(inner);
        Ok(Response::new(v1::PutBodyAck {
            body_id,
            bytes_received: received,
        }))
    }

    type GetBodyStream =
        tokio_stream::wrappers::ReceiverStream<Result<v1::BodyChunk, tonic::Status>>;

    async fn get_body(
        &self,
        _request: Request<v1::BodyRequest>,
    ) -> Result<Response<Self::GetBodyStream>, Status> {
        Err(Status::unimplemented(
            "the fake instance answers writes only",
        ))
    }

    async fn query(
        &self,
        _request: Request<v1::QueryRequest>,
    ) -> Result<Response<v1::QueryResponse>, Status> {
        Err(Status::unimplemented(
            "the fake instance answers writes only",
        ))
    }

    async fn changes(
        &self,
        _request: Request<v1::ChangesRequest>,
    ) -> Result<Response<v1::ChangesResponse>, Status> {
        Err(Status::unimplemented(
            "the fake instance answers writes only",
        ))
    }

    async fn get_health(
        &self,
        _request: Request<v1::HealthRequest>,
    ) -> Result<Response<v1::HealthResponse>, Status> {
        Err(Status::unimplemented(
            "the fake instance answers writes only",
        ))
    }
}

// ---------------------------------------------------------------------------
// The proxy
// ---------------------------------------------------------------------------

async fn proxy(
    front: TcpListener,
    backend: SocketAddr,
    state: Arc<State>,
    generation: watch::Receiver<u64>,
) {
    loop {
        let Ok((mut inbound, _)) = front.accept().await else {
            return;
        };
        if state.behaviour() == Behaviour::Unreachable {
            // Closed the moment it opens: a transport failure, which is the
            // same class of thing as a refused connection and is what a client
            // must treat as a wait.
            drop(inbound);
            continue;
        }
        let mut watcher = generation.clone();
        let at_open = *watcher.borrow_and_update();
        tokio::spawn(async move {
            let Ok(mut outbound) = TcpStream::connect(backend).await else {
                return;
            };
            tokio::select! {
                _ = tokio::io::copy_bidirectional(&mut inbound, &mut outbound) => {}
                () = generation_changed(&mut watcher, at_open) => {}
            }
        });
    }
}

async fn generation_changed(watcher: &mut watch::Receiver<u64>, from: u64) {
    loop {
        if *watcher.borrow_and_update() != from {
            return;
        }
        if watcher.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }
}
