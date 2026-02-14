//! Deterministic RNG utilities
//!
//! Provides keyed random number generation for deterministic simulation.

/// Simple keyed RNG for deterministic randomness
/// Uses a hash-based approach with seed, purpose tag, tick, and entity ID
pub fn keyed_rand(seed: u64, purpose: u32, tick: u64, entity_id: u32) -> u64 {
    // Simple hash combining all inputs
    let mut h = seed;
    h ^= purpose as u64;
    h = h.wrapping_mul(6364136223846793005);
    h ^= tick;
    h = h.wrapping_mul(6364136223846793005);
    h ^= entity_id as u64;
    h = h.wrapping_mul(6364136223846793005);
    h ^= h >> 32;
    h
}

/// Generate a random i32 in range [0, FIXED_ONE)
pub fn keyed_rand_fixed(seed: u64, purpose: u32, tick: u64, entity_id: u32) -> i32 {
    use super::fixed::FIXED_ONE;
    let r = keyed_rand(seed, purpose, tick, entity_id);
    ((r % FIXED_ONE as u64) as i32).abs()
}

/// Generate a random i32 in range [min, max]
pub fn keyed_rand_range(
    seed: u64,
    purpose: u32,
    tick: u64,
    entity_id: u32,
    min: i32,
    max: i32,
) -> i32 {
    if min >= max {
        return min;
    }
    let r = keyed_rand(seed, purpose, tick, entity_id);
    let range = (max - min) as u64 + 1;
    min + (r % range) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyed_rand_deterministic() {
        let r1 = keyed_rand(12345, 1, 100, 42);
        let r2 = keyed_rand(12345, 1, 100, 42);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_keyed_rand_different_inputs() {
        let r1 = keyed_rand(12345, 1, 100, 42);
        let r2 = keyed_rand(12345, 1, 100, 43);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_keyed_rand_range() {
        let r = keyed_rand_range(12345, 1, 100, 42, 10, 20);
        assert!(r >= 10 && r <= 20);
    }
}
