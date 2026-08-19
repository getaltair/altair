//! The running client.
//!
//! # What this loop is and is not
//!
//! **It draws and it takes keys.** Every rule lives somewhere else: acceptance
//! is [`crate::device`]'s, getting things to the instance is
//! [`crate::sender`]'s, keeping a copy is [`crate::replica`]'s, and what may be
//! said to a person is [`crate::signals`]'s. A loop that started deciding any
//! of those would be the place the rules quietly diverge from the ones the
//! conformance suite checks — and the suite drives the adapter, not this.
//!
//! **No habit of this client becomes the definition of a client.** What crosses
//! the boundary is the component model's list. A key binding, a screen order,
//! a fold state: none of that is anybody else's business.

pub mod compose;

use std::io;
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Frame as Surface};

use crate::capture::Captured;
use crate::config::Config;
use crate::device::{Held, Store};
use crate::editor::Editor;
use crate::signals::Signals;
use crate::ui::screens::{browse, sideways, write};
use crate::ui::view::Row;
use crate::ui::{Frame, Glyphs, Modal, Shell, Theme};
use crate::wire;

/// How long to wait for a key before drawing again.
///
/// **This is why a row arrives without the person asking.** The replica is
/// writing into the store on its own; redrawing on a cadence is how what it
/// wrote reaches the screen. It is never reported and there is nothing anywhere
/// that says how long ago anything happened.
const TICK: Duration = Duration::from_millis(250);

/// Which screen the person is on.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Where {
    Captures,
    Ladder,
    Tracking,
    Detail(Vec<u8>),
    Capturing,
    Faults,
    Help,
}

pub struct App {
    shell: Shell,
    store: Store,
    editor: Editor,
    signals: Arc<Signals>,
    at: Where,
    /// Where the person came from, so backing out goes somewhere sensible.
    came_from: Vec<Where>,
    cursor: usize,
    typed: String,
    leaving: bool,
}

impl App {
    /// # Errors
    ///
    /// Fails when the device store cannot be opened, which is the one local
    /// condition that reaches the guarantee rather than delaying it.
    pub fn new(config: &Config, signals: Arc<Signals>) -> rusqlite::Result<Self> {
        std::fs::create_dir_all(&config.state_dir).ok();
        Ok(Self {
            shell: Shell::new(Theme::of(config.mode), Glyphs::of(config.glyphs)),
            store: Store::open(&config.state_dir)?,
            editor: Editor::new(&config.state_dir),
            signals,
            at: Where::Captures,
            came_from: Vec::new(),
            cursor: 0,
            typed: String::new(),
            leaving: false,
        })
    }

