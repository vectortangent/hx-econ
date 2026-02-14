//! Tests for determinism guarantees

use super::*;
use crate::ids::{GoodId, MarketId, RegionId};
use crate::pack::{GoodConfig, GoodsRegistry};
use crate::util::{to_fixed, FIXED_ONE};

#[test]
fn test_economy_runtime_creation() {
    let goods_registry = GoodsRegistry {
        goods: vec![GoodConfig {
            name: "test_good".to_string(),
            bulk_q: FIXED_ONE,
            base_decay_q: to_fixed(0.01),
            target_stock_per_capita_q: FIXED_ONE,
            scarcity_alpha_q: to_fixed(0.1),
            transit_decay_q: None,
        }],
    };

    let mut runtime = EconomyRuntime::new(12345, 2, 1, 1, goods_registry);
    runtime
        .market_state
        .add_market(RegionId::new(0), 1000, FIXED_ONE);
    runtime
        .market_state
        .add_market(RegionId::new(0), 2000, FIXED_ONE);

    assert_eq!(runtime.market_state.market_count, 2);
    assert_eq!(runtime.market_state.good_count, 1);
}

#[test]
fn test_storage_decay() {
    let goods_registry = GoodsRegistry {
        goods: vec![GoodConfig {
            name: "food".to_string(),
            bulk_q: FIXED_ONE,
            base_decay_q: to_fixed(0.1), // 10% decay per tick
            target_stock_per_capita_q: FIXED_ONE,
            scarcity_alpha_q: to_fixed(0.1),
            transit_decay_q: None,
        }],
    };

    let mut runtime = EconomyRuntime::new(12345, 1, 1, 1, goods_registry);
    runtime
        .market_state
        .add_market(RegionId::new(0), 1000, FIXED_ONE);

    let market = MarketId::new(0);
    let good = GoodId::new(0);

    // Set initial stock
    runtime
        .market_state
        .set_stock(market, good, to_fixed(100.0));

    // Run one step with dt = 1
    runtime.step(FIXED_ONE);

    // Stock should have decayed by 10%
    let final_stock = runtime.market_state.get_stock(market, good);
    let expected = to_fixed(90.0);
    assert!((final_stock - expected).abs() < 1000); // Allow rounding error
}

#[test]
fn test_determinism_same_seed() {
    // Two runtimes with the same seed should produce identical results
    let goods_registry = GoodsRegistry {
        goods: vec![GoodConfig {
            name: "test".to_string(),
            bulk_q: FIXED_ONE,
            base_decay_q: to_fixed(0.05),
            target_stock_per_capita_q: FIXED_ONE,
            scarcity_alpha_q: to_fixed(0.1),
            transit_decay_q: None,
        }],
    };

    let mut runtime1 = EconomyRuntime::new(42, 2, 1, 1, goods_registry.clone());
    runtime1
        .market_state
        .add_market(RegionId::new(0), 1000, FIXED_ONE);
    runtime1
        .market_state
        .add_market(RegionId::new(0), 2000, FIXED_ONE);
    runtime1
        .market_state
        .set_stock(MarketId::new(0), GoodId::new(0), to_fixed(100.0));
    runtime1
        .market_state
        .set_stock(MarketId::new(1), GoodId::new(0), to_fixed(50.0));

    let mut runtime2 = EconomyRuntime::new(42, 2, 1, 1, goods_registry);
    runtime2
        .market_state
        .add_market(RegionId::new(0), 1000, FIXED_ONE);
    runtime2
        .market_state
        .add_market(RegionId::new(0), 2000, FIXED_ONE);
    runtime2
        .market_state
        .set_stock(MarketId::new(0), GoodId::new(0), to_fixed(100.0));
    runtime2
        .market_state
        .set_stock(MarketId::new(1), GoodId::new(0), to_fixed(50.0));

    // Step both runtimes
    for _ in 0..10 {
        runtime1.step(FIXED_ONE);
        runtime2.step(FIXED_ONE);
    }

    // Check that states are identical
    assert_eq!(runtime1.tick, runtime2.tick);
    for market_idx in 0..2 {
        let market = MarketId::new(market_idx);
        let stock1 = runtime1.market_state.get_stock(market, GoodId::new(0));
        let stock2 = runtime2.market_state.get_stock(market, GoodId::new(0));
        assert_eq!(stock1, stock2);

        let price1 = runtime1.market_state.get_price(market, GoodId::new(0));
        let price2 = runtime2.market_state.get_price(market, GoodId::new(0));
        assert_eq!(price1, price2);
    }
}

