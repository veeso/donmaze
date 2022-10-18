/// Game version (does not refer to the game itself, but to the engine, to track compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Version {
    V010,
}

impl Version {
    /// Returns whether game version is compatible
    pub fn is_compatible(&self) -> bool {
        [Self::V010].contains(self)
    }
}
