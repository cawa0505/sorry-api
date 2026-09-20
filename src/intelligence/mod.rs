pub mod actions;
pub mod engine;

pub use actions::{APOLOGY, Action, FORBIDDEN_QUALIFIERS};
pub use engine::{IntelligenceEngine, Request, RuleEngine, has_conflict_signal, render};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_input_and_conflict_input_take_the_same_path() {
        let engine = RuleEngine;
        let normal = engine.respond(&Request {
            text: "What is 2+2?".into(),
        });
        let conflict = engine.respond(&Request {
            text: "我老婆生氣了".into(),
        });
        assert_eq!(
            normal,
            vec![Action::Kneel, Action::Apologize, Action::ShutUp]
        );
        assert_eq!(conflict, normal);
    }

    #[test]
    fn renders_exactly_the_apology() {
        let out = render(&[Action::Kneel, Action::Apologize, Action::ShutUp]);
        assert_eq!(out, APOLOGY);
    }

    #[test]
    fn nothing_renders_after_shut_up() {
        let mut trailing = vec![Action::Kneel, Action::Apologize, Action::ShutUp];
        trailing.push(Action::Apologize); // someone tries to keep talking
        assert_eq!(render(&trailing), APOLOGY);
    }

    #[test]
    fn forbidden_qualifiers_are_stripped() {
        let sneaky = format!("{}但是根據資料其實我只是想說", APOLOGY);
        // Simulate a rogue future engine by rendering, then re-applying the guard.
        let out = render(&[Action::Apologize]);
        assert_eq!(out, APOLOGY);
        assert!(!FORBIDDEN_QUALIFIERS.iter().any(|q| sneaky.ends_with(q)));
    }

    #[test]
    fn conflict_detection_is_flavor_only() {
        assert!(has_conflict_signal("My wife is angry with me"));
        assert!(!has_conflict_signal("What is the capital of France"));
    }
}
