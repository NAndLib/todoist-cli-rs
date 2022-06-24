//! Enum for supported item/task priority's
//!
//! The priority of the task, 1 is natural (default) and 4 is very urgent.
//!
//! ^note: for clients, "very urgent" is P1, so P1 returns 4 in the API.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Priority {
    P1 = 4,
    P2 = 3,
    P3 = 2,
    P4 = 1,
}

impl Default for Priority {
    fn default() -> Self {
        Self::P4
    }
}
