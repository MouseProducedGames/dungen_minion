// External includes.

// Standard includes.

// Internal includes.
use super::{DoesDunGen, DoesDunGenStatic, Room, SupportsDunGen, TileType};
use crate::geometry::*;

pub struct WalledRoomDunGen {
    size: Size,
    marker: std::marker::PhantomData<dyn Room>,
}

impl WalledRoomDunGen {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            marker: std::marker::PhantomData::default(),
        }
    }
}

impl DoesDunGen for WalledRoomDunGen {
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        // Convenience.
        let size = self.size;
        if size.width() == 0 || size.height() == 0 {
            return;
        }

        let map = target.get_map_mut();
        for x in 0..size.width() {
            map.tile_type_at_local_set(LocalPosition::new(x, 0), TileType::Wall);
        }
        for y in 0..size.height() {
            map.tile_type_at_local_set(LocalPosition::new(0, y), TileType::Wall);
            map.tile_type_at_local_set(LocalPosition::new(size.width() - 1, y), TileType::Wall);
        }
        for x in 0..size.width() {
            map.tile_type_at_local_set(LocalPosition::new(x, size.height() - 1), TileType::Wall);
        }
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        // Convenience.
        let size = self.size;
        if size.width() == 0 || size.height() == 0 {
            return;
        }

        for x in 0..size.width() {
            map.tile_type_at_local_set(LocalPosition::new(x, 0), TileType::Wall);
        }
        for y in 0..size.height() {
            map.tile_type_at_local_set(LocalPosition::new(0, y), TileType::Wall);
            map.tile_type_at_local_set(LocalPosition::new(size.width() - 1, y), TileType::Wall);
        }
        for x in 0..size.width() {
            map.tile_type_at_local_set(LocalPosition::new(x, size.height() - 1), TileType::Wall);
        }
    }
}

impl DoesDunGenStatic for WalledRoomDunGen {
    fn dun_gen_static(target: &mut dyn SupportsDunGen) {
        let size = *target.get_map().size();
        WalledRoomDunGen::new(size).dun_gen(target);
    }

    fn dun_gen_map_static(map: &mut Box<dyn Room>) {
        let size = *(map.size());
        WalledRoomDunGen::new(size).dun_gen_map(map);
    }
}
