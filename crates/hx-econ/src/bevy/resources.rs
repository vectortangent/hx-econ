//! Bevy resources for economy system

use bevy::prelude::*;
use crate::runtime::EconomyRuntime;

/// Main economy runtime resource
#[derive(Resource)]
pub struct Economy {
    pub runtime: EconomyRuntime,
}

/// Configuration for economy simulation
#[derive(Resource, Clone)]
pub struct EconomyConfig {
    /// Simulation seed
    pub seed: u64,
    /// Delta time per step (fixed-point)
    pub dt: i32,
    /// Enable automatic stepping
    pub auto_step: bool,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            dt: crate::util::FIXED_ONE,
            auto_step: true,
        }
    }
}
