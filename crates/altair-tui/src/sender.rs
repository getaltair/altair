//! Getting what the person captured to the instance.
//!
//! **This runs behind acceptance and never in front of it.** Nothing here is
//! on the path between a person capturing something and being told it is
//! theirs; stopping it for a week changes nothing they were promised. That
//! separation is the whole reason the outbox exists, and it is why this module
//! holds its own handle on the store rather than sharing the surface's.
//!
//! # The four rules that are easy to lose
//!
//! **One intent per entity in flight.** Ordering is per entity rather than
//! global, and taking at most one outstanding intent per entity per pass is
//! what makes a create reach the instance before its own edits without any
//! explicit sequencing anywhere.
//!
//! **An edit is rebased before it is sent.** Two edits composed while the
//! instance was unreachable were both composed against the counter the client
//! last knew. Sent as composed, the second arrives stale against the first,
//! touches the same part, and makes the person conflict with themselves over a
//! sequence they performed in a known order on one device. The client is the
//! only party that knows that order, so carrying it is its obligation.
//!
//! **Bytes before the record.** A body stream completes before the intent
//! naming it is submitted.
//!
//! **A refusal is retained and not retried.** It will not clear by continuing
//! to run, so retrying is pointless, and dropping it is forbidden. It stays,
//! and everything queued behind it for the same entity waits.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use altair_proto::v1;
use tonic::transport::Channel;

use crate::device::{Pending, Store};
use crate::signals::Signals;
use crate::wire::{self, Condition};

/// How long to wait before trying again after a pass that could not finish.
/// Short enough that an instance coming back is noticed promptly, and it is
/// never reported: nothing about this interval reaches a person.
const RETRY: Duration = Duration::from_millis(100);

/// How many intents go in one submission. The instance answers each on its own
/// terms, so a refusal in the middle leaves everything around it applied.
const BATCH: usize = 50;

/// Bytes per body chunk.
const CHUNK: usize = 64 * 1024;

/// A call that has not answered in this long is treated as an unanswered call,
/// which is a wait. A stalled send must not gate anything, and it does not:
/// the person's surface is not behind this.
const PATIENCE: Duration = Duration::from_secs(30);

/// What one pass did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pass {
    /// Something reached the instance and was answered. Go again immediately;
    /// there may be a next intent for the same entity now unblocked.
    Sent,
    /// Nothing to send.
    Idle,
    /// The instance could not be reached, or would not answer. Wait.
    Held,
}

pub struct Sender {
    store: Store,
    client: v1::altair_client::AltairClient<Channel>,
    token: String,
    signals: Arc<Signals>,
}

impl Sender {
    /// Point a sender at an instance. Nothing connects here: the channel is
    /// lazy, so an instance that is not there costs nothing until there is
    /// something to send.
    ///
    /// # Errors
    ///
    /// Fails when the store cannot be opened or the instance's address cannot
    /// be read as one.
    pub fn new(
        state_dir: &Path,
        instance_url: &str,
        token: String,
        signals: Arc<Signals>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let store = Store::reopen(state_dir)?;
        let channel =
            tonic::transport::Endpoint::from_shared(instance_url.to_string())?.connect_lazy();
        Ok(Self {
            store,
            client: v1::altair_client::AltairClient::new(channel),
            token,
            signals,
        })
    }

    /// Run until the process ends. There is no other way out: an outbox that
    /// gave up would be an outbox that dropped something.
    pub async fn run(mut self) {
        // What is already held decides what is shown, before anything is
        // tried. A refusal from before a restart is still a refusal.
        self.publish_refusals();
        loop {
            match self.pass().await {
                Pass::Sent => {}
                Pass::Idle | Pass::Held => tokio::time::sleep(RETRY).await,
            }
        }
    }

