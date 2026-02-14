//! Fixed-point arithmetic for deterministic calculations
//!
//! Uses power-of-2 scaling for efficient shift-based operations.
//! All fixed-point values use Q16.16 format (16 integer bits, 16 fractional bits).

/// Fixed-point scaling factor (2^16)
pub const FIXED_SCALE: i32 = 1 << 16;
pub const FIXED_ONE: i32 = FIXED_SCALE;
pub const FIXED_HALF: i32 = FIXED_SCALE / 2;

/// Convert a floating-point value to fixed-point
#[inline]
pub fn to_fixed(f: f32) -> i32 {
    (f * FIXED_SCALE as f32) as i32
}

/// Convert a fixed-point value to floating-point
#[inline]
pub fn from_fixed(q: i32) -> f32 {
    q as f32 / FIXED_SCALE as f32
}

/// Multiply two fixed-point values
#[inline]
pub fn fixed_mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) >> 16) as i32
}

/// Divide two fixed-point values
#[inline]
pub fn fixed_div(a: i32, b: i32) -> i32 {
    if b == 0 {
        return i32::MAX;
    }
    (((a as i64) << 16) / b as i64) as i32
}

/// Clamp a value between min and max
#[inline]
pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Quantize a value to prevent accumulation drift
/// Rounds to nearest 1/256 of a unit
#[inline]
pub fn quantize(value: i32) -> i32 {
    let quantum = FIXED_SCALE / 256;
    ((value + quantum / 2) / quantum) * quantum
}

/// Linear interpolation in fixed-point
#[inline]
pub fn lerp(a: i32, b: i32, t: i32) -> i32 {
    a + fixed_mul(b - a, t)
}

/// Exponential decay in fixed-point
/// Returns: value * (1 - decay_rate)
#[inline]
pub fn apply_decay(value: i32, decay_rate_q: i32) -> i32 {
    fixed_mul(value, FIXED_ONE - decay_rate_q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_conversion() {
        assert_eq!(to_fixed(1.0), FIXED_ONE);
        assert_eq!(to_fixed(0.5), FIXED_HALF);
        assert_eq!(from_fixed(FIXED_ONE), 1.0);
    }

    #[test]
    fn test_fixed_mul() {
        let a = to_fixed(2.0);
        let b = to_fixed(3.0);
        let result = fixed_mul(a, b);
        assert_eq!(from_fixed(result), 6.0);
    }

    #[test]
    fn test_fixed_div() {
        let a = to_fixed(6.0);
        let b = to_fixed(3.0);
        let result = fixed_div(a, b);
        assert_eq!(from_fixed(result), 2.0);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_decay() {
        let value = to_fixed(100.0);
        let decay_10_percent = to_fixed(0.1);
        let result = apply_decay(value, decay_10_percent);
        let expected = to_fixed(90.0);
        assert!((result - expected).abs() < 100); // Allow small rounding error
    }
}
