use crate::Iso;
use std::collections::{HashMap, HashSet};

/// Computes a field of movement around `tile` given a `budget`.
/// This algorithm takes a `cost` function, which calculates and
/// returns the cost of movement through a given `Iso` tile.
/// The `cost` function should return an `Option<u32>`.
/// A tile that returns a computable cost would return `Some(cost)`, whereas
/// `None` should be returned for tiles that have no computable cost (i.e.
/// cannot be moved through).
///
/// The `field_of_movement` algorithm will always add `+ 1` to the computed cost
/// in order to avoid the possibility of unlimited movement range (i.e. a `Iso`
/// instance will always have a minimum movement `cost` of 1).
///
/// # Warning
///
/// The implementation of this function is pretty naive and has a high
/// complexity. It is not suitable for production use
///
/// # Examples
///
/// - Compute field of movement with no boundaries and some wall tiles that
///   cannot be traversed
///
/// ```rust
/// # use isometric::*;
/// # use std::collections::HashMap;
/// # use isometric::algorithms::fom;
///
/// enum Biome {
///     Mountain,
///     Plains,
///     Forest,
///     Desert,
/// }
///
/// impl Biome {
///     pub fn cost(&self) -> Option<u32> {
///         match self {
///             Self::Mountain => None, // Moutains cannot be traversed
///             Self::Plains => Some(0),
///             Self::Forest => Some(1),
///             Self::Desert => Some(2),
///         }
///     }
/// }
///
/// let start = iso(0, 0);
/// let movement_budget = 5u32;
/// let mut biomes: HashMap<Iso, Biome> = HashMap::new();
/// // Set tileinate biomes
/// // biomes.insert(hex(1, 2), Biome::Mountain);
/// // ..
/// let reachable_tiles = field_of_movement(start, movement_budget, |h| {
///     biomes.get(&h).and_then(|b| b.cost())
/// });
/// ```
pub fn field_of_movement(
    tile: Iso,
    budget: u32,
    cost: impl Fn(Iso) -> Option<u32>,
) -> HashSet<Iso> {
    let mut computed_costs = HashMap::with_capacity(Iso::range_count(budget) as usize);
    computed_costs.insert(tile, 0);

    // We cache the rings and costs
    let rings: Vec<(Iso, u32)> = tile
        .rings(1..=budget)
        .flatten()
        .filter_map(|h| cost(h).map(|c| (h, c)))
        .collect();

    let mut loop_again = true;
    while loop_again {
        loop_again = false;
        for (tile, tile_cost) in &rings {
            let Some(neighbor_cost) = tile
                .all_neighbors()
                .into_iter()
                .filter_map(|n| computed_costs.get(&n))
                .min()
            else {
                continue;
            };
            let computed_cost = tile_cost + 1 + neighbor_cost;
            let res = computed_costs.insert(*tile, computed_cost);
            if !loop_again && (res.is_none() || res != Some(computed_cost)) {
                loop_again = true;
            }
        }
    }
    computed_costs
        .into_iter()
        .filter_map(|(tile, cost)| (cost <= budget).then_some(tile))
        .collect()
}