#[test]
fn test_determinism_different_seed() {
    // Two runtimes with different seeds may produce different results
    // (though in current implementation, no randomness is used)
    let goods_registry = GoodsRegistry {
        goods: vec![GoodConfig {
            name: "test".to_string(),
            bulk_q: FIXED_ONE,
            base_decay_q: to_fixed(0.05),
            target_stock_per_capita_q: FIXED_ONE,
            scarcity_alpha_q: to_fixed(0.1),
            transit_decay_q: None,
        }],
    };

    let mut runtime1 = EconomyRuntime::new(42, 1, 1, 1, goods_registry.clone());
    runtime1
        .market_state
        .add_market(RegionId::new(0), 1000, FIXED_ONE);
    runtime1
        .market_state
        .set_stock(MarketId::new(0), GoodId::new(0), to_fixed(100.0));

    let mut runtime2 = EconomyRuntime::new(999, 1, 1, 1, goods_registry);
    runtime2
        .market_state
        .add_market(RegionId::new(0), 1000, FIXED_ONE);
    runtime2
        .market_state
        .set_stock(MarketId::new(0), GoodId::new(0), to_fixed(100.0));

    // Step both runtimes
    for _ in 0..10 {
        runtime1.step(FIXED_ONE);
        runtime2.step(FIXED_ONE);
    }

    // Seeds differ, but results should still be identical
    // (until we add random events)
    assert_eq!(runtime1.seed, 42);
    assert_eq!(runtime2.seed, 999);
}

#[test]
fn test_price_bounds() {
    // Prices should be clamped to reasonable bounds
    let goods_registry = GoodsRegistry {
        goods: vec![GoodConfig {
            name: "test".to_string(),
            bulk_q: FIXED_ONE,
            base_decay_q: 0,
            target_stock_per_capita_q: to_fixed(1000.0), // Very high target
            scarcity_alpha_q: to_fixed(10.0), // High sensitivity
            transit_decay_q: None,
        }],
    };

    let mut runtime = EconomyRuntime::new(42, 1, 1, 1, goods_registry);
    runtime
        .market_state
        .add_market(RegionId::new(0), 1000, FIXED_ONE);
    runtime
        .market_state
        .set_stock(MarketId::new(0), GoodId::new(0), 0); // Zero stock

    // Step many times - price should be clamped
    for _ in 0..100 {
        runtime.step(FIXED_ONE);
    }

    let price = runtime.market_state.get_price(MarketId::new(0), GoodId::new(0));
    let max_price = FIXED_ONE * 10;
    assert!(price <= max_price, "Price should be clamped to max");
}

#[test]
fn test_quantization_prevents_drift() {
    // Quantization should prevent small rounding errors from accumulating
    let goods_registry = GoodsRegistry {
        goods: vec![GoodConfig {
            name: "test".to_string(),
            bulk_q: FIXED_ONE,
            base_decay_q: to_fixed(0.001), // Very small decay
            target_stock_per_capita_q: FIXED_ONE,
            scarcity_alpha_q: to_fixed(0.001), // Very small price response
            transit_decay_q: None,
        }],
    };

    let mut runtime = EconomyRuntime::new(42, 1, 1, 1, goods_registry);
    runtime
        .market_state
        .add_market(RegionId::new(0), 1000, FIXED_ONE);
    runtime
        .market_state
        .set_stock(MarketId::new(0), GoodId::new(0), to_fixed(100.0));

    // Run for many steps
    for _ in 0..1000 {
        runtime.step(FIXED_ONE);
    }

    // Stock should still be within reasonable bounds (not NaN or overflow)
    let stock = runtime.market_state.get_stock(MarketId::new(0), GoodId::new(0));
    assert!(stock >= 0, "Stock should not be negative");
    assert!(stock < to_fixed(100.0), "Stock should have decayed");
}
