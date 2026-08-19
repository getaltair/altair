//! Turning what a person captured into what crosses the wire, and reading what
//! comes back.
//!
//! Nothing here touches the store and nothing here does i/o. It is the one
//! place that knows the contract's shapes, so that the store holds opaque
//! messages and the sender holds a policy.

use std::time::SystemTime;

use altair_proto::prost_types::Timestamp;
use altair_proto::v1;

use crate::capture::Captured;

/// How this entity came to exist. The contract keeps this an open set of
/// strings precisely so a client can add a way of capturing without a schema
/// change; this is the terminal client's ordinary one.
pub const CAPTURE_METHOD: &str = "terminal";

/// A capture, composed for the wire.
pub struct Composed {
    pub entity_id: Vec<u8>,
    pub content: v1::EntityContent,
    /// The body and its bytes, where a file was captured. Both are durable
    /// before acceptance is shown, and the bytes reach the instance before the
    /// record naming them.
    pub body: Option<(Vec<u8>, Vec<u8>)>,
    pub intent: v1::Intent,
}

/// Compose a capture: a fresh entity identity, a fresh intent identity, and
/// the content the person gave.
#[must_use]
pub fn compose_creation(captured: &Captured) -> Composed {
    let entity_id = fresh_id();
    let mut body = None;
    let specific = match captured {
        // The floor: a type and nothing else. No title is invented, because
        // no field is required at any point on this path.
        Captured::Bare => v1::entity_content::Specific::Note(v1::NoteContent::default()),
        Captured::Note { body: text, .. } => v1::entity_content::Specific::Note(v1::NoteContent {
            body: (!text.is_empty()).then(|| text.clone()),
            cleared: Vec::new(),
        }),
        Captured::File { media_type, bytes } => {
            let body_id = fresh_id();
            body = Some((body_id.clone(), bytes.clone()));
            v1::entity_content::Specific::File(v1::FileContent {
                body_id: Some(body_id),
                media_type: Some(media_type.clone()),
                extracted_text: None,
                cleared: Vec::new(),
            })
        }
    };
    let content = v1::EntityContent {
        title: match captured {
            Captured::Note { title, .. } => Some(title.clone()),
            Captured::Bare | Captured::File { .. } => None,
        },
        specific: Some(specific),
        ..Default::default()
    };
    let intent = v1::Intent {
        intent_id: fresh_id(),
        action: Some(v1::intent::Action::Create(v1::Create {
            subject: Some(v1::create::Subject::Entity(v1::CreateEntity {
                entity_id: entity_id.clone(),
                created_at: Some(now()),
                capture_method: CAPTURE_METHOD.to_string(),
                content: Some(content.clone()),
            })),
        })),
    };
    Composed {
        entity_id,
        content,
        body,
        intent,
    }
}

/// Compose an edit to an entity's title.
///
/// The base counter is left at zero here and filled in at the moment of
/// sending. An edit composed while the instance was unreachable knows nothing
/// about what the instance has since acknowledged, and sending it as composed
/// is what makes a person conflict with themselves over a sequence they
/// performed in a known order.
#[must_use]
pub fn compose_title_edit(entity_id: &[u8], title: &str) -> (v1::EntityContent, v1::Intent) {
    let content = v1::EntityContent {
        title: Some(title.to_string()),
        ..Default::default()
    };
    let intent = v1::Intent {
        intent_id: fresh_id(),
        action: Some(v1::intent::Action::Edit(v1::Edit {
            subject: Some(v1::edit::Subject::Entity(v1::EditEntity {
                entity_id: entity_id.to_vec(),
                base_counter: 0,
                content: Some(content.clone()),
            })),
        })),
    };
    (content, intent)
}

/// Set the counter an edit declares it was based on. Does nothing to anything
/// that is not an edit of an entity.
pub fn set_base_counter(intent: &mut v1::Intent, counter: u64) {
    if let Some(v1::intent::Action::Edit(v1::Edit {
        subject: Some(v1::edit::Subject::Entity(entity)),
    })) = intent.action.as_mut()
    {
        entity.base_counter = counter;
    }
}

/// Point an intent at a different entity identity, after a recreation.
pub fn rename_subject(intent: &mut v1::Intent, entity_id: &[u8]) {
    match intent.action.as_mut() {
        Some(v1::intent::Action::Create(v1::Create {
            subject: Some(v1::create::Subject::Entity(entity)),
        })) => entity.entity_id = entity_id.to_vec(),
        Some(v1::intent::Action::Edit(v1::Edit {
            subject: Some(v1::edit::Subject::Entity(entity)),
        })) => entity.entity_id = entity_id.to_vec(),
        _ => {}
    }
}

/// The body an intent names, where it names one. The sender uses this to get
/// the bytes there first.
#[must_use]
pub fn body_id_of(intent: &v1::Intent) -> Option<Vec<u8>> {
    let content = match intent.action.as_ref()? {
        v1::intent::Action::Create(v1::Create {
            subject: Some(v1::create::Subject::Entity(entity)),
        }) => entity.content.as_ref()?,
        v1::intent::Action::Edit(v1::Edit {
            subject: Some(v1::edit::Subject::Entity(entity)),
        }) => entity.content.as_ref()?,
        _ => return None,
    };
    match content.specific.as_ref()? {
        v1::entity_content::Specific::File(file) => file.body_id.clone(),
        _ => None,
    }
}

/// What a condition the instance answered with means for the person.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    /// The ordinary path clears this by continuing to run. Nothing is shown:
    /// an unreachable instance and an expired session are the same wait.
    Wait,
    /// This will not clear that way, so it is put in front of the person. A
    /// condition not *known* to be self clearing lands here, because a wrong
    /// signal costs attention and a wrong silence costs data.
    Fault,
}

/// Classify what came back from a call.
///
/// **The discriminator is `source`, and it was measured rather than assumed.**
/// A status tonic synthesised from a transport failure carries the underlying
/// error as its source; a status the instance actually sent carries none. Both
/// arrive as code `Unknown` when the instance tears a connection down mid-call
/// and when it answers with something unrecognised, so the code alone cannot
/// tell a wait from a fault. `tests/wire_conditions.rs` pins this against the
/// fake instance, and it is the test to read first if silence and signalling
/// ever go strange.
#[must_use]
pub fn classify(status: &tonic::Status) -> Condition {
    if std::error::Error::source(status).is_some() {
        return Condition::Wait;
    }
    match status.code() {
        // DR-005: the client refreshes without a person, so an expired session
        // is a wait and never a login page.
        tonic::Code::Unauthenticated | tonic::Code::Unavailable => Condition::Wait,
        _ => Condition::Fault,
    }
}

/// A fresh identifier. Every identity in this system is a random UUIDv4
/// generated by whatever brings the thing into existence, and nothing ever
/// reads one (DR-007).
#[must_use]
pub fn fresh_id() -> Vec<u8> {
    uuid::Uuid::new_v4().as_bytes().to_vec()
}

fn now() -> Timestamp {
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    Timestamp {
        seconds: i64::try_from(since_epoch.as_secs()).unwrap_or(i64::MAX),
        nanos: i32::try_from(since_epoch.subsec_nanos()).unwrap_or(0),
    }
}
