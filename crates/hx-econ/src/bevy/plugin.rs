//! Economy plugin for Bevy

use bevy::prelude::*;
use crate::bevy::{
    Economy, EconomyConfig, EconomySet, PriceChangeEvent, SimulationStepEvent, StockChangeEvent,
    step_economy,
};
use crate::pack::GoodsRegistry;
use crate::runtime::EconomyRuntime;

/// Plugin that adds economy simulation to a Bevy app
pub struct EconomyPlugin {
    /// Number of markets to allocate
    pub market_count: usize,
    /// Number of goods
    pub good_count: usize,
    /// Number of regions
    pub region_count: usize,
    /// Goods registry
    pub goods_registry: GoodsRegistry,
    /// Configuration
    pub config: EconomyConfig,
}

impl Default for EconomyPlugin {
    fn default() -> Self {
        Self {
            market_count: 0,
            good_count: 0,
            region_count: 0,
            goods_registry: GoodsRegistry::default(),
            config: EconomyConfig::default(),
        }
    }
}

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        // Create economy runtime
        let runtime = EconomyRuntime::new(
            self.config.seed,
            self.market_count,
            self.good_count,
            self.region_count,
            self.goods_registry.clone(),
        );

        // Add resources
        app.insert_resource(Economy { runtime })
            .insert_resource(self.config.clone());

        // Add events
        app.add_event::<StockChangeEvent>()
            .add_event::<PriceChangeEvent>()
            .add_event::<SimulationStepEvent>();

        // Configure system sets
        app.configure_sets(
            Update,
            (
                EconomySet::Collect,
                EconomySet::Simulate,
                EconomySet::Apply,
            )
                .chain(),
        );

        // Add systems
        app.add_systems(Update, step_economy.in_set(EconomySet::Simulate));
    }
}
