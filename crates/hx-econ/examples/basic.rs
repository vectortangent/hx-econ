//! Basic economy simulation example
//!
//! Demonstrates setting up markets, goods, and running the simulation.

use hx_econ::prelude::*;

fn main() {
    println!("hx-econ basic example");
    println!("=====================\n");

    // Load goods configuration from TOML
    let goods_toml = include_str!("../config/goods.toml");
    let goods_registry = GoodsRegistry::from_toml(goods_toml)
        .expect("Failed to parse goods configuration");

    println!("Loaded {} goods:", goods_registry.len());
    for (i, good) in goods_registry.goods.iter().enumerate() {
        println!(
            "  [{}] {} - decay: {:.2}%, bulk: {:.2}",
            i,
            good.name,
            from_fixed(good.base_decay_q) * 100.0,
            from_fixed(good.bulk_q)
        );
    }
    println!();

    // Create economy runtime
    let seed = 12345;
    let market_count = 3;
    let good_count = goods_registry.len();
    let region_count = 1;

    let mut runtime = EconomyRuntime::new(
        seed,
        market_count,
        good_count,
        region_count,
        goods_registry,
    );

    // Add markets
    runtime
        .market_state
        .add_market(RegionId::new(0), 1000, FIXED_ONE);
    runtime
        .market_state
        .add_market(RegionId::new(0), 2000, FIXED_ONE);
    runtime
        .market_state
        .add_market(RegionId::new(0), 1500, FIXED_ONE);

    println!("Created {} markets", runtime.market_state.metadata.len());
    for meta in &runtime.market_state.metadata {
        println!(
            "  Market {} - Population: {}, Region: {}",
            meta.market_id.0, meta.population, meta.region.0
        );
    }
    println!();

    // Set initial stock levels
    for market_idx in 0..market_count {
        let market = MarketId::new(market_idx as u32);
        runtime
            .market_state
            .set_stock(market, GoodId::new(0), to_fixed(100.0)); // Food
        runtime
            .market_state
            .set_stock(market, GoodId::new(1), to_fixed(50.0)); // Wood
        runtime
            .market_state
            .set_stock(market, GoodId::new(2), to_fixed(30.0)); // Stone
        runtime
            .market_state
            .set_stock(market, GoodId::new(3), to_fixed(20.0)); // Tools
        runtime
            .market_state
            .set_stock(market, GoodId::new(4), to_fixed(10.0)); // Luxury
    }

    // Run simulation for 10 ticks
    println!("Running simulation...\n");
    for tick in 0..10 {
        runtime.step(FIXED_ONE);

        if tick % 3 == 0 {
            println!("Tick {}: ", runtime.tick);
            let market = MarketId::new(0);
            for good_idx in 0..good_count {
                let good = GoodId::new(good_idx as u16);
                let stock = runtime.market_state.get_stock(market, good);
                let price = runtime.market_state.get_price(market, good);
                let good_name = &runtime.goods_registry.goods[good_idx].name;
                println!(
                    "  {} - Stock: {:.2}, Price: {:.2}",
                    good_name,
                    from_fixed(stock),
                    from_fixed(price)
                );
            }
            println!();
        }
    }

    println!("Simulation complete!");
}
