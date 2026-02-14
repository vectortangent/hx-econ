//! Good configuration and metadata

use serde::{Deserialize, Serialize};

/// Good metadata loaded from TOML configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodConfig {
    /// Human-readable name
    pub name: String,

    /// Congestion weight (bulk per unit, fixed-point)
    /// Higher values contribute more to transport congestion
    pub bulk_q: i32,

    /// Base storage decay rate per tick (fixed-point [0..1])
    /// 0 = no decay, FIXED_ONE = complete decay
    pub base_decay_q: i32,

    /// Target stock level per capita (fixed-point)
    pub target_stock_per_capita_q: i32,

    /// Price response strength to scarcity (fixed-point)
    /// Higher values = prices respond more to supply/demand imbalance
    pub scarcity_alpha_q: i32,

    /// Optional: additional decay during transit (not used initially)
    #[serde(default)]
    pub transit_decay_q: Option<i32>,
}

impl Default for GoodConfig {
    fn default() -> Self {
        Self {
            name: String::from("unknown"),
            bulk_q: crate::util::FIXED_ONE,
            base_decay_q: 0,
            target_stock_per_capita_q: crate::util::FIXED_ONE,
            scarcity_alpha_q: crate::util::FIXED_ONE,
            transit_decay_q: None,
        }
    }
}

/// Registry of all goods in the economy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GoodsRegistry {
    pub goods: Vec<GoodConfig>,
}

impl GoodsRegistry {
    /// Load goods from TOML string
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Get number of goods
    pub fn len(&self) -> usize {
        self.goods.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.goods.is_empty()
    }

    /// Get good config by index
    pub fn get(&self, index: usize) -> Option<&GoodConfig> {
        self.goods.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_goods_toml() {
        let toml_str = r#"
[[goods]]
name = "food"
bulk_q = 65536
base_decay_q = 6553
target_stock_per_capita_q = 131072
scarcity_alpha_q = 32768

[[goods]]
name = "wood"
bulk_q = 131072
base_decay_q = 0
target_stock_per_capita_q = 65536
scarcity_alpha_q = 16384
"#;

        let registry = GoodsRegistry::from_toml(toml_str).unwrap();
        assert_eq!(registry.len(), 2);
        assert_eq!(registry.goods[0].name, "food");
        assert_eq!(registry.goods[1].name, "wood");
    }
}
