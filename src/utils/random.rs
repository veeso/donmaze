//! # Random
//!
//! Random utilities

use rand::{distributions::Alphanumeric, rngs::ThreadRng, thread_rng, Rng};

/// Choose a random element from `choices`
pub fn choice<'a, T>(rng: &'a mut ThreadRng, choices: &'a [T]) -> &'a T {
    &choices[rng.gen_range(0..choices.len())]
}

/// Given a percentage, returns whether the event should happen
/// Panics if `probability` is out of range 1-100
pub fn happens(rng: &mut ThreadRng, probability: u8) -> bool {
    assert!(probability <= 100);
    rng.gen_range(0..100) < probability
}

/// Generate a random alphanumeric string with provided length
pub fn random_alphanumeric_with_len(rng: &mut ThreadRng, len: usize) -> String {
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

/// Get random generator
pub fn rng() -> ThreadRng {
    thread_rng()
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_tell_whether_event_happens() {
        assert!(happens(&mut rng(), 100));
        assert_eq!(happens(&mut rng(), 0), false);
    }

    #[test]
    fn should_make_choice() {
        assert!(&[1, 2, 3].contains(choice(&mut rng(), &[1, 2, 3])));
    }

    #[test]
    fn should_generate_random_alphanumeric_with_len() {
        assert_eq!(random_alphanumeric_with_len(&mut rng(), 256).len(), 256);
    }
}
