// External includes.

// Standard includes.

// Internal includes.
use super::{DoesDunGen, DoesDunGenStatic, SupportsDunGen, TileType};
use crate::geometry::{LocalPosition, Size};

pub struct EmptyRoomDunGen {
    size: Size,
}

impl EmptyRoomDunGen {
    pub fn new(size: Size) -> Self {
        Self { size }
    }
}

impl<'a> DoesDunGen<'a> for EmptyRoomDunGen {
    fn dun_gen<'b>(&self, target: &mut dyn SupportsDunGen<'b>) {
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
}

impl<'a> DoesDunGenStatic<'a> for EmptyRoomDunGen {
    fn dun_gen_static<'b>(target: &mut dyn SupportsDunGen<'b>) {
        let size = *(target.get_map().size());
        EmptyRoomDunGen::new(size).dun_gen(target);
    }
}
