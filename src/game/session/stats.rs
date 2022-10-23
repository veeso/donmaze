//! # stats
//!
//! Game stats

use chrono::{DateTime, Local};

/// Game stats
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Stats {
    pub damage_inflicted: u64,
    pub damage_suffered: u64,
    pub enemies_killed: u64,
    pub fights_escaped: u64,
    pub items_used: u64,
    pub last_turn: DateTime<Local>,
    pub rooms_explored: u64,
    pub slept_for_turns: u64,
    pub turn: u64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            damage_inflicted: 0,
            damage_suffered: 0,
            enemies_killed: 0,
            fights_escaped: 0,
            items_used: 0,
            last_turn: Local::now(),
            rooms_explored: 1,
            slept_for_turns: 0,
            turn: 0,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct Test {
            stats: Stats,
        }
        let test = Test {
            stats: Stats::default(),
        };
        let json = serde_json::to_string(&test).unwrap();
        let decoded: Test = serde_json::from_str(&json).unwrap();
        assert_eq!(test, decoded);
    }
}
