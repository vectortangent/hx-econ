//! Transport graph data structures
//!
//! Edges are stored sorted by (RegionId, from_market, to_market, EdgeId)
//! for efficient region-scoped iteration.

use crate::ids::{EdgeId, MarketId, RegionId};

/// A transport edge connecting two markets
#[derive(Debug, Clone, Copy)]
pub struct TransportEdge {
    pub edge_id: EdgeId,
    pub from_market: MarketId,
    pub to_market: MarketId,
    pub region: RegionId,
    /// Base cost in fixed-point (distance, terrain difficulty, etc.)
    pub base_cost_q: i32,
    /// Current usage level (affects congestion)
    pub usage_q: i32,
    /// Congestion coefficient (higher = more sensitive to usage)
    pub congestion_coeff_q: i32,
}

impl TransportEdge {
    /// Calculate effective cost including congestion
    pub fn effective_cost(&self) -> i32 {
        use crate::util::{fixed_mul, FIXED_ONE};
        // cost = base_cost * (1 + congestion_coeff * usage)
        let congestion_factor = FIXED_ONE + fixed_mul(self.congestion_coeff_q, self.usage_q);
        fixed_mul(self.base_cost_q, congestion_factor)
    }
}

/// Transport graph with edges stored in sorted order
pub struct TransportGraph {
    /// All edges, sorted by (region, from, to, edge_id)
    pub edges: Vec<TransportEdge>,

    /// Range of edges for each region: region_ranges[region_id] = (start, end)
    /// Edges for region R are edges[start..end]
    pub region_ranges: Vec<(usize, usize)>,
}

impl TransportGraph {
    /// Create a new empty transport graph
    pub fn new(num_regions: usize) -> Self {
        Self {
            edges: Vec::new(),
            region_ranges: vec![(0, 0); num_regions],
        }
    }

    /// Get edges for a specific region
    pub fn edges_for_region(&self, region: RegionId) -> &[TransportEdge] {
        let idx = region.index();
        if idx < self.region_ranges.len() {
            let (start, end) = self.region_ranges[idx];
            &self.edges[start..end]
        } else {
            &[]
        }
    }

    /// Get mutable edges for a specific region
    pub fn edges_for_region_mut(&mut self, region: RegionId) -> &mut [TransportEdge] {
        let idx = region.index();
        if idx < self.region_ranges.len() {
            let (start, end) = self.region_ranges[idx];
            &mut self.edges[start..end]
        } else {
            &mut []
        }
    }

    /// Get total number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

/// Builder for constructing a transport graph
pub struct TransportGraphBuilder {
    edges: Vec<TransportEdge>,
    num_regions: usize,
}

impl TransportGraphBuilder {
    pub fn new(num_regions: usize) -> Self {
        Self {
            edges: Vec::new(),
            num_regions,
        }
    }

    /// Add an edge to the graph
    pub fn add_edge(
        &mut self,
        edge_id: EdgeId,
        from: MarketId,
        to: MarketId,
        region: RegionId,
        base_cost_q: i32,
        congestion_coeff_q: i32,
    ) {
        self.edges.push(TransportEdge {
            edge_id,
            from_market: from,
            to_market: to,
            region,
            base_cost_q,
            usage_q: 0,
            congestion_coeff_q,
        });
    }

    /// Build the transport graph, sorting edges by region
    pub fn build(mut self) -> TransportGraph {
        // Sort edges by (region, from, to, edge_id) for deterministic order
        self.edges.sort_by_key(|e| {
            (e.region.0, e.from_market.0, e.to_market.0, e.edge_id.0)
        });

        // Build region ranges
        let mut region_ranges = vec![(0, 0); self.num_regions];
        let mut current_region = None;
        let mut start = 0;

        for (i, edge) in self.edges.iter().enumerate() {
            let region_idx = edge.region.index();
            if current_region != Some(region_idx) {
                if let Some(prev_region) = current_region {
                    region_ranges[prev_region] = (start, i);
                }
                current_region = Some(region_idx);
                start = i;
            }
        }

        // Handle last region
        if let Some(region) = current_region {
            region_ranges[region] = (start, self.edges.len());
        }

        TransportGraph {
            edges: self.edges,
            region_ranges,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_graph_builder() {
        let mut builder = TransportGraphBuilder::new(2);
        builder.add_edge(
            EdgeId::new(0),
            MarketId::new(0),
            MarketId::new(1),
            RegionId::new(0),
            1000,
            100,
        );
        builder.add_edge(
            EdgeId::new(1),
            MarketId::new(2),
            MarketId::new(3),
            RegionId::new(1),
            2000,
            200,
        );

        let graph = builder.build();
        assert_eq!(graph.edge_count(), 2);
        assert_eq!(graph.edges_for_region(RegionId::new(0)).len(), 1);
        assert_eq!(graph.edges_for_region(RegionId::new(1)).len(), 1);
    }

    #[test]
    fn test_effective_cost() {
        use crate::util::FIXED_ONE;
        let edge = TransportEdge {
            edge_id: EdgeId::new(0),
            from_market: MarketId::new(0),
            to_market: MarketId::new(1),
            region: RegionId::new(0),
            base_cost_q: FIXED_ONE * 100,
            usage_q: FIXED_ONE / 2, // 50% usage
            congestion_coeff_q: FIXED_ONE, // 1.0 coefficient
        };

        let cost = edge.effective_cost();
        // cost = 100 * (1 + 1.0 * 0.5) = 100 * 1.5 = 150
        let expected = FIXED_ONE * 150;
        assert!((cost - expected).abs() < 1000); // Allow small rounding error
    }
}
