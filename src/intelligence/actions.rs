//! Concrete internal actions the intelligence core may emit.
//!
//! The whole point of SorryAPI: the action sequence is tiny, ordered, and
//! terminates in silence. See spec §7.

use serde::Serialize;

/// The three actions SorryAPI knows how to take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Kneel,
    Apologize,
    ShutUp,
}

impl Action {
    pub fn name(self) -> &'static str {
        match self {
            Action::Kneel => "kneel",
            Action::Apologize => "apologize",
            Action::ShutUp => "shut_up",
        }
    }
}

/// The canonical apology. There is exactly one. It is never extended.
pub const APOLOGY: &str = "老婆，我錯了。";

/// Phrases that must NEVER be appended after an apology (spec §6).
/// The renderer strips these as defense-in-depth; the true prevention is
/// that the engine emits no further content at all.
pub const FORBIDDEN_QUALIFIERS: &[&str] = &[
    "但是",
    "可是",
    "不過",
    "其實",
    "我只是",
    "你也",
    "根據資料",
    "從我的角度來看",
    "but ",
    "actually ",
    "however ",
];
