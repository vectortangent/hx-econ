//! Bevy events for economy system

use bevy::prelude::*;
use crate::ids::{GoodId, MarketId};

/// Event fired when a market's stock changes
#[derive(Event, Clone)]
pub struct StockChangeEvent {
    pub market: MarketId,
    pub good: GoodId,
    pub old_stock: i32,
    pub new_stock: i32,
}

/// Event fired when a market's price changes
#[derive(Event, Clone)]
pub struct PriceChangeEvent {
    pub market: MarketId,
    pub good: GoodId,
    pub old_price: i32,
    pub new_price: i32,
}

/// Event fired when economy simulation step completes
#[derive(Event, Clone)]
pub struct SimulationStepEvent {
    pub tick: u64,
}
