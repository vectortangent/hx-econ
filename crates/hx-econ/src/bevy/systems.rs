//! Bevy systems for economy simulation

use bevy::prelude::*;
use crate::bevy::{Economy, EconomyConfig, SimulationStepEvent};

/// System that steps the economy simulation forward
pub fn step_economy(
    mut economy: ResMut<Economy>,
    config: Res<EconomyConfig>,
    mut step_events: EventWriter<SimulationStepEvent>,
) {
    if !config.auto_step {
        return;
    }

    // Run simulation step
    economy.runtime.step(config.dt);

    // Emit event
    step_events.send(SimulationStepEvent {
        tick: economy.runtime.tick,
    });
}
