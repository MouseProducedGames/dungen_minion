// External includes.
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::{
    DoesDunGen, DoesDunGenPlaced, DoesDunGenPlacedStatic, DoesDunGenStatic, PlacedRoom, Room,
    SupportsDunGen, SupportsDunGenPlaced,
};
use crate::geometry::*;

/// A generator for adding one or more instances of `Portal` to a room.
///
/// The `EdgePortalsDunGen` **cannot** be called statically, but can be called with an explicit
/// count to add one or more internal `Portal` and `TileType::Portal` instances.
///
/// The portals will be generated randomly on the edge of the room, excluding corners..
///
/// ```
/// // Will generate 5 `Portal` and `TileType::Portal` instances; each matching instance will be on
/// // the same `LocalPosition`. Each `Portal` will have an attached `Box<dyn PlacedRoom>` which can
/// // be edited by calling the appropriate methods with various generators, or manually after
/// // generation.
/// let map =
///     dungen_minion::DunGen::new(Box::new(dungen_minion::RoomHashMap::new()))
///     .gen_with(dungen_minion::EmptyRoomDunGen::new(dungen_minion::geometry::Size::new(8, 6)))
///     .gen::<dungen_minion::WalledRoomDunGen>()
///     .build();
/// ```
pub struct EdgePortalsDunGen {
    count: usize,
    placed_room_box_func: Box<dyn Fn() -> Box<dyn PlacedRoom>>,
}

impl EdgePortalsDunGen {
    /// Creates a new generator for adding portals to a room.
    pub fn new(count: usize, placed_room_box_func: Box<dyn Fn() -> Box<dyn PlacedRoom>>) -> Self {
        Self {
            count,
            placed_room_box_func,
        }
    }
}

impl DoesDunGen for EdgePortalsDunGen {
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

        let mut rng = thread_rng();
        for _ in 0..self.count {
            let total_odds = size.height() as f64 + size.width() as f64;
            let on_vertical_wall = rng.gen_bool(size.height() as f64 / total_odds);
            if on_vertical_wall {
                let portal_y = rng.gen_range(1, size.height() - 1);
                let on_left_wall = rng.gen_bool(0.5);
                if on_left_wall {
                    map.add_portal(
                        LocalPosition::new(0, portal_y),
                        OrdinalDirection::East,
                        (self.placed_room_box_func)(),
                    );
                } else {
                    map.add_portal(
                        LocalPosition::new(size.width() - 1, portal_y),
                        OrdinalDirection::West,
                        (self.placed_room_box_func)(),
                    );
                }
            } else {
                let portal_x = rng.gen_range(1, size.width() - 1);
                let on_top_wall = rng.gen_bool(0.5);
                if on_top_wall {
                    map.add_portal(
                        LocalPosition::new(portal_x, 0),
                        OrdinalDirection::South,
                        (self.placed_room_box_func)(),
                    );
                } else {
                    map.add_portal(
                        LocalPosition::new(portal_x, size.height() - 1),
                        OrdinalDirection::North,
                        (self.placed_room_box_func)(),
                    );
                }
            }
        }
    }
}

impl DoesDunGenPlaced for EdgePortalsDunGen {
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

        let mut rng = thread_rng();
        for _ in 0..self.count {
            let total_odds = size.height() as f64 + size.width() as f64;
            let on_vertical_wall = rng.gen_bool(size.height() as f64 / total_odds);
            if on_vertical_wall {
                let portal_y = rng.gen_range(1, size.height() - 1);
                let on_left_wall = rng.gen_bool(0.5);
                if on_left_wall {
                    map.add_portal(
                        LocalPosition::new(0, portal_y),
                        OrdinalDirection::East,
                        (self.placed_room_box_func)(),
                    );
                } else {
                    map.add_portal(
                        LocalPosition::new(size.width() - 1, portal_y),
                        OrdinalDirection::West,
                        (self.placed_room_box_func)(),
                    );
                }
            } else {
                let portal_x = rng.gen_range(1, size.width() - 1);
                let on_top_wall = rng.gen_bool(0.5);
                if on_top_wall {
                    map.add_portal(
                        LocalPosition::new(portal_x, 0),
                        OrdinalDirection::South,
                        (self.placed_room_box_func)(),
                    );
                } else {
                    map.add_portal(
                        LocalPosition::new(portal_x, size.height() - 1),
                        OrdinalDirection::North,
                        (self.placed_room_box_func)(),
                    );
                }
            }
        }
    }
}

impl DoesDunGenStatic for EdgePortalsDunGen {
    fn dun_gen_static(_target: &mut dyn SupportsDunGen) {
        panic!("No idea how many portals to add.");
    }

    fn dun_gen_map_static(_map: &mut Box<dyn Room>) {
        panic!("No idea how many portals to add.");
    }
}

impl DoesDunGenPlacedStatic for EdgePortalsDunGen {
    fn dun_gen_placed_static(_target: &mut dyn SupportsDunGenPlaced) {
        panic!("No idea how many portals to add.");
    }

    fn dun_gen_placed_map_static(_map: &mut Box<dyn PlacedRoom>) {
        panic!("No idea how many portals to add.");
    }
}
