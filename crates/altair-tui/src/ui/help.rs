//! The help surface: the only place this client's promises are written in the
//! person's own words.
//!
//! **Treated the way an assembled document is treated.** It decides nothing.
//! Every line restates something the substrate already requires, and where it
//! and the substrate disagree, this is what gets corrected. `tests/help.rs`
//! checks the shapes that would make it a lie.
//!
//! # The one it would be easiest to get wrong
//!
//! The design was drawn when an in-client editor was assumed, and it promised
//! the buffer was kept as the person typed. That promise is no longer this
//! client's to make: composition happens in the person's own editor, and what
//! is true now is that **the client owns the file the editor writes into** and
//! still has it after a crash. Saying the older thing would be describing
//! software that does not exist.

/// One thing the client promises, and what it means in practice.
pub struct Promise {
    pub says: &'static str,
    pub means: &'static str,
}

/// One group of keys, as the help lists them.
pub struct Keys {
    pub about: &'static str,
    pub keys: &'static [(&'static str, &'static str)],
}

/// What the client promises, in the person's words.
///
/// Ordered by what a person would want to know first when something looks
/// wrong, which is nearly always "where did my thing go".
pub const PROMISES: &[Promise] = &[
    Promise {
        says: "What you capture is yours the moment you capture it.",
        means: "It is written to this device and confirmed before you are told. \
                Nothing about the network or your session is involved, and \
                turning the machine off straight afterwards does not lose it.",
    },
    Promise {
        says: "Nothing is ever waiting on you to press send.",
        means: "Your captures go to your instance on their own, whenever it is \
                reachable. There is nothing to flush and nothing to retry by \
                hand.",
    },
    Promise {
        says: "Quiet means fine.",
        means: "An instance you cannot reach and a session that has run out are \
                the same thing: a wait that clears itself. Neither is shown, \
                and neither will ever ask you to sign in again mid-sentence.",
    },
    Promise {
        says: "Anything you are told about is something that will not fix itself.",
        means: "If your instance refuses something, you are told, and it stays \
                exactly where it is until you deal with it. Nothing is dropped \
                and nothing is quietly discarded.",
    },
    Promise {
        says: "You write in your own editor, and the file is ours to keep.",
        means: "Composing opens $EDITOR on a file this client owns. If anything \
                dies while you are in there — the editor, this client, the \
                machine — the file is still on disk and is offered back to you \
                next time.",
    },
    Promise {
        says: "Two people editing the same thing keeps both.",
        means: "Nothing is overwritten and nothing is handed back to you to \
                redo. Where two edits touched the same part, both values are \
                kept and shown, and the thing stays readable throughout.",
    },
];

/// What the keys do, grouped the way a person would look for them.
pub const KEYS: &[Keys] = &[
    Keys {
        about: "getting around",
        keys: &[
            ("↑ ↓", "move"),
            ("← →", "fold and unfold"),
            ("↵", "open where it lives"),
            ("esc", "back out"),
        ],
    },
    Keys {
        about: "making something",
        keys: &[
            ("c", "capture"),
            ("o", "new quest here"),
            ("e", "edit in your editor"),
            ("space", "next state"),
        ],
    },
    Keys {
        about: "finding something",
        keys: &[("/", "search"), ("g", "ladder"), (":", "command")],
    },
    Keys {
        about: "everything else",
        keys: &[
            ("?", "this"),
            // Bound to a key that exists off a Mac. The design drew ⌘s, which
            // is not a key this client can expect to be given on Linux or on
            // Windows.
            ("ctrl+s", "keep what you are writing"),
            ("q", "leave"),
        ],
    },
];

/// Every line of prose the help puts in front of a person, for the checks that
/// read it as a whole.
#[must_use]
pub fn prose() -> Vec<&'static str> {
    let mut out = Vec::new();
    for promise in PROMISES {
        out.push(promise.says);
        out.push(promise.means);
    }
    for group in KEYS {
        out.push(group.about);
        for (_, says) in group.keys {
            out.push(says);
        }
    }
    out
}
