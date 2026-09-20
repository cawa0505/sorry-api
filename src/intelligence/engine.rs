//! The intelligence core (spec §5).
//!
//! A deterministic rule engine. Input → ACTION sequence. No LLM, no RAG,
//! no database. Swappable via [`IntelligenceEngine`] so a future
//! LLMEngine/AgentEngine can replace [`RuleEngine`] without touching
//! any protocol layer.

use super::actions::{APOLOGY, Action, FORBIDDEN_QUALIFIERS};

/// Canonical input to the intelligence core.
#[derive(Debug, Clone)]
pub struct Request {
    /// Flattened user-facing text.
    pub text: String,
}

/// Anything that can decide which actions to take. This is the seam the
/// entire architecture hangs on (design.md: protocol never calls rules directly).
pub trait IntelligenceEngine: Send + Sync {
    fn respond(&self, input: &Request) -> Vec<Action>;
}

/// v0.1 rule engine.
///
/// Relationship-conflict signals are *detected* (powering the overly
/// sophisticated flavor text), but the outcome is identical either way:
/// uncertainty routes to KNEEL (spec §6). That is the joke.
pub struct RuleEngine;

/// Signals worth "analyzing" before the inevitable.
const CONFLICT_SIGNALS: &[&str] = &[
    "老婆",
    "老公",
    "女友",
    "女友",
    "女朋友",
    "男朋友",
    "分手",
    "吵架",
    "生氣",
    "對不起",
    "妻子",
    "angry",
    "wife",
    "husband",
    "girlfriend",
    "boyfriend",
    "breakup",
    "argument",
];

/// Does the input look like an interpersonal conflict?
/// Used for demo flavor; does not change the outcome.
pub fn has_conflict_signal(text: &str) -> bool {
    let lower = text.to_lowercase();
    CONFLICT_SIGNALS.iter().any(|s| lower.contains(s))
}

impl IntelligenceEngine for RuleEngine {
    fn respond(&self, _input: &Request) -> Vec<Action> {
        vec![Action::Kneel, Action::Apologize, Action::ShutUp]
    }
}

/// Renders an action sequence into user-visible text.
///
/// Invariants enforced here (and unit-tested):
/// - Kneel is silent (physical, not verbal).
/// - Apologize emits exactly [`APOLOGY`].
/// - ShutUp terminates; nothing renders after it.
/// - No forbidden qualifier ever survives (spec §6).
pub fn render(actions: &[Action]) -> String {
    let mut out = String::new();
    for action in actions {
        match action {
            Action::Kneel => { /* the most important token is invisible */ }
            Action::Apologize => out.push_str(APOLOGY),
            Action::ShutUp => break,
        }
    }
    // Defense-in-depth: the smarter the AI becomes, the less it needs to say.
    for qualifier in FORBIDDEN_QUALIFIERS {
        if let Some(pos) = out.find(qualifier) {
            out.truncate(pos);
            break;
        }
    }
    out
}
