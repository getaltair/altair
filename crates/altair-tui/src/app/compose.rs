//! Turning what the store holds into what a screen is handed.
//!
//! **One direction only.** A view is built from the store and never asks it
//! anything mid-draw, which is what keeps every screen a pure function of its
//! view and checkable as a snapshot.

use altair_proto::v1;
use chrono::{DateTime, Datelike, Local, TimeZone};

use crate::device::Held;
use crate::ui::glyphs::State;
use crate::ui::view::{Group, Kept, Link, Row, Rung};

/// What the person calls this type.
///
/// **The person's words, not the wire's.** The tag of `content.specific` is the
/// type, and this is the only place that mapping is written down.
#[must_use]
pub fn kind_of(content: &v1::EntityContent) -> &'static str {
    match content.specific.as_ref() {
        Some(v1::entity_content::Specific::Campaign(_)) => "campaign",
        Some(v1::entity_content::Specific::Arc(_)) => "arc",
        Some(v1::entity_content::Specific::Quest(_)) => "quest",
        Some(v1::entity_content::Specific::Routine(_)) => "routine",
        Some(v1::entity_content::Specific::FocusSession(_)) => "focus",
        Some(v1::entity_content::Specific::CheckIn(_)) => "check-in",
        Some(v1::entity_content::Specific::Note(_)) => "note",
        Some(v1::entity_content::Specific::File(_)) => "file",
        Some(v1::entity_content::Specific::Item(_)) => "item",
        Some(v1::entity_content::Specific::Location(_)) => "location",
        Some(v1::entity_content::Specific::ShoppingList(_)) => "list",
        Some(v1::entity_content::Specific::Category(_)) => "category",
        // The type is the tag, so an entity without one has no type. It is
        // still the person's and still listed.
        None => "",
    }
}

/// What a person calls something they never titled.
///
/// **Not "(untitled)".** No field is required at capture, so an entity with no
/// title is ordinary rather than deficient, and the first line of what they
/// wrote is what they would recognise.
#[must_use]
pub fn what_of(held: &Held) -> String {
    if let Some(title) = held.content.title.as_deref()
        && !title.trim().is_empty()
    {
        return title.to_string();
    }
    if let Some(body) = held.body()
        && let Some(first) = body.lines().find(|line| !line.trim().is_empty())
    {
        return first.trim().to_string();
    }
    String::new()
}

/// The captures list, grouped by day.
///
/// **Nothing here counts anything.** The groups are days, the rows are what
/// moved, and there is no number anywhere saying how much of it has not
/// reached the instance.
#[must_use]
pub fn captures(held: &[Held], selected: usize) -> Vec<Group> {
    let today = Local::now().date_naive();
    let mut groups: Vec<Group> = Vec::new();
    for (index, thing) in held.iter().enumerate() {
        let when = thing.at.and_then(|at| Local.timestamp_opt(at, 0).single());
        let label = match when {
            Some(at) if at.date_naive() == today => "what you touched".to_string(),
            Some(at) => day_label(at),
            // Something nothing knows the age of. Said plainly rather than
            // guessed at.
            None => "whenever".to_string(),
        };
        let quiet = label != "what you touched";
        if groups.last().map(|group| group.label.as_str()) != Some(label.as_str()) {
            groups.push(Group {
                label,
                rows: Vec::new(),
                quiet,
            });
        }
        let group = groups.last_mut().expect("just pushed");
        group.rows.push(Row {
            when: when
                .map(|at| at.format("%H:%M").to_string())
                .unwrap_or_default(),
            what: what_of(thing),
            kind: kind_of(&thing.content).to_string(),
            // **Nothing here says whether it has been sent.** It looked like a
            // fact about one row rather than a count, and it is not: forty rows
            // each saying "not sent" is an indication of how many, which is the
            // thing that may never reach a person. If it was accepted it is
            // theirs, and that is the whole of what they are owed.
            note: String::new(),
            selected: index == selected,
            past: quiet,
        });
    }
    groups
}

fn day_label(at: DateTime<Local>) -> String {
    format!(
        "{} {}",
        at.format("%A").to_string().to_lowercase(),
        at.day()
    )
}

/// The guidance ladder, walked from the roots down.
#[must_use]
pub fn ladder(rows: &[(usize, Held)], selected: usize) -> Vec<Rung> {
    rows.iter()
        .enumerate()
        .map(|(index, (depth, held))| Rung {
            depth: *depth,
            state: state_of(&held.content),
            what: what_of(held),
            kind: kind_of(&held.content).to_string(),
            // Silent for the same reason the captures list is.
            note: String::new(),
            selected: index == selected,
            blocked: false,
        })
        .collect()
}

/// Which of the three states a piece of guidance is in.
///
/// One upward movement, and anything that names no state is waiting — which is
/// where everything starts.
#[must_use]
pub fn state_of(content: &v1::EntityContent) -> State {
    let state = match content.specific.as_ref() {
        Some(v1::entity_content::Specific::Campaign(campaign)) => campaign.state,
        Some(v1::entity_content::Specific::Arc(arc)) => arc.state,
        Some(v1::entity_content::Specific::Quest(quest)) => quest.state,
        _ => None,
    };
    match state.and_then(|value| v1::GuidanceState::try_from(value).ok()) {
        Some(v1::GuidanceState::Working) => State::Working,
        Some(v1::GuidanceState::Worked) => State::Worked,
        _ => State::Waiting,
    }
}

/// The location tree, and what is kept in it.
#[must_use]
pub fn tracking(rows: &[(usize, Held)], selected: usize) -> Vec<Kept> {
    rows.iter()
        .enumerate()
        .map(|(index, (depth, held))| Kept {
            depth: *depth,
            what: what_of(held),
            amount: amount_of(&held.content),
            committed: String::new(),
            selected: index == selected,
        })
        .collect()
}

/// What the person last said is there, in their own units.
///
/// Never computed from history, and never adjusted by anything except an
/// explicit act.
#[must_use]
pub fn amount_of(content: &v1::EntityContent) -> String {
    let Some(v1::entity_content::Specific::Item(item)) = content.specific.as_ref() else {
        return String::new();
    };
    let Some(amount) = item.asserted_amount.as_ref() else {
        return String::new();
    };
    let unit = item.unit.clone().unwrap_or_default();
    let whole = amount.units;
    if amount.nanos == 0 {
        format!("{whole} {unit}").trim().to_string()
    } else {
        // Exact, because a household's stock is not a floating point number.
        let fraction = format!("{:09}", amount.nanos.abs());
        let fraction = fraction.trim_end_matches('0');
        format!("{whole}.{fraction} {unit}").trim().to_string()
    }
}

/// The relations touching an entity, read from where the person is standing.
#[must_use]
pub fn links(here: &[u8], relations: &[v1::Relation], name: impl Fn(&[u8]) -> String) -> Vec<Link> {
    relations
        .iter()
        .filter_map(|relation| {
            let content = relation.content.as_ref()?;
            let from = content.from_entity_id.clone().unwrap_or_default();
            let to = content.to_entity_id.clone().unwrap_or_default();
            let outward = from == here;
            let other = if outward { to } else { from };
            Some(Link {
                // Empty means untyped, which is the common case. An untyped
                // relation is untyped, not possibly typed.
                kind: String::new(),
                outward,
                what: name(&other),
                anchor: content
                    .anchor
                    .as_ref()
                    .filter(|anchor| !anchor.phrase.is_empty())
                    .map(|anchor| format!("at “{}”", anchor.phrase))
                    .unwrap_or_default(),
            })
        })
        .collect()
}
