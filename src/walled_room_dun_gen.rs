// External includes.

// Standard includes.

// Internal includes.
use crate::traits::{
    DoesDunGen, DoesDunGenStatic, LocalPosition, Room, Size, SupportsDunGen, TileType,
};

pub struct WalledRoomDunGen<'a> {
    size: Size,
    marker: std::marker::PhantomData<dyn Room<'a>>,
}

impl<'a> WalledRoomDunGen<'a> {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            marker: std::marker::PhantomData::default(),
        }
    }
}

impl<'a> DoesDunGen<'a> for WalledRoomDunGen<'a> {
    fn dun_gen<'b>(&'a self, target: &mut dyn SupportsDunGen<'b>) {
        // Convenience.
        let size = self.size;
        if size.width() == 0 || size.height() == 0 {
            return;
        }

        let map = target.get_map_mut();
        for x in 0..size.width() {
            map.tile_type_at_local_set(LocalPosition::new(x, 0), TileType::Floor);
        }
        for y in 0..size.height() {
            map.tile_type_at_local_set(LocalPosition::new(0, y), TileType::Floor);
            map.tile_type_at_local_set(LocalPosition::new(size.width() - 1, y), TileType::Floor);
        }
        for x in 0..size.width() {
            map.tile_type_at_local_set(LocalPosition::new(x, size.height() - 1), TileType::Floor);
        }
    }
}

impl<'a> DoesDunGenStatic<'a> for WalledRoomDunGen<'a> {
    fn dun_gen_static<'b>(target: &mut dyn SupportsDunGen<'b>) {
        let size = *target.get_map().size();
        WalledRoomDunGen::new(size).dun_gen(target);
    }
}