    /// Draw, take a key, repeat, until the person leaves.
    ///
    /// # Errors
    ///
    /// Fails on a terminal that cannot be drawn to or read from.
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.leaving {
            terminal.draw(|surface| self.draw(surface))?;
            if event::poll(TICK)?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                self.key(key);
            }
        }
        Ok(())
    }

    fn rows(&self) -> Vec<Held> {
        self.store.captures().unwrap_or_default()
    }

    /// Everything the instance would not take.
    ///
    /// **Refused, not unsent.** An earlier version of this filtered on "has
    /// something outstanding", which put the whole backlog on the faults screen
    /// under a heading counting it — the exact number this client is never
    /// allowed to state. Waiting is silent; only a refusal is a fault.
    fn refused(&self) -> Vec<Row> {
        self.store
            .refused_entities()
            .unwrap_or_default()
            .iter()
            .map(|held| Row {
                what: compose::what_of(held),
                kind: compose::kind_of(&held.content).to_string(),
                ..Row::default()
            })
            .collect()
    }

    fn draw(&mut self, surface: &mut Surface<'_>) {
        let area = surface.area();
        let buffer = surface.buffer_mut();
        match self.at.clone() {
            Where::Captures => self.draw_captures(area, buffer),
            Where::Ladder => self.draw_list("guidance · ladder", area, buffer),
            Where::Tracking => self.draw_list("tracking", area, buffer),
            Where::Detail(id) => self.draw_detail(&id, area, buffer),
            Where::Capturing => self.draw_capture(area, buffer),
            Where::Faults => self.draw_faults(area, buffer),
            Where::Help => self.draw_help(area, buffer),
        }
    }

    fn draw_captures(&mut self, area: Rect, buffer: &mut ratatui::buffer::Buffer) {
        let held = self.rows();
        let groups = compose::captures(&held, self.cursor);
        let here = held
            .get(self.cursor)
            .map(compose::what_of)
            .unwrap_or_default();
        let shell = self.shell;
        shell.frame(
            &Frame {
                domain: "captures",
                meta: "",
                modal: Modal::Browsing,
                crumbs: &["captures", &here],
                position: "",
                keys: &[
                    ("↑↓", "move"),
                    ("↵", "open where it lives"),
                    ("c", "capture"),
                    ("g", "ladder"),
                    ("t", "tracking"),
                    ("?", "help"),
                ],
            },
            area,
            buffer,
            |body, buffer| browse::captures(&shell, &groups, &[], body, buffer),
        );
    }

    fn draw_list(&mut self, domain: &str, area: Rect, buffer: &mut ratatui::buffer::Buffer) {
        // The ladder and the location tree are the same walk over different
        // containers, and the replica already holds both orders.
        let held = self.rows();
        let shell = self.shell;
        let rows: Vec<(usize, Held)> = held.into_iter().map(|held| (0, held)).collect();
        let ladder = self.at == Where::Ladder;
        let rungs = compose::ladder(&rows, self.cursor);
        let kept = compose::tracking(&rows, self.cursor);
        shell.frame(
            &Frame {
                domain,
                meta: "",
                modal: Modal::Browsing,
                crumbs: &[domain],
                position: "",
                keys: &[("↑↓", "move"), ("↵", "detail"), ("esc", "back out")],
            },
            area,
            buffer,
            |body, buffer| {
                if ladder {
                    browse::ladder(&shell, &rungs, body, buffer);
                } else {
                    browse::tracking(&shell, &kept, body, buffer);
                }
            },
        );
    }

    fn draw_detail(&mut self, id: &[u8], area: Rect, buffer: &mut ratatui::buffer::Buffer) {
        let held = self.store.held(id).ok().flatten();
        let relations = self.store.relations_touching(id).unwrap_or_default();
        let store = &self.store;
        let links = compose::links(id, &relations, |other| {
            store
                .held(other)
                .ok()
                .flatten()
                .map(|held| compose::what_of(&held))
                .unwrap_or_default()
        });
        let title = held.as_ref().map(compose::what_of).unwrap_or_default();
        let body: Vec<String> = held
            .as_ref()
            .and_then(|held| held.body().map(str::to_string))
            .unwrap_or_default()
            .lines()
            .map(str::to_string)
            .collect();
        let lines: Vec<&str> = body.iter().map(String::as_str).collect();
        let shell = self.shell;
        shell.frame(
            &Frame {
                domain: "",
                meta: "",
                modal: Modal::Browsing,
                crumbs: &[&title],
                position: "",
                keys: &[("e", "edit in your editor"), ("esc", "back out")],
            },
            area,
            buffer,
            |body, buffer| browse::detail(&shell, &title, &lines, &links, body, buffer),
        );
    }

    fn draw_capture(&mut self, area: Rect, buffer: &mut ratatui::buffer::Buffer) {
        let shell = self.shell;
        let typed = self.typed.clone();
        shell.frame(
            &Frame {
                domain: "captures",
                meta: "",
                modal: Modal::Capturing,
                crumbs: &["captures"],
                position: "",
                keys: &[("↵", "keep it"), ("esc", "never mind")],
            },
            area,
            buffer,
            |body, buffer| write::capture(&shell, &typed, body, buffer),
        );
    }

    fn draw_faults(&mut self, area: Rect, buffer: &mut ratatui::buffer::Buffer) {
        let refused = self.refused();
        let shell = self.shell;
        shell.frame(
            &Frame {
                domain: "faults",
                meta: "",
                modal: Modal::Faults,
                crumbs: &["faults"],
                position: "",
                keys: &[("esc", "back out")],
            },
            area,
            buffer,
            |body, buffer| sideways::faults(&shell, &refused, body, buffer),
        );
    }

    fn draw_help(&mut self, area: Rect, buffer: &mut ratatui::buffer::Buffer) {
        let shell = self.shell;
        shell.frame(
            &Frame {
                domain: "",
                meta: "",
                modal: Modal::Help,
                crumbs: &["help"],
                position: "",
                keys: &[("esc", "back out")],
            },
            area,
            buffer,
            |body, buffer| shell.help_surface(body, buffer),
        );
    }

    fn key(&mut self, key: KeyEvent) {
        if self.at == Where::Capturing {
            self.typing(key);
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.leaving = true,
            KeyCode::Char('c') => self.go(Where::Capturing),
            KeyCode::Char('g') => self.go(Where::Ladder),
            KeyCode::Char('t') => self.go(Where::Tracking),
            KeyCode::Char('?') => self.go(Where::Help),
            KeyCode::Char('f') => self.go(Where::Faults),
            KeyCode::Up => self.cursor = self.cursor.saturating_sub(1),
            KeyCode::Down => {
                let last = self.rows().len().saturating_sub(1);
                self.cursor = (self.cursor + 1).min(last);
            }
            KeyCode::Enter => {
                if let Some(held) = self.rows().get(self.cursor) {
                    self.go(Where::Detail(held.entity_id.clone()));
                }
            }
            KeyCode::Char('e') => self.compose(),
            KeyCode::Esc => {
                self.at = self.came_from.pop().unwrap_or(Where::Captures);
            }
            _ => {}
        }
    }

    /// Capturing, which is the one thing that must never be expensive.
    ///
    /// **Return accepts and nothing else is asked for.** No title is required,
    /// no destination, and nothing about the network participates. The moment
    /// the store commits, it is theirs.
    fn typing(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.typed.clear();
                self.at = self.came_from.pop().unwrap_or(Where::Captures);
            }
            KeyCode::Backspace => {
                self.typed.pop();
            }
            KeyCode::Enter => {
                let typed = std::mem::take(&mut self.typed);
                self.accept(&typed);
                self.at = self.came_from.pop().unwrap_or(Where::Captures);
            }
            KeyCode::Char(character) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.typed.push(character);
            }
            _ => {}
        }
    }

    fn accept(&mut self, typed: &str) {
        let composed = wire::compose_creation(&Captured::Note {
            title: typed.to_string(),
            body: String::new(),
        });
        let taken = self.store.accept_creation(
            &composed.entity_id,
            &composed.content,
            None,
            &composed.intent,
        );
        if let Err(error) = taken {
            // The one local condition that reaches the guarantee rather than
            // delaying it, so it is said rather than swallowed.
            self.signals.unrecognised(true);
            eprintln!("altair: {error}");
        }
    }

    /// Hand the body to the person's own editor.
    ///
    /// The terminal has to be given back while they are in there, and taken
    /// again afterwards, because two programs cannot both own it.
    fn compose(&mut self) {
        let Where::Detail(id) = self.at.clone() else {
            return;
        };
        let seed = self
            .store
            .held(&id)
            .ok()
            .flatten()
            .and_then(|held| held.body().map(str::to_string))
            .unwrap_or_default();
        let Ok(composition) = self.editor.begin(&seed) else {
            self.signals.unrecognised(true);
            return;
        };
        ratatui::restore();
        let written = self.editor.edit(&composition);
        let _ = ratatui::try_init();
        match written {
            Ok(body) => {
                let (patch, intent) = wire::compose_body_edit(&id, &body);
                if self.store.accept_edit(&id, &patch, &intent).is_ok() {
                    let _ = self.editor.finish(&composition);
                }
            }
            // An editor that is not there will not become there because time
            // passed. The file stays, and is offered back.
            Err(_) => self.signals.unrecognised(true),
        }
    }

    fn go(&mut self, at: Where) {
        self.came_from.push(std::mem::replace(&mut self.at, at));
        self.cursor = 0;
    }
}
