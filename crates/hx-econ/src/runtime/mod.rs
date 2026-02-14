//! Economy runtime and simulation step logic

use crate::ids::{GoodId, MarketId};
use crate::pack::GoodsRegistry;
use crate::transport::TransportGraph;
use crate::util::{apply_decay, clamp, fixed_mul, quantize, BitSet, FIXED_ONE};

pub mod state;

#[cfg(test)]
mod tests;

pub use state::{MarketMetadata, MarketState};

/// Economy runtime state and simulation
pub struct EconomyRuntime {
    /// Simulation seed for determinism
    pub seed: u64,
    /// Current tick counter
    pub tick: u64,

    /// Market state
    pub market_state: MarketState,
    /// Transport graph
    pub transport_graph: TransportGraph,
    /// Goods configuration
    pub goods_registry: GoodsRegistry,

    /// Dirty set for markets that need recomputation
    pub dirty_markets: BitSet,
    /// Dirty set for edges that need recomputation
    pub dirty_edges: BitSet,
}

impl EconomyRuntime {
    /// Create a new economy runtime
    pub fn new(
        seed: u64,
        market_count: usize,
        good_count: usize,
        region_count: usize,
        goods_registry: GoodsRegistry,
    ) -> Self {
        Self {
            seed,
            tick: 0,
            market_state: MarketState::new(market_count, good_count),
            transport_graph: TransportGraph::new(region_count),
            goods_registry,
            dirty_markets: BitSet::new(market_count),
            dirty_edges: BitSet::new(0), // Will be resized when edges are added
        }
    }

    /// Perform one simulation step
    pub fn step(&mut self, dt: i32) {
        // Apply storage decay to all markets
        self.apply_storage_decay(dt);

        // Update prices based on supply/demand
        self.update_prices();

        // Quantize values to prevent drift
        self.quantize_state();

        // Increment tick counter
        self.tick += 1;

        // Clear dirty sets
        self.dirty_markets.clear_all();
        self.dirty_edges.clear_all();
    }

    /// Apply storage decay to all goods in all markets
    fn apply_storage_decay(&mut self, dt: i32) {
        let market_count = self.market_state.market_count;
        let good_count = self.market_state.good_count;

        for market_idx in 0..market_count {
            let market_id = MarketId::new(market_idx as u32);
            let metadata = &self.market_state.metadata[market_idx];
            let _storage_modifier = metadata.storage_modifier_q;

            for good_idx in 0..good_count {
                let good_id = GoodId::new(good_idx as u16);
                let good_config = &self.goods_registry.goods[good_idx];

                // Calculate effective decay rate
                // decay = base_decay * dt * (1 / storage_modifier)
                let base_decay = good_config.base_decay_q;
                let decay_rate = fixed_mul(base_decay, dt);
                let decay_rate = fixed_mul(decay_rate, FIXED_ONE); // Could modulate by storage_modifier

                // Apply decay
                let current_stock = self.market_state.get_stock(market_id, good_id);
                let new_stock = apply_decay(current_stock, decay_rate);
                self.market_state.set_stock(market_id, good_id, new_stock);
            }
        }
    }

    /// Update prices based on stock levels vs target stock
    fn update_prices(&mut self) {
        let market_count = self.market_state.market_count;
        let good_count = self.market_state.good_count;

        for market_idx in 0..market_count {
            let market_id = MarketId::new(market_idx as u32);
            let metadata = &self.market_state.metadata[market_idx];
            let population = metadata.population;

            for good_idx in 0..good_count {
                let good_id = GoodId::new(good_idx as u16);
                let good_config = &self.goods_registry.goods[good_idx];

                let current_stock = self.market_state.get_stock(market_id, good_id);
                let current_price = self.market_state.get_price(market_id, good_id);

                // Calculate target stock for this market
                let target_stock = fixed_mul(good_config.target_stock_per_capita_q, population);

                // Calculate scarcity: (target - actual) / target
                let shortage = target_stock - current_stock;
                let scarcity = if target_stock > 0 {
                    fixed_mul(shortage, FIXED_ONE) / target_stock
                } else {
                    0
                };

                // Update price: price += alpha * scarcity
                let price_delta = fixed_mul(good_config.scarcity_alpha_q, scarcity);
                let new_price = current_price + price_delta;

                // Clamp price to reasonable range [0.1, 10.0]
                let min_price = FIXED_ONE / 10;
                let max_price = FIXED_ONE * 10;
                let clamped_price = clamp(new_price, min_price, max_price);

                self.market_state.set_price(market_id, good_id, clamped_price);
            }
        }
    }

    /// Quantize all state values to prevent drift
    fn quantize_state(&mut self) {
        let market_count = self.market_state.market_count;
        let good_count = self.market_state.good_count;

        for market_idx in 0..market_count {
            let market_id = MarketId::new(market_idx as u32);
            for good_idx in 0..good_count {
                let good_id = GoodId::new(good_idx as u16);

                let stock = self.market_state.get_stock(market_id, good_id);
                self.market_state
                    .set_stock(market_id, good_id, quantize(stock));

                let price = self.market_state.get_price(market_id, good_id);
                self.market_state
                    .set_price(market_id, good_id, quantize(price));
            }
        }
    }

    /// Mark a market as dirty (needs recomputation)
    pub fn mark_market_dirty(&mut self, market: MarketId) {
        self.dirty_markets.set(market.index());
    }

    /// Mark an edge as dirty (needs recomputation)
    pub fn mark_edge_dirty(&mut self, edge_idx: usize) {
        self.dirty_edges.set(edge_idx);
    }
}
