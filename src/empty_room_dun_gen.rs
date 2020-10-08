// External includes.

// Standard includes.

// Internal includes.
use crate::traits::{DoesDunGen, LocalPosition, Size, SupportsDunGen};

pub struct EmptyRoomDunGen<'a> {
    size: Size,
}

impl<'a> EmptyRoomDunGen<'a> {
    pub fn new(size: Size) -> Self {
        Self { size }
    }
}

impl<'a> DoesDunGen<'a> for EmptyRoomDunGen<'a> {
    fn dun_gen(&'a self, target: &'a mut dyn SupportsDunGen<'a>) {
        let mut map = target.get_map_mut();
        for y in 0..size.height() {
            for x in 0..size.width() {
                map.tile_type_at_local_set(LocalPosition::new(x, y), TileType::Floor);
            }
        }
    }
}

impl<'a> DoesDunGenStatic<'a> for EmptyRoomDunGen<'a> {
    fn dun_gen(&'a self, target: &'a mut dyn SupportsDunGen<'a>) {
        let (width, height) = {
            let map = target.get_map();
            (map.width(), map.height())
        }
        EmptyRoomDunGen<'a>::new(
    }
}