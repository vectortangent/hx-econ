//! Market state buffers
//!
//! Dense SoA (Structure of Arrays) storage for market data.

use crate::ids::{GoodId, MarketId, RegionId};

/// Market metadata
#[derive(Debug, Clone)]
pub struct MarketMetadata {
    pub market_id: MarketId,
    pub region: RegionId,
    pub population: i32,
    /// Storage quality modifier (fixed-point, 1.0 = normal)
    pub storage_modifier_q: i32,
}

/// Market state for all markets and goods
pub struct MarketState {
    /// Number of markets
    pub market_count: usize,
    /// Number of goods
    pub good_count: usize,

    /// Market metadata (indexed by MarketId)
    pub metadata: Vec<MarketMetadata>,

    /// Stock levels: \[market\]\[good\] flattened
    /// Access: stock[market_id * good_count + good_id]
    pub stock: Vec<i32>,

    /// Prices: \[market\]\[good\] flattened
    /// Access: price[market_id * good_count + good_id]
    pub price: Vec<i32>,
}

impl MarketState {
    /// Create a new market state
    pub fn new(market_count: usize, good_count: usize) -> Self {
        let total_cells = market_count * good_count;
        Self {
            market_count,
            good_count,
            metadata: Vec::with_capacity(market_count),
            stock: vec![0; total_cells],
            price: vec![crate::util::FIXED_ONE; total_cells], // Default price = 1.0
        }
    }

    /// Get stock level for a market-good pair
    #[inline]
    pub fn get_stock(&self, market: MarketId, good: GoodId) -> i32 {
        let idx = market.index() * self.good_count + good.index();
        self.stock[idx]
    }

    /// Set stock level for a market-good pair
    #[inline]
    pub fn set_stock(&mut self, market: MarketId, good: GoodId, value: i32) {
        let idx = market.index() * self.good_count + good.index();
        self.stock[idx] = value;
    }

    /// Add to stock level
    #[inline]
    pub fn add_stock(&mut self, market: MarketId, good: GoodId, delta: i32) {
        let idx = market.index() * self.good_count + good.index();
        self.stock[idx] += delta;
    }

    /// Get price for a market-good pair
    #[inline]
    pub fn get_price(&self, market: MarketId, good: GoodId) -> i32 {
        let idx = market.index() * self.good_count + good.index();
        self.price[idx]
    }

    /// Set price for a market-good pair
    #[inline]
    pub fn set_price(&mut self, market: MarketId, good: GoodId, value: i32) {
        let idx = market.index() * self.good_count + good.index();
        self.price[idx] = value;
    }

    /// Get market metadata
    #[inline]
    pub fn get_metadata(&self, market: MarketId) -> Option<&MarketMetadata> {
        self.metadata.get(market.index())
    }

    /// Add a market
    pub fn add_market(&mut self, region: RegionId, population: i32, storage_modifier_q: i32) {
        let market_id = MarketId::new(self.metadata.len() as u32);
        self.metadata.push(MarketMetadata {
            market_id,
            region,
            population,
            storage_modifier_q,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_state() {
        let mut state = MarketState::new(2, 3);
        state.add_market(RegionId::new(0), 1000, crate::util::FIXED_ONE);
        state.add_market(RegionId::new(1), 2000, crate::util::FIXED_ONE);

        let m0 = MarketId::new(0);
        let g0 = GoodId::new(0);

        assert_eq!(state.get_stock(m0, g0), 0);
        state.set_stock(m0, g0, 100);
        assert_eq!(state.get_stock(m0, g0), 100);

        state.add_stock(m0, g0, 50);
        assert_eq!(state.get_stock(m0, g0), 150);
    }
}
