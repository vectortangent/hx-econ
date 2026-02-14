//! # hx-econ
//!
//! Deterministic price-diffusion economy simulation with Bevy ECS integration.
//!
//! ## Features
//!
//! - **Deterministic**: Results are reproducible given the same seed and inputs
//! - **Incremental**: Only recomputes dirty regions/edges/markets
//! - **Price-diffusion**: Market prices respond to supply/demand imbalances
//! - **Congestion**: Transport costs increase with usage
//! - **Storage decay**: Goods spoil over time
//! - **Terrain coupling**: Economic parameters can be modulated by terrain fields
//! - **Bevy ECS**: Full integration with Bevy for change detection and orchestration
//!
//! ## Core Concepts
//!
//! - **Markets**: Locations where goods are stored, priced, and traded
//! - **Goods**: Commodities with configurable properties (bulk, decay, target stock)
//! - **Transport Edges**: Connections between markets with congestion modeling
//! - **Regions**: Spatial grouping for efficient computation
//!
//! ## Example
//!
//! ```rust,no_run
//! use hx_econ::prelude::*;
//! use bevy::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(MinimalPlugins)
//!         .add_plugins(EconomyPlugin {
//!             market_count: 10,
//!             good_count: 5,
//!             region_count: 2,
//!             goods_registry: GoodsRegistry::default(),
//!             config: EconomyConfig {
//!                 seed: 12345,
//!                 dt: FIXED_ONE,
//!                 auto_step: true,
//!             },
//!         })
//!         .run();
//! }
//! ```

// Module declarations
pub mod ids;
pub mod pack;
pub mod fields;
pub mod transport;
pub mod runtime;
pub mod util;

// Bevy integration (feature-gated to avoid forcing Bevy dependency)
#[cfg(feature = "bevy")]
pub mod bevy;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::ids::*;
    pub use crate::pack::*;
    pub use crate::fields::*;
    pub use crate::transport::*;
    pub use crate::runtime::*;
    pub use crate::util::*;

    #[cfg(feature = "bevy")]
    pub use crate::bevy::*;
}
