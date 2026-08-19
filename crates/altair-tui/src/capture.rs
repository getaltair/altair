//! What a person captured, in the client's own words.
//!
//! Deliberately not the wire's types and deliberately not the conformance
//! harness's: [`wire`](crate::wire) turns one of these into an `Intent`, and
//! the adapter turns the harness's vocabulary into one of these. Both
//! translations are somebody's job, and neither is this type's.

/// One thing a person captured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Captured {
    /// An identity, a type, and a timestamp, and nothing else. The floor the
    /// substrate puts under every client: no field is required on this path.
    Bare,
    /// A note. The body is markdown as the person typed it; the client never
    /// divides it and puts no relation markers in it.
    Note { title: String, body: String },
    /// A file. The bytes are durable locally before acceptance is shown, and
    /// they reach the instance before the record naming them.
    File { media_type: String, bytes: Vec<u8> },
}
