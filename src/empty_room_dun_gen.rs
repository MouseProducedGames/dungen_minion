// External includes.

// Standard includes.

// Internal includes.
use super::{DoesDunGen, DoesDunGenStatic, Room, SupportsDunGen, TileType};
use crate::geometry::*;

pub struct EmptyRoomDunGen {
    size: Size,
}

impl EmptyRoomDunGen {
    pub fn new(size: Size) -> Self {
        Self { size }
    }
}

impl DoesDunGen for EmptyRoomDunGen {
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        // Convenience.
        let size = self.size;
        if size.width() == 0 || size.height() == 0 {
            return;
        }

        let map = target.get_map_mut();
        for y in 0..size.height() {
            for x in 0..size.width() {
                map.tile_type_at_local_set(LocalPosition::new(x, y), TileType::Floor);
            }
        }
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        // Convenience.
        let size = self.size;
        if size.width() == 0 || size.height() == 0 {
            return;
        }

        for y in 0..size.height() {
            for x in 0..size.width() {
                map.tile_type_at_local_set(LocalPosition::new(x, y), TileType::Floor);
            }
        }
    }
}

impl DoesDunGenStatic for EmptyRoomDunGen {
    fn dun_gen_static(target: &mut dyn SupportsDunGen) {
        let size = *(target.get_map().size());
        EmptyRoomDunGen::new(size).dun_gen(target);
    }

    fn dun_gen_map_static(map: &mut Box<dyn Room>) {
        let size = *(map.size());
        EmptyRoomDunGen::new(size).dun_gen_map(map);
    }
}
