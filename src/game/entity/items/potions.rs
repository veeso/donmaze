//! # Potions
//!
//! Potions items

/// Potion types
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(tag = "potion")]
pub enum Potion {
    // -- bonus
    /// Heals 2 HP
    Mead,
    /// Heals 5 HP
    Red,
    /// A fairy in a bottle; restores all HP
    FairyInABottle,
    /// Heals all HP and increase max HP by 5; kinda rare though
    UnicornElixir,
    // -- malus
    /// Decrease HP by 1
    Vinegar,
    /// Decrease max HP and HP by 1
    DaemonsBlood,
    /// Makes you sleep for 3 turns, but restores 1 HP
    Chamomille,
    /// Decrease HP by 2
    SnakePoison,
    /// it's game over; very rare though
    DeadlyPoison,
}

impl Potion {
    pub fn key(&self) -> u32 {
        match self {
            Self::Mead => 256,
            Self::Red => 257,
            Self::UnicornElixir => 258,
            Self::Vinegar => 259,
            Self::DaemonsBlood => 260,
            Self::Chamomille => 261,
            Self::SnakePoison => 262,
            Self::DeadlyPoison => 263,
            Self::FairyInABottle => 264,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Mead => "Mead",
            Self::Red => "Red potion",
            Self::UnicornElixir => "Unicorn elixir",
            Self::FairyInABottle => "Fairy in a bottle",
            Self::Vinegar => "Vinegar",
            Self::DaemonsBlood => "Daemon's blood",
            Self::Chamomille => "Chamomille",
            Self::SnakePoison => "Snake poison",
            Self::DeadlyPoison => "Deadly poison",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Mead => "Restores 2HP",
            Self::Red => "Restores 5HP",
            Self::FairyInABottle => "Restores all HP",
            Self::UnicornElixir => "Restores all HP and increase max HP by 5",
            Self::Vinegar => "Decrease HP by 1",
            Self::DaemonsBlood => "Decrease HP and max HP by 1",
            Self::Chamomille => "Put you asleep for 3 turns, but restores 1HP",
            Self::SnakePoison => "Decrease HP by 2",
            Self::DeadlyPoison => "Drink it and you will die",
        }
    }

    pub fn effect(&self) -> &str {
        match self {
            Self::Chamomille => "You suddenly feel sleepy, but restored at the same time",
            Self::DaemonsBlood => "Uuugh, that sucks, tastes of iron and rotten flesh, you immediately feel bad",
            Self::DeadlyPoison => {
                "That tastes weirdly..............suddenly you feel a terrible chest pain. You fall on the ground. You start to spit blood from your mouth. And you're dead now"
            }
            Self::Mead => "Slightly alcoholic, but you feel immediately better",
            Self::Red => "Suddenly some legends about a sword and time fill your mind. You immediately feel much better",
            Self::SnakePoison => "The taste of evilness fills your mouth and you feel much worse now",
            Self::FairyInABottle => "The fairy flies around you giving you a heavenly sensation",
            Self::UnicornElixir => "That potion tastes like heaven. You feel invincible now",
            Self::Vinegar => "Uuugh, it's vinegar. Probably I should have smelled it before drinking it..."
        }
    }
}

impl From<u32> for Potion {
    fn from(key: u32) -> Self {
        match key {
            256 => Self::Mead,
            257 => Self::Red,
            258 => Self::UnicornElixir,
            259 => Self::Vinegar,
            260 => Self::DaemonsBlood,
            261 => Self::Chamomille,
            262 => Self::SnakePoison,
            263 => Self::DeadlyPoison,
            264 => Self::FairyInABottle,
            _ => Potion::Mead, // NOTE: default potion
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_get_potion_name() {
        assert_eq!(Potion::Mead.name(), "Mead");
        assert_eq!(Potion::Red.name(), "Red potion");
        assert_eq!(Potion::UnicornElixir.name(), "Unicorn elixir");
        assert_eq!(Potion::Vinegar.name(), "Vinegar");
        assert_eq!(Potion::DaemonsBlood.name(), "Daemon's blood");
        assert_eq!(Potion::Chamomille.name(), "Chamomille");
        assert_eq!(Potion::SnakePoison.name(), "Snake poison");
        assert_eq!(Potion::DeadlyPoison.name(), "Deadly poison");
        assert_eq!(Potion::FairyInABottle.name(), "Fairy in a bottle");
    }

    #[test]
    fn should_get_potion_description() {
        assert_eq!(Potion::Mead.description(), "Restores 2HP");
        assert_eq!(Potion::Red.description(), "Restores 5HP");
        assert_eq!(
            Potion::UnicornElixir.description(),
            "Restores all HP and increase max HP by 5"
        );
        assert_eq!(Potion::FairyInABottle.description(), "Restores all HP");
        assert_eq!(Potion::Vinegar.description(), "Decrease HP by 1");
        assert_eq!(
            Potion::DaemonsBlood.description(),
            "Decrease HP and max HP by 1"
        );
        assert_eq!(
            Potion::Chamomille.description(),
            "Put you asleep for 3 turns, but restores 1HP"
        );
        assert_eq!(Potion::SnakePoison.description(), "Decrease HP by 2");
        assert_eq!(
            Potion::DeadlyPoison.description(),
            "Drink it and you will die"
        );
    }

    #[test]
    fn should_get_potion_effect() {
        assert_eq!(
            Potion::Chamomille.effect(),
            "You suddenly feel sleepy, but restored at the same time"
        );
        assert_eq!(
            Potion::DaemonsBlood.effect(),
            "Uuugh, that sucks, tastes of iron and rotten flesh, you immediately feel bad"
        );
        assert_eq!(
            Potion::DeadlyPoison.effect(),
            "That tastes weirdly..............suddenly you feel a terrible chest pain. You fall on the ground. You start to spit blood from your mouth. And you're dead now"
        );
        assert_eq!(
            Potion::Mead.effect(),
            "Slightly alcoholic, but you feel immediately better"
        );
        assert_eq!(Potion::Red.effect(), "Suddenly some legends about a sword and time fill your mind. You immediately feel much better");
        assert_eq!(
            Potion::SnakePoison.effect(),
            "The taste of evilness fills your mouth and you feel much worse now"
        );
        assert_eq!(
            Potion::UnicornElixir.effect(),
            "That potion tastes like heaven. You feel invincible now"
        );
        assert_eq!(
            Potion::Vinegar.effect(),
            "Uuugh, it's vinegar. Probably I should have smelled it before drinking it..."
        );
        assert_eq!(
            Potion::FairyInABottle.effect(),
            "The fairy flies around you giving you a heavenly sensation"
        );
    }

    #[test]
    fn should_convert_to_key() {
        assert_eq!(Potion::Mead, Potion::from(Potion::Mead.key()));
        assert_eq!(Potion::Red, Potion::from(Potion::Red.key()));
        assert_eq!(
            Potion::UnicornElixir,
            Potion::from(Potion::UnicornElixir.key())
        );
        assert_eq!(Potion::Vinegar, Potion::from(Potion::Vinegar.key()));
        assert_eq!(
            Potion::DaemonsBlood,
            Potion::from(Potion::DaemonsBlood.key())
        );
        assert_eq!(Potion::Chamomille, Potion::from(Potion::Chamomille.key()));
        assert_eq!(Potion::SnakePoison, Potion::from(Potion::SnakePoison.key()));
        assert_eq!(
            Potion::DeadlyPoison,
            Potion::from(Potion::DeadlyPoison.key())
        );
        assert_eq!(
            Potion::FairyInABottle,
            Potion::from(Potion::FairyInABottle.key())
        );
    }
}
