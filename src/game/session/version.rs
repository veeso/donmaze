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

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_tell_whether_version_is_compatible() {
        assert_eq!(Version::V010.is_compatible(), true);
    }

    #[test]
    fn should_serialize() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct Test {
            version: Version,
        }
        let test = Test {
            version: Version::V010,
        };
        let json = serde_json::to_string(&test).unwrap();
        let decoded: Test = serde_json::from_str(&json).unwrap();
        assert_eq!(test, decoded);
    }
}
