// External includes.
use super::*;

// Standard includes.
use std::collections::{HashSet, VecDeque};
use std::sync::RwLock;

// Internal includes.

/// Merges the maps connected to this map through [`Portal`](struct.Portal.html)s as sub-maps of this map.
///
/// The following code will generate some rooms and hallways connected through portals, and then merge them as sub-maps of the central map.
///
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// use rayon::prelude::*;
/// [0..1_000].par_iter().for_each(|_i| {
///     // We could provide CountRange directly to EdgePortalsGenerator, but that would not let us
///     // test that we have the right number of portals.
///     // This CountRange will generate a number in the range [2, 5].
///     let num_portals = CountRange::new(2, 5).provide_count();
///     let map_id =
///         DunGen::new(SparseMap::new())
///         .gen_with(SequentialGenerator::new(&[
///             &EmptyRoomGenerator::new(Size::new(12, 8)),
///             &WalledRoomGenerator::new(Size::zero()),
///             &EdgePortalsGenerator::new(
///                 num_portals,
///                 // A boxed generator which provides the `MapId`'s for the `Map`'s that will be
///                 // placed at the end of the portal.
///                 Box::new(|| SparseMap::new())
///             ),
///         ]))
///         .gen_with(TraversePortalsGenerator::new(SequentialGenerator::new(&[
///             &EmptyRoomGenerator::new(Size::new(8, 6)),
///             &WalledRoomGenerator::new(Size::zero()),
///         ])))
///         .gen_with(TraverseThisAndPortalsGenerator::new(ReciprocatePortalsGenerator::new()))
///         .gen_with(MergePortalMapsAsSubMapsGenerator::new(1, |portal| true))
///         .build();
///
///     let maps = MAPS.read();
///     let map = maps[map_id].read();
///
///     assert!(map.portal_count() == num_portals);
///     assert!(map.portal_count() >= 2 && map.portal_count() <= 5);
///     let mut portal_count = 0;
///     for portal in map.portals() {
///         let target_map = maps[portal.target()].read();
///         assert!(*target_map.size() == Size::new(8, 6));
///         assert!(target_map.tile_type_at_local(Position::new(0, 0)) == Some(TileType::Wall));
///         assert!(target_map.tile_type_at_local(Position::new(1, 1)) == Some(TileType::Floor));
///         let target_local_position = *portal.local_position();
///         let target_other_position = *portal.portal_to_map_position();
///         println!("Target map portal count: {}", target_map.portal_count());
///         assert!(target_map.portal_count() == 1);
///         let mut other_portal_count = 0;
///         for other_portal in target_map.portals() {
///             let other_local_position = *other_portal.local_position();
///             let other_target_position = *other_portal.portal_to_map_position();
///             assert!(other_local_position == target_other_position);
///             assert!(target_local_position == other_target_position);
///             other_portal_count += 1;
///         }
///         assert!(other_portal_count == 1);
///         portal_count += 1;
///     }
///     assert!(portal_count == num_portals);
///     assert!(portal_count >= 2 && portal_count <= 5);
/// })
/// ```
pub struct MergePortalMapsAsSubMapsGenerator<TPortalFilter>
where
    TPortalFilter: Fn(&Portal) -> bool,
{
    portal_filter: TPortalFilter,
    recursion_depth: usize,
    visited: RwLock<HashSet<MapId>>,
}

impl<TPortalFilter> MergePortalMapsAsSubMapsGenerator<TPortalFilter>
where
    TPortalFilter: Fn(&Portal) -> bool,
{
    /// Creates a new MergePortalMapsAsSubMapsGenerator.
    pub fn new(recursion_depth: usize, portal_filter: TPortalFilter) -> Self {
        Self {
            portal_filter,
            recursion_depth,
            visited: RwLock::new(HashSet::new()),
        }
    }
}

impl<TPortalFilter> DoesDunGen for MergePortalMapsAsSubMapsGenerator<TPortalFilter>
where
    TPortalFilter: Fn(&Portal) -> bool,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        self.dun_gen_map(map_id);
    }

    fn dun_gen_map(&self, map_id: MapId) {
        let mut visited = self.visited.write().unwrap();
        if visited.contains(&map_id) {
            return;
        }
        visited.insert(map_id);

        if self.recursion_depth == 0 {
            return;
        }

        let recursion_depth = self.recursion_depth;
        let maps = &MAPS.read();
        let map = &mut maps[map_id].write();

        let mut on_maps = VecDeque::new();
        let mut positions_map_ids = Vec::new();
        {
            for portal in map.portals() {
                if (self.portal_filter)(portal) {
                    let portal_map_id = portal.target();

                    if visited.contains(&portal_map_id) {
                        continue;
                    }
                    visited.insert(portal_map_id);

                    let portal_map_position =
                        *portal.local_position() - (*portal.portal_to_map_position());

                    positions_map_ids.push((portal_map_position, portal_map_id));
                }
            }

            for (portal_map_position, portal_map_id) in positions_map_ids.iter() {
                map.add_sub_map(*portal_map_position, *portal_map_id);
                if recursion_depth > 1 {
                    on_maps.push_back((*portal_map_position, recursion_depth - 1, *portal_map_id));
                }
            }
        }

        if recursion_depth == 1 {
            return;
        }

        while !on_maps.is_empty() {
            let (accumulated_position, recursion_depth, target_map_id) =
                on_maps.pop_front().unwrap();

            if recursion_depth == 0 {
                continue;
            }

            positions_map_ids.clear();
            let target_map = &maps[target_map_id].read();
            for portal in target_map.portals() {
                if (self.portal_filter)(portal) {
                    let portal_map_id = portal.target();

                    if visited.contains(&portal_map_id) {
                        continue;
                    }
                    visited.insert(portal_map_id);

                    let portal_map_position =
                        *portal.local_position() - (*portal.portal_to_map_position());

                    positions_map_ids.push((portal_map_position, portal_map_id));
                }
            }

            for (portal_map_position, portal_map_id) in positions_map_ids.iter() {
                map.add_sub_map(accumulated_position + *portal_map_position, *portal_map_id);
                on_maps.push_back((
                    accumulated_position + *portal_map_position,
                    recursion_depth - 1,
                    *portal_map_id,
                ));
            }
        }
    }
}
