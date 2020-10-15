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
/// The portals will be generated randomly on the edge of the room, excluding corners, and are one-way only.
///
/// Will create a room with a `Size` of 8 tiles wide by 6 tiles high, and then generate 5 `Portal` and `TileType::Portal` instances projecting off of it. Each matching `Portal` and `TileType::Portal` instance will be on the same [`LocalPosition`](geometry/struct.LocalPosition.html). Each `Portal` will have an attached Box<dyn [`PlacedRoom`](trait.PlacedRoom.html)> which can be edited by calling the appropriate methods with various generators, or manually after generation.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// for _ in 0..5_000 {
///     // We could provide CountRange directly to EdgePortalsGenerator, but that would not let us
///     // test that we have the right number of portals.
///     // This CountRange will generate a number in the range [2, 5].
///     let num_portals = CountRange::new(2, 5).provide_count();
///     let map_id =
///         DunGen::new(MapSparse::new())
///         .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
///         .gen_with(WalledRoomGenerator::new(Size::zero()))
///         .gen_with(EdgePortalsGenerator::new(
///             num_portals,
///             // A boxed generator which provides the `MapId`s for the `Map`s that will be placed at
///             // the end of the portal.
///
///             Box::new(|| MapSparse::new())
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
/// }
/// ```
pub struct EdgePortalsGenerator<TProvidesCount>
where
    TProvidesCount: ProvidesCount + Sized,
{
    provides_count: TProvidesCount,
    placed_room_box_func: Box<dyn Fn() -> MapId>,
}

impl<TProvidesCount> EdgePortalsGenerator<TProvidesCount>
where
    TProvidesCount: ProvidesCount + Sized,
{
    /// Creates a new generator for adding portals to a room.
    pub fn new(
        provides_count: TProvidesCount,
        placed_room_box_func: Box<dyn Fn() -> MapId>,
    ) -> Self {
        Self {
            provides_count,
            placed_room_box_func,
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
        let mut data = Vec::<(Position, OrdinalDirection)>::new();
        {
            let maps = &MAPS.read();
            let map = &mut maps[map_id].write();
            // Convenience.
            let area = *map.area();
            if area.width() < 3 || area.height() < 3 {
                return;
            }

            let count = self.provides_count.provide_count();
            let mut rng = thread_rng();
            for _ in 0..count {
                let total_odds = area.height() as f64 + area.width() as f64;
                let on_vertical_wall = rng.gen_bool(area.height() as f64 / total_odds);
                data.push(if on_vertical_wall {
                    let portal_y = rng.gen_range(1, area.bottom()) as i32;
                    let on_left_wall = rng.gen_bool(0.5);
                    if on_left_wall {
                        (Position::new(0, portal_y), OrdinalDirection::East)
                    } else {
                        (
                            Position::new(area.right(), portal_y),
                            OrdinalDirection::West,
                        )
                    }
                } else {
                    let portal_x = rng.gen_range(1, area.width() - 1) as i32;
                    let on_top_wall = rng.gen_bool(0.5);
                    if on_top_wall {
                        (Position::new(portal_x, 0), OrdinalDirection::South)
                    } else {
                        (
                            Position::new(portal_x, area.bottom()),
                            OrdinalDirection::North,
                        )
                    }
                });
            }
        }

        let data = data
            .iter()
            .map(|(local_position, portal_to_room_facing)| {
                (
                    local_position,
                    portal_to_room_facing,
                    (self.placed_room_box_func)(),
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
