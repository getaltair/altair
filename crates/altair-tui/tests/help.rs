//! The help surface, read the way an assembled document is read.
//!
//! It decides nothing. Every line restates something the substrate already
//! requires, so what can be checked here is the shapes that would make it a
//! lie — a promise the client does not keep, or a number it is not allowed to
//! state.

use altair_tui::ui::help::{KEYS, PROMISES, prose};

#[test]
fn nothing_here_states_a_number() {
    // The client states exactly one number anywhere, and it is how many the
    // instance refused. That is a live count on a fault, not a sentence in the
    // help, so nothing here has any business carrying a digit.
    for line in prose() {
        assert!(
            !line.chars().any(|c| c.is_ascii_digit()),
            "the help states a number: {line:?}. The only number this client \
             may state is how many the instance refused, and that is a signal \
             rather than a promise."
        );
    }
}

#[test]
fn it_says_that_waiting_is_silent_and_gives_the_reason() {
    let said = prose().join(" ").to_lowercase();
    assert!(
        said.contains("session"),
        "a person whose session ran out sees nothing at all, so the help owes \
         them the reason they are seeing nothing"
    );
    assert!(
        said.contains("clears itself") || said.contains("clear itself"),
        "the distinction the whole silence rests on is that a wait clears \
         itself and a fault does not; the help must actually say it"
    );
}

#[test]
fn it_promises_acceptance_before_the_network() {
    let said = prose().join(" ").to_lowercase();
    assert!(
        said.contains("before you are told"),
        "acceptance is local and durable before the person is told, and that \
         is the client's first promise"
    );
    assert!(
        said.contains("nothing is dropped"),
        "a refusal is retained rather than discarded, and a person who is told \
         their instance refused something needs to know their work is still there"
    );
}

#[test]
fn it_does_not_promise_a_buffer_this_client_no_longer_holds() {
    // The design was drawn when an in-client editor was assumed, and promised
    // the buffer was kept as the person typed. Composition is a handoff now.
    // What is true is that the client owns the file the editor writes into.
    let said = prose().join(" ").to_lowercase();
    assert!(
        !said.contains("as you type"),
        "that promise belonged to an editor this client does not have"
    );
    assert!(
        said.contains("still on disk"),
        "what is true now is that the client owns the file and still has it \
         after a crash, and the help should say the true thing"
    );
}

#[test]
fn no_key_is_one_this_client_can_be_sure_of_being_given() {
    for group in KEYS {
        for (cap, says) in group.keys {
            assert!(
                !cap.contains('⌘'),
                "{cap:?} is a Mac key, in a client that has to run on Linux and \
                 on Windows"
            );
            assert!(
                !says.is_empty(),
                "{cap:?} has no plain-language label, and a keycap without one \
                 is a hint string"
            );
            assert!(
                !says.split_whitespace().all(|word| word.len() == 1),
                "{says:?} reads as a motion rather than as what happens"
            );
        }
    }
}

#[test]
fn every_promise_says_what_it_means() {
    for promise in PROMISES {
        assert!(
            promise.means.len() > promise.says.len(),
            "{:?} is stated and not explained, and an unexplained promise is \
             the kind that quietly stops being true",
            promise.says
        );
    }
}
