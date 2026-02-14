//! Dense ID types for SIMD-friendly indexing
//!
//! All IDs are dense, meaning they occupy a contiguous range [0..count).
//! This enables efficient array-based lookups and SIMD operations.

use std::fmt;

/// Market identifier (dense, 0..market_count)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MarketId(pub u32);

impl MarketId {
    #[inline]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for MarketId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Market({})", self.0)
    }
}

/// Transport edge identifier (dense, 0..edge_count)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeId(pub u32);

impl EdgeId {
    #[inline]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Edge({})", self.0)
    }
}

/// Good/commodity identifier (dense, 0..good_count)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GoodId(pub u16);

impl GoodId {
    #[inline]
    pub fn new(id: u16) -> Self {
        Self(id)
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for GoodId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Good({})", self.0)
    }
}

/// Region identifier (dense, 0..region_count)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionId(pub u32);

impl RegionId {
    #[inline]
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for RegionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Region({})", self.0)
    }
}

/// Faction identifier (dense, 0..faction_count)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactionId(pub u16);

impl FactionId {
    #[inline]
    pub fn new(id: u16) -> Self {
        Self(id)
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for FactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Faction({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_id() {
        let id = MarketId::new(42);
        assert_eq!(id.0, 42);
        assert_eq!(id.index(), 42);
    }

    #[test]
    fn test_id_ordering() {
        let id1 = MarketId::new(1);
        let id2 = MarketId::new(2);
        assert!(id1 < id2);
    }

    #[test]
    fn test_good_id() {
        let id = GoodId::new(10);
        assert_eq!(id.0, 10);
        assert_eq!(id.index(), 10);
    }
}
