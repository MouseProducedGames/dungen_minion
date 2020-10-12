// External includes.
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for adding one or more instances of [`Portal`](struct.Portal.html) to a room.
///
/// The `EdgePortalsGenerator` **cannot** be called statically, but can be called with an explicit count to add one or more internal `Portal` and [`TileType`](enum.TileType.html)::Portal instances.
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
///     let map =
///         DunGen::new(Box::new(RoomHashMap::new()))
///         .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
///         .gen::<WalledRoomGenerator::<Size>>()
///         .gen_with(EdgePortalsGenerator::new(
///             num_portals,
///             // A boxed generator which provides the boxed `PlacedRoom`s that will be placed at
///             // the end of the portal.
///
///             Box::new(|| {
///                 Box::new(PlacedRoomWrapper::new(
///                     Position::new(0, 0),
///                     RoomHashMap::default(),
///                 ))
///             })
///         ))
///         .build();
///
///     assert!(*map.size() == Size::new(8, 6));
///     assert!(map.portal_count() == num_portals);
///     assert!(map.portal_count() >= 2 && map.portal_count() <= 5);
///     let mut portal_count = 0;
///     for portal in map.portals() {
///         assert!(*portal.target().size() == Size::zero());
///         assert!(
///             portal.target().tile_type_at_local(
///                 ShapePosition::new(0, 0)
///             ) == None);
///         assert!(
///             portal.target().tile_type_at_local(
///                 ShapePosition::new(1, 1)
///             ) == None);
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
    placed_room_box_func: Box<dyn Fn() -> Box<dyn PlacedRoom>>,
}

impl<TProvidesCount> EdgePortalsGenerator<TProvidesCount>
where
    TProvidesCount: ProvidesCount + Sized,
{
    /// Creates a new generator for adding portals to a room.
    pub fn new(
        provides_count: TProvidesCount,
        placed_room_box_func: Box<dyn Fn() -> Box<dyn PlacedRoom>>,
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
        // Convenience.
        let map = target.get_map_mut();
        let size = *map.size();
        if size.width() < 3 || size.height() < 3 {
            return;
        }

        self.dun_gen_map(map);
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        // Convenience.
        let size = *map.size();
        if size.width() < 3 || size.height() < 3 {
            return;
        }

        let count = self.provides_count.provide_count();
        let mut rng = thread_rng();
        for _ in 0..count {
            let total_odds = size.height() as f64 + size.width() as f64;
            let on_vertical_wall = rng.gen_bool(size.height() as f64 / total_odds);
            if on_vertical_wall {
                let portal_y = rng.gen_range(1, size.height() - 1) as i32;
                let on_left_wall = rng.gen_bool(0.5);
                if on_left_wall {
                    map.add_portal(
                        ShapePosition::new(0, portal_y),
                        OrdinalDirection::East,
                        (self.placed_room_box_func)(),
                    );
                } else {
                    map.add_portal(
                        ShapePosition::new(size.width() as i32 - 1, portal_y),
                        OrdinalDirection::West,
                        (self.placed_room_box_func)(),
                    );
                }
            } else {
                let portal_x = rng.gen_range(1, size.width() - 1) as i32;
                let on_top_wall = rng.gen_bool(0.5);
                if on_top_wall {
                    map.add_portal(
                        ShapePosition::new(portal_x, 0),
                        OrdinalDirection::South,
                        (self.placed_room_box_func)(),
                    );
                } else {
                    map.add_portal(
                        ShapePosition::new(portal_x, size.height() as i32 - 1),
                        OrdinalDirection::North,
                        (self.placed_room_box_func)(),
                    );
                }
            }
        }
    }
}

impl<TProvidesCount> DoesDunGenPlaced for EdgePortalsGenerator<TProvidesCount>
where
    TProvidesCount: ProvidesCount + Sized,
{
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced) {
        // Convenience.
        let map = target.get_placed_map_mut();
        let size = *map.size();
        if size.width() < 3 || size.height() < 3 {
            return;
        }

        self.dun_gen_placed_map(map);
    }

    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>) {
        // Convenience.
        let size = *map.size();
        if size.width() < 3 || size.height() < 3 {
            return;
        }

        let count = self.provides_count.provide_count();
        let mut rng = thread_rng();
        for _ in 0..count {
            let total_odds = size.height() as f64 + size.width() as f64;
            let on_vertical_wall = rng.gen_bool(size.height() as f64 / total_odds);
            if on_vertical_wall {
                let portal_y = rng.gen_range(1, size.height() - 1) as i32;
                let on_left_wall = rng.gen_bool(0.5);
                if on_left_wall {
                    map.add_portal(
                        ShapePosition::new(0, portal_y),
                        OrdinalDirection::East,
                        (self.placed_room_box_func)(),
                    );
                } else {
                    map.add_portal(
                        ShapePosition::new(size.width() as i32 - 1, portal_y),
                        OrdinalDirection::West,
                        (self.placed_room_box_func)(),
                    );
                }
            } else {
                let portal_x = rng.gen_range(1, size.width() - 1) as i32;
                let on_top_wall = rng.gen_bool(0.5);
                if on_top_wall {
                    map.add_portal(
                        ShapePosition::new(portal_x, 0),
                        OrdinalDirection::South,
                        (self.placed_room_box_func)(),
                    );
                } else {
                    map.add_portal(
                        ShapePosition::new(portal_x, size.height() as i32 - 1),
                        OrdinalDirection::North,
                        (self.placed_room_box_func)(),
                    );
                }
            }
        }
    }
}

impl<TProvidesCount> DoesAllInstancedDunGen for EdgePortalsGenerator<TProvidesCount> where
    TProvidesCount: ProvidesCount + Sized
{
}
