//! Bevy ECS integration for economy simulation
//!
//! Provides plugin, resources, events, and systems for integrating the economy
//! runtime with Bevy ECS.

use bevy::prelude::*;

pub mod events;
pub mod plugin;
pub mod resources;
pub mod systems;

pub use events::*;
pub use plugin::EconomyPlugin;
pub use resources::*;
pub use systems::*;

/// System set for economy update stages
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum EconomySet {
    /// Collect mutations and dirty sets
    Collect,
    /// Apply mutations and run simulation step
    Simulate,
    /// Apply results back to ECS components
    Apply,
}
