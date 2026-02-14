//! Terrain field sampling for economy-terrain coupling
//!
//! Provides trait for sampling named fields from terrain (e.g., from hx-noise).

use crate::ids::RegionId;

/// Trait for sampling world fields at region locations
///
/// This allows the economy system to read terrain data (elevation, climate, etc.)
/// to modulate economic parameters like storage decay or transport costs.
pub trait WorldFieldSampler {
    /// Sample a named field at a region's location
    ///
    /// Returns None if the field doesn't exist or the region is invalid.
    /// Common field names might include:
    /// - "elevation" - terrain height
    /// - "temperature" - climate temperature
    /// - "humidity" - climate humidity
    /// - "terrain_cost" - base movement cost
    fn sample_field(&self, region: RegionId, field_name: &str) -> Option<f32>;

    /// Sample a field and convert to fixed-point
    fn sample_field_q(&self, region: RegionId, field_name: &str) -> Option<i32> {
        self.sample_field(region, field_name)
            .map(crate::util::to_fixed)
    }
}

/// Simple field sampler for testing that returns constant values
pub struct ConstantFieldSampler {
    pub value: f32,
}

impl WorldFieldSampler for ConstantFieldSampler {
    fn sample_field(&self, _region: RegionId, _field_name: &str) -> Option<f32> {
        Some(self.value)
    }
}

/// Field sampler that returns None for all queries (no terrain data)
pub struct NullFieldSampler;

impl WorldFieldSampler for NullFieldSampler {
    fn sample_field(&self, _region: RegionId, _field_name: &str) -> Option<f32> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_sampler() {
        let sampler = ConstantFieldSampler { value: 42.0 };
        let result = sampler.sample_field(RegionId::new(0), "test");
        assert_eq!(result, Some(42.0));
    }

    #[test]
    fn test_null_sampler() {
        let sampler = NullFieldSampler;
        let result = sampler.sample_field(RegionId::new(0), "test");
        assert_eq!(result, None);
    }
}