    async fn pass(&mut self) -> Pass {
        let Ok(batch) = self.store.next_to_send(BATCH) else {
            return Pass::Held;
        };
        if batch.is_empty() {
            return Pass::Idle;
        }

        // Bytes before the record: every body these intents name completes
        // before any of them is submitted.
        for pending in &batch {
            if let Some(body_id) = wire::body_id_of(&pending.intent) {
                match self.send_body(&body_id).await {
                    Ok(()) => {}
                    Err(condition) => return self.hold(condition),
                }
            }
        }

        let intents: Vec<v1::Intent> = batch
            .iter()
            .map(|pending| {
                let mut intent = pending.intent.clone();
                wire::set_base_counter(&mut intent, pending.acknowledged_counter);
                intent
            })
            .collect();

        let request = self.authorised(v1::SubmitRequest { intents });
        let call = self.client.submit(request);
        let answer = match tokio::time::timeout(PATIENCE, call).await {
            // A call that never answered is an unanswered call, which the
            // ordinary path clears by running again.
            Err(_elapsed) => return self.hold(Condition::Wait),
            Ok(Err(status)) => return self.hold(wire::classify(&status)),
            Ok(Ok(response)) => response.into_inner(),
        };

        self.signals.unrecognised(false);
        for pending in &batch {
            let Some(acknowledgement) = answer
                .acknowledgements
                .iter()
                .find(|ack| ack.intent_id == pending.intent_id)
            else {
                // Unanswered, so still outstanding. Submitted again next pass,
                // under the identity it already has.
                continue;
            };
            self.settle(pending, acknowledgement);
        }
        self.publish_refusals();
        Pass::Sent
    }

    /// Record what the instance said about one intent.
    fn settle(&mut self, pending: &Pending, acknowledgement: &v1::Acknowledgement) {
        let result = match acknowledgement.outcome.as_ref() {
            // A retained conflict arrives inside an application and is not a
            // failure: both values were kept and the write applied. The intent
            // is done, nothing is retried, and nothing is signalled.
            Some(v1::acknowledgement::Outcome::Applied(applied)) => {
                self.store
                    .applied(&pending.intent_id, &pending.entity_id, applied.counter)
            }
            Some(v1::acknowledgement::Outcome::Refused(_)) => {
                self.store.refused(&pending.intent_id, &pending.entity_id)
            }
            // The household erased the entity and the person's work came back
            // under a new identity. Nothing was lost, so this is carried
            // silently rather than signalled.
            Some(v1::acknowledgement::Outcome::Recreated(recreated)) => self.store.recreated(
                &pending.intent_id,
                &pending.entity_id,
                &recreated.new_entity_id,
                recreated.counter,
            ),
            // An acknowledgement naming no outcome says nothing about what
            // happened, so nothing is concluded and the intent stays.
            None => Ok(()),
        };
        // A store that cannot be written to leaves the intent outstanding,
        // which is the safe direction: it is resubmitted under the identity it
        // already has, and the instance returns what it already issued.
        let _ = result;
    }

    async fn send_body(&mut self, body_id: &[u8]) -> Result<(), Condition> {
        let Ok(Some(bytes)) = self.store.unsent_body(body_id) else {
            return Ok(());
        };
        let mut chunks: Vec<v1::BodyChunk> = bytes
            .chunks(CHUNK)
            .map(|data| v1::BodyChunk {
                body_id: body_id.to_vec(),
                data: data.to_vec(),
            })
            .collect();
        if chunks.is_empty() {
            chunks.push(v1::BodyChunk {
                body_id: body_id.to_vec(),
                data: Vec::new(),
            });
        }
        let request = self.authorised(tokio_stream::iter(chunks));
        match tokio::time::timeout(PATIENCE, self.client.put_body(request)).await {
            Err(_elapsed) => Err(Condition::Wait),
            Ok(Err(status)) => Err(wire::classify(&status)),
            Ok(Ok(_)) => {
                self.signals.unrecognised(false);
                // Re uploading the same body identity is idempotent, so losing
                // this mark costs a repeat and never a wrong record.
                let _ = self.store.body_sent(body_id);
                Ok(())
            }
        }
    }

    /// Nothing got through. Say so where saying so is required, and nowhere
    /// else.
    fn hold(&self, condition: Condition) -> Pass {
        self.signals.unrecognised(condition == Condition::Fault);
        Pass::Held
    }

    fn publish_refusals(&self) {
        if let Ok(count) = self.store.refused_count() {
            self.signals.refusals(count);
        }
    }

    fn authorised<T>(&self, message: T) -> tonic::Request<T> {
        let mut request = tonic::Request::new(message);
        if let Ok(value) = self.token.parse() {
            request.metadata_mut().insert("authorization", value);
        }
        request
    }
}
