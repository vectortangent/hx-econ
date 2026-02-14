# hx-econ

Deterministic price-diffusion economy simulation with Bevy ECS integration.

## Features

- **Deterministic**: Reproducible results given the same seed and inputs
- **Incremental**: Dirty-set based recomputation for efficiency
- **Price-diffusion**: Market prices respond to supply/demand imbalances
- **Congestion modeling**: Transport costs increase with edge usage
- **Storage decay**: Configurable spoilage rates for goods
- **Terrain coupling**: Economic parameters modulated by terrain fields
- **Bevy ECS integration**: Full integration with Bevy for orchestration

## Core Concepts

### Markets
Locations where goods are stored, priced, and traded. Each market has:
- Population (affects demand)
- Storage quality modifier (affects decay rates)
- Per-good stock levels
- Per-good prices

### Goods
Commodities with configurable properties:
- `bulk_q`: Congestion weight (how much space it takes)
- `base_decay_q`: Storage decay rate per tick
- `target_stock_per_capita_q`: Desired stock level per population
- `scarcity_alpha_q`: Price response sensitivity

### Transport Graph
Network of edges connecting markets:
- Base cost (distance, terrain difficulty)
- Usage-dependent congestion
- Sorted by region for efficient iteration

### Regions
Spatial grouping for efficient computation and dirty-set tracking.

## Architecture

### Determinism Guarantees

All state updates are pure functions:
```
S_{t+1} = Step(S_t, dt, DirtySet, MutationList)
```

Key determinism features:
- Fixed-point arithmetic (Q16.16 format) for all calculations
- Stable iteration order (ascending IDs)
- No HashMap iteration in hot loops
- Quantization at phase boundaries to prevent drift
- Keyed RNG for any random events

### Data Layout

Dense, SIMD-friendly storage:
- Markets: `[0..market_count)`
- Goods: `[0..good_count)`
- Edges: sorted by `(region, from, to, edge_id)`

Stock and price arrays are flattened:
```
index = market_id * good_count + good_id
```

### Hot Loop Optimization

- No allocations in diffusion loops
- Structure-of-Arrays (SoA) for cache efficiency
- Pre-sorted iteration for determinism and cache coherence
- Bitsets for dirty tracking

## Usage

### Basic Example

```rust
use hx_econ::prelude::*;
use bevy::prelude::*;

fn main() {
    // Load goods configuration
    let goods_toml = r#"
    [[goods]]
    name = "food"
    bulk_q = 65536
    base_decay_q = 6553
    target_stock_per_capita_q = 131072
    scarcity_alpha_q = 32768
    "#;
    
    let goods_registry = GoodsRegistry::from_toml(goods_toml).unwrap();

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(EconomyPlugin {
            market_count: 10,
            good_count: goods_registry.len(),
            region_count: 2,
            goods_registry,
            config: EconomyConfig {
                seed: 12345,
                dt: FIXED_ONE,
                auto_step: true,
            },
        })
        .run();
}
```

### Manual Stepping

```rust
fn my_system(mut economy: ResMut<Economy>) {
    // Manually step the economy
    economy.runtime.step(FIXED_ONE);
    
    // Read market state
    let market = MarketId::new(0);
    let good = GoodId::new(0);
    let stock = economy.runtime.market_state.get_stock(market, good);
    let price = economy.runtime.market_state.get_price(market, good);
}
```

## Configuration

Goods are configured via TOML:

```toml
[[goods]]
name = "food"
bulk_q = 65536              # 1.0 in fixed-point
base_decay_q = 6553         # ~0.1 (10% per tick)
target_stock_per_capita_q = 131072  # 2.0 per capita
scarcity_alpha_q = 32768    # 0.5 price sensitivity

[[goods]]
name = "wood"
bulk_q = 131072             # 2.0 in fixed-point
base_decay_q = 0            # No decay
target_stock_per_capita_q = 65536   # 1.0 per capita
scarcity_alpha_q = 16384    # 0.25 price sensitivity
```

Fixed-point scale: `FIXED_ONE = 65536 = 2^16`

## License

Licensed under either of:
- Apache License, Version 2.0
- MIT license

at your option.
