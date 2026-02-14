//! Architecture documentation for hx-econ
//!
//! This module provides detailed documentation on the internal architecture
//! and design decisions.

/*! # Architecture Overview

## Data Layout

The economy system uses Structure-of-Arrays (SoA) layout for cache-friendly access:

```text
MarketState:
  - metadata: [MarketMetadata; market_count]
  - stock:    [i32; market_count * good_count]  (flattened 2D array)
  - price:    [i32; market_count * good_count]  (flattened 2D array)
```

Array indexing is done via: `index = market_id * good_count + good_id`

## Determinism Strategy

### Fixed-Point Arithmetic
All economic calculations use Q16.16 fixed-point format:
- Integer part: 16 bits
- Fractional part: 16 bits
- Scale factor: 65536 (2^16)
- Range: -32768.0 to +32767.99998

### Stable Iteration Order
All collections use sorted, dense IDs:
- Markets: [0..market_count)
- Goods: [0..good_count)
- Edges: sorted by (region, from, to, edge_id)

No HashMap iteration in hot loops!

### Quantization
State values are quantized at phase boundaries to prevent accumulation of
rounding errors. Quantum size: 1/256 of a unit.

### Keyed RNG
Any randomness uses deterministic keyed RNG:
```text
rand(seed, purpose_tag, tick, entity_id)
```

## Hot Loop Optimization

### No Allocations
Price diffusion and decay loops operate entirely on pre-allocated buffers.
No Vec::push, no HashMap::insert.

### SIMD-Friendly Layout
Dense, contiguous arrays enable auto-vectorization:
- Sequential memory access patterns
- Predictable stride lengths
- No pointer chasing

### Dirty Sets
Incremental recomputation using bitsets:
- Only process changed markets/edges
- O(1) bit test, O(population) iteration over set bits
- Cleared after each step

## Transport Graph

Edges are stored sorted by region for cache coherence:

```text
TransportGraph:
  edges:         [Edge_R0_0, Edge_R0_1, ..., Edge_R1_0, Edge_R1_1, ...]
  region_ranges: [(0, 100), (100, 250), ...]
                     ^R0      ^R1
```

Benefits:
- Sequential access within region
- No indirection for region queries
- Enables parallel region processing

## Bevy Integration

### Resource Flow
```text
ECS Components → Collect → Mutations → Runtime.step() → Apply → ECS Components
                   ↓                                      ↑
                 DirtySet ──────────────────────────────┘
```

### System Sets
- `EconomySet::Collect`: Gather mutations from ECS
- `EconomySet::Simulate`: Run economy step
- `EconomySet::Apply`: Write results back to ECS

### Change Detection
Bevy's change detection triggers mutation collection.
Results applied deterministically in sorted order.

## Future Extensions

### Price Diffusion
Markets will trade goods to equalize prices, with transport costs affecting
flow direction and magnitude.

### Congestion Modeling
Edge usage increases transport costs:
```text
cost = base_cost * (1 + congestion_coeff * usage)
```

Usage decays over time if no goods flow through edge.

### Storage Modulation
Storage quality modulates decay rates:
```text
effective_decay = base_decay / storage_modifier
```

Can be coupled to terrain fields (climate, infrastructure).

### Terrain Coupling
WorldFieldSampler allows reading terrain data:
- Elevation affects transport costs
- Temperature affects decay rates
- Humidity affects storage quality

*/

#[cfg(doc)]
use crate::prelude::*;
