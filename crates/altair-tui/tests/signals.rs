//! What the client says, and the far longer list of what it does not.

use altair_tui::signals::{Event, Kind, Signals};

#[test]
fn nothing_is_shown_by_default() {
    let signals = Signals::new();
    assert!(
        signals.showing().is_empty(),
        "waiting is silent, and a client with nothing wrong is waiting"
    );
}

#[test]
fn a_backlog_is_never_reported() {
    let signals = Signals::new();
    // There is no call here that could report one. That is the point of this
    // test: forty things waiting and forty things waiting for three weeks are
    // the same, because neither is a condition and neither is counted.
    assert!(signals.showing().is_empty());
    assert!(signals.drain_events().is_empty());
}

#[test]
fn the_one_number_is_how_many_the_instance_refused() {
    let signals = Signals::new();
    signals.refusals(3);
    let showing = signals.showing();
    assert_eq!(showing.len(), 1);
    assert_eq!(showing[0].kind, Kind::Fault);
    assert_eq!(showing[0].quantity, Some(3));
    assert!(
        showing[0].text.contains('3'),
        "a person reads the statement, not the field beside it"
    );
}

#[test]
fn a_standing_fault_does_not_escalate_repeat_or_age() {
    let signals = Signals::new();
    signals.unrecognised(true);
    let first = signals.showing();
    assert_eq!(first.len(), 1);
    let _ = signals.drain_events();

    // The condition persists. Every pass the sender makes says so again.
    for _ in 0..100 {
        signals.unrecognised(true);
    }

    assert_eq!(
        signals.showing(),
        first,
        "the person finds one current statement, unchanged: nothing says how \
         long it has been true"
    );
    assert!(
        signals.drain_events().is_empty(),
        "and nothing repeated in the interval"
    );
}

#[test]
fn a_signal_goes_when_its_condition_does() {
    let signals = Signals::new();
    signals.unrecognised(true);
    let raised = signals.drain_events();
    assert!(matches!(raised.as_slice(), [Event::Raised(_)]));

    signals.unrecognised(false);
    assert!(
        signals.showing().is_empty(),
        "with nothing for the person to acknowledge or dismiss"
    );
    assert!(matches!(
        signals.drain_events().as_slice(),
        [Event::Cleared(_)]
    ));
}

#[test]
fn two_refusals_read_the_same_whatever_the_instance_said_about_them() {
    // A refusal covers both an entity that does not exist and one this member
    // cannot see, and nothing anywhere distinguishes them — including this.
    // The instance's detail is free text for a log and never reaches a person.
    let absent = Signals::new();
    absent.refusals(1);
    let invisible = Signals::new();
    invisible.refusals(1);
    assert_eq!(absent.showing(), invisible.showing());
}
