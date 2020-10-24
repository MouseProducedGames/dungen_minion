// External includes.
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for adding one or more instances of [`Portal`](struct.Portal.html) to the edges of a map.
///
/// The `EdgePortalsGenerator` can be called with an explicit count to add one or more internal `Portal` and [`TileType`](enum.TileType.html)::Portal instances, or with an instance of [`ProvidesCount`](geometry/trait.ProvidesCount.html).
///
/// The portals will be generated randomly on the edge of the map, excluding corners, and are one-way only.
///
/// Will create a map with a `Size` of 8 tiles wide by 6 tiles high, and then generate 5 `Portal` and `TileType::Portal` instances projecting off of it. Each matching `Portal` and `TileType::Portal` instance will be on the same [`LocalPosition`](geometry/struct.LocalPosition.html). Each `Portal` will have an attached MapId which can be edited by calling the appropriate methods with various generators, or manually after generation.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// use rayon::prelude::*;
/// [0..5_000].par_iter().for_each(|_i| {
///     // We could provide CountRange directly to EdgePortalsGenerator, but that would not let us
///     // test that we have the right number of portals.
///     // This CountRange will generate a number in the range [2, 5].
///     let num_portals = CountRange::new(2, 5).provide_count();
///     let map_id =
///         DunGen::new(SparseMap::new())
///         .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
///         .gen_with(WalledRoomGenerator::new(Size::zero()))
///         .gen_with(EdgePortalsGenerator::new(
///             num_portals,
///             // A boxed generator which provides the `MapId`s for the `Map`s that will be placed at
///             // the end of the portal.
///
///             Box::new(|| SparseMap::new())
///         ))
///         .build();
///
///     let maps = MAPS.read();
///     let map = maps[map_id].read();
///
///     assert!(*map.size() == Size::new(8, 6));
///     assert!(map.portal_count() == num_portals);
///     assert!(map.portal_count() >= 2 && map.portal_count() <= 5);
///     let mut portal_count = 0;
///     for portal in map.portals() {
///         let target_map = maps[portal.target()].read();
///         assert!(*target_map.size() == Size::zero());
///         assert!(target_map.tile_type_at_local(Position::new(0, 0)) == None);
///         assert!(target_map.tile_type_at_local(Position::new(1, 1)) == None);
///         portal_count += 1;
///     }
///     assert!(portal_count == num_portals);
///     assert!(portal_count >= 2 && portal_count <= 5);
/// })
/// ```
pub struct EdgePortalsGenerator<TProvidesCount>
where
    TProvidesCount: ProvidesCount + Sized,
{
    provides_count: TProvidesCount,
    placed_map_box_func: Box<dyn Fn() -> MapId>,
}

impl<TProvidesCount> EdgePortalsGenerator<TProvidesCount>
where
    TProvidesCount: ProvidesCount + Sized,
{
    /// Creates a new generator for adding portals to a map.
    pub fn new(
        provides_count: TProvidesCount,
        placed_map_box_func: Box<dyn Fn() -> MapId>,
    ) -> Self {
        Self {
            provides_count,
            placed_map_box_func,
        }
    }
}

impl<TProvidesCount> DoesDunGen for EdgePortalsGenerator<TProvidesCount>
where
    TProvidesCount: ProvidesCount + Sized,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        self.dun_gen_map(map_id);
    }

    fn dun_gen_map(&self, map_id: MapId) {
        let mut data = Vec::<(Position, CardinalDirection)>::new();
        {
            let maps = &MAPS.read();
            let map = &mut maps[map_id].write();
            // Convenience.
            let area = *map.area();
            if area.width() < 3 || area.height() < 3 {
                return;
            }

            let mut edge_tiles = Vec::new();
            for x in (map.left() + 1)..map.right() {
                let position = Position::new(x, 0);
                if map.contains_position(position) == Containment::Intersects {
                    edge_tiles.push(position);
                }
            }
            for y in (map.top() + 1)..map.bottom() {
                {
                    let position = Position::new(map.left(), y);
                    if map.contains_position(position) == Containment::Intersects {
                        edge_tiles.push(position);
                    }
                }

                {
                    let position = Position::new(map.right(), y);
                    if map.contains_position(position) == Containment::Intersects {
                        edge_tiles.push(position);
                    }
                }
            }
            for x in (map.left() + 1)..map.right() {
                let position = Position::new(x, map.bottom());
                if map.contains_position(position) == Containment::Intersects {
                    edge_tiles.push(position);
                }
            }

            let count = self.provides_count.provide_count();
            let mut rng = thread_rng();
            for _ in 0..count {
                let index = rng.gen_range(0, edge_tiles.len());
                let edge_portal_position = edge_tiles[index];
                edge_tiles.truncate(edge_tiles.len() - 1);
                data.push((
                    edge_portal_position,
                    if edge_portal_position.x() == map.left() {
                        CardinalDirection::East
                    } else if edge_portal_position.x() == map.right() {
                        CardinalDirection::West
                    } else if edge_portal_position.y() == map.top() {
                        CardinalDirection::South
                    } else {
                        CardinalDirection::North
                    },
                ));
            }
        }

        let data = data
            .iter()
            .map(|(local_position, portal_to_map_facing)| {
                (
                    local_position,
                    portal_to_map_facing,
                    (self.placed_map_box_func)(),
                )
            })
            .collect::<Vec<_>>();

        {
            let maps = &MAPS.read();
            let map = &mut maps[map_id].write();
            for data in data {
                map.add_portal(*data.0, *data.1, Position::zero(), data.2);
            }
        }
    }
}
