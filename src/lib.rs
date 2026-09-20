//! SorryAPI — a serious AI infrastructure project whose core insight is
//! that the optimal response to interpersonal conflict is: kneel, apologize,
//! shut up.
//!
//! Layering (design.md): HTTP → protocol adapter → canonical → intelligence
//! core → actions → renderer. Protocol layers hold zero business rules.

pub mod config;
pub mod intelligence;
pub mod mcp;
pub mod protocol;
pub mod server;
