// External includes.
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for iterating through the [`Portal`](struct.Portal.html)s on a [`Map`](trait.Map.html), and creating one or more linking `Portal`s on the target map, if they do not exist.
///
/// The `ReciprocatePortalsGenerator` does not take input in its [`ReciprocatePortalsGenerator::new()`](#method.new) method.
///
/// The portals will be generated randomly on the edge of the map, excluding corners, and are one-way only.
///
/// Will create a map with a `Size` of 12 tiles wide by 8 tiles high, and then generate 5 `Portal` and `TileType::Portal` instances projecting off of it. The `Map`s at the ends of the `Portal`s will be expanded to 8 tiles wide by 6 tiles high, and matching portals back to the main room will be generated.
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
///         .build();
///
///     let maps = MAPS.read();
///     let map = maps[map_id].read();
///
///     assert!(*map.size() == Size::new(12, 8));
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
pub struct ReciprocatePortalsGenerator {}

impl ReciprocatePortalsGenerator {
    /// Creates a new generator for adding portals to a map.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl DoesDunGen for ReciprocatePortalsGenerator {
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        self.dun_gen_map(map_id);
    }

    fn dun_gen_map(&self, map_id: MapId) {
        let maps = &MAPS.read();
        let map = &mut maps[map_id].write();

        // Convenience.
        let size = *map.size();
        if size.width() < 3 || size.height() < 3 {
            return;
        }

        for portal_mut in map.portals_mut() {
            let target_map_id = portal_mut.target();
            let target_map_mut = &mut maps[target_map_id].write();
            let target_map_size = *target_map_mut.size();
            if target_map_size.width() < 3 || target_map_size.height() < 3 {
                return;
            }

            let mut found_match = false;
            for other_portal in target_map_mut.portals() {
                if portal_mut.local_position() == other_portal.portal_to_map_position() {
                    found_match = true;
                }
            }

            if !found_match {
                let mut rng = thread_rng();
                let portal_facing = *portal_mut.portal_to_map_facing();
                let (portal_x, portal_y) = match portal_facing {
                    CardinalDirection::North => {
                        (rng.gen_range(1, target_map_size.width() - 1) as i32, 0)
                    }
                    CardinalDirection::East => (
                        target_map_size.width() as i32 - 1,
                        rng.gen_range(1, target_map_size.height() - 1) as i32,
                    ),
                    CardinalDirection::South => (
                        rng.gen_range(1, target_map_size.width() - 1) as i32,
                        target_map_size.height() as i32 - 1,
                    ),
                    CardinalDirection::West => {
                        (0, rng.gen_range(1, target_map_size.height() - 1) as i32)
                    }
                };
                let target_local_position = Position::new(portal_x, portal_y);
                target_map_mut.add_portal(
                    target_local_position,
                    -portal_facing,
                    *portal_mut.local_position(),
                    map_id,
                );
                *portal_mut.portal_to_map_position_mut() = target_local_position;
            }
        }
    }
}
