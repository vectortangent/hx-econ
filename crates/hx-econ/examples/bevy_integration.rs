//! Bevy integration example
//!
//! Demonstrates using the EconomyPlugin with a Bevy app.

use bevy::prelude::*;
use hx_econ::prelude::*;

fn main() {
    println!("hx-econ Bevy integration example");
    println!("=================================\n");

    // Load goods configuration
    let goods_toml = include_str!("../config/goods.toml");
    let goods_registry = GoodsRegistry::from_toml(goods_toml)
        .expect("Failed to parse goods configuration");

    println!("Setting up Bevy app with {} goods", goods_registry.len());

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(EconomyPlugin {
            market_count: 5,
            good_count: goods_registry.len(),
            region_count: 2,
            goods_registry,
            config: EconomyConfig {
                seed: 12345,
                dt: FIXED_ONE,
                auto_step: false, // We'll step manually
            },
        })
        .add_systems(Startup, setup_markets)
        .add_systems(Update, (step_system, print_stats).chain())
        .run();
}

fn setup_markets(mut economy: ResMut<Economy>) {
    println!("Setting up markets...");

    // Add markets in different regions
    for i in 0..5 {
        let region = if i < 3 {
            RegionId::new(0)
        } else {
            RegionId::new(1)
        };
        let population = 1000 + i * 500;

        economy
            .runtime
            .market_state
            .add_market(region, population as i32, FIXED_ONE);

        println!("  Created Market {} in Region {} with {} population", i, region.0, population);
    }

    // Set initial stock levels
    for market_idx in 0..5 {
        let market = MarketId::new(market_idx);
        economy
            .runtime
            .market_state
            .set_stock(market, GoodId::new(0), to_fixed(100.0)); // Food
        economy
            .runtime
            .market_state
            .set_stock(market, GoodId::new(1), to_fixed(50.0)); // Wood
    }

    println!("Markets initialized!\n");
}

fn step_system(
    mut economy: ResMut<Economy>,
    mut step_events: EventWriter<SimulationStepEvent>,
    config: Res<EconomyConfig>,
) {
    // Only run for first 10 ticks
    if economy.runtime.tick >= 10 {
        std::process::exit(0);
    }

    // Step the simulation
    economy.runtime.step(config.dt);

    // Emit event
    step_events.send(SimulationStepEvent {
        tick: economy.runtime.tick,
    });
}

fn print_stats(mut step_reader: EventReader<SimulationStepEvent>, economy: Res<Economy>) {
    for event in step_reader.read() {
        if event.tick % 3 == 0 {
            println!("=== Tick {} ===", event.tick);

            // Print stats for first market
            let market = MarketId::new(0);
            for good_idx in 0..2 {
                // Just show first 2 goods
                let good = GoodId::new(good_idx);
                let stock = economy.runtime.market_state.get_stock(market, good);
                let price = economy.runtime.market_state.get_price(market, good);
                let good_name = &economy.runtime.goods_registry.goods[good_idx as usize].name;

                println!(
                    "  Market 0 - {}: stock={:.2}, price={:.3}",
                    good_name,
                    from_fixed(stock),
                    from_fixed(price)
                );
            }
            println!();
        }
    }
}
