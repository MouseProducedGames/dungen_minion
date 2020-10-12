// External includes.

// Standard includes.

// Internal includes.
use super::{
    DoesDunGen, DoesDunGenPlaced, DoesDunGenPlacedStatic, DoesDunGenStatic, PlacedRoom, Room,
    SupportsDunGen, SupportsDunGenPlaced, TileType,
};
use crate::geometry::*;

/// A generator for creating an area of [`TileType`](enum.TileType.html)::Floor.
///
/// The `EmptyRoomDunGen` can be called statically to generate `TileType::Floor` across the entire area of the room, or with an explicit size to add internal `TileType::Floor`.
///
/// The floors will be generated as a rectangle defined by a [`Size`](geometry/struct.Size.html) starting from the [0, 0] [`LocalPosition`](geometry/struct.LocalPosition.html).
///
/// Will generate an empty room with a 'Size' 8 tiles wide, and 6 tiles high; its internal area will consist of `TileType::Floor` and be 8 tiles wide, and 6 tiles high, with no remainder.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map =
///     DunGen::new(Box::new(RoomHashMap::new()))
///     .gen_with(EmptyRoomDunGen::new(Size::new(8, 6)))
///     .build();
///
/// assert!(*map.size() == Size::new(8, 6));
/// let mut tile_count = 0;
/// for y in 0..map.size().height() {
///     for x in 0..map.size().width() {
///         let shape_position = ShapePosition::new(x as i32, y as i32);
///         assert!(map.tile_type_at_local(shape_position) == Some(&TileType::Floor));
///         tile_count += 1;
///     }    
/// }
/// assert!(tile_count == (8 * 6));
///
/// assert!(map.portal_count() == 0);
/// let mut count = 0;
/// for portal in map.portals() {
///     // Test will error out if it enters this loop (ie., any portals exist).
///     assert!(false);
///     count += 1;
/// }
/// assert!(count == 0);
/// ```
pub struct EmptyRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    provides_shape_area: TProvidesShapeArea,
}

impl<TProvidesShapeArea> EmptyRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    /// Creates a new generator for adding flooring to a room.
    pub fn new(provides_shape_area: TProvidesShapeArea) -> Self {
        Self {
            provides_shape_area,
        }
    }
}

impl<TProvidesShapeArea> DoesDunGen for EmptyRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map = target.get_map_mut();
        self.dun_gen_map(map);
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        let shape_area = self.provides_shape_area.provide_shape_area();
        if shape_area.width() == 0 || shape_area.height() == 0 {
            return;
        }

        for y in shape_area.shape_position().y()..=shape_area.bottom() {
            for x in shape_area.shape_position().x()..=shape_area.right() {
                map.tile_type_at_local_set(ShapePosition::new(x, y), TileType::Floor);
            }
        }
    }
}

impl<TProvidesShapeArea> DoesDunGenPlaced for EmptyRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced) {
        let map = target.get_placed_map_mut();
        self.dun_gen_placed_map(map);
    }

    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>) {
        let shape_area = self.provides_shape_area.provide_shape_area();
        if shape_area.width() == 0 || shape_area.height() == 0 {
            return;
        }

        for y in shape_area.shape_position().y()..=shape_area.bottom() {
            for x in shape_area.shape_position().x()..=shape_area.right() {
                map.tile_type_at_local_set(ShapePosition::new(x, y), TileType::Floor);
            }
        }
    }
}

impl<TProvidesShapeArea> DoesDunGenStatic for EmptyRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    fn dun_gen_static(target: &mut dyn SupportsDunGen) {
        let size = *(target.get_map().size());
        EmptyRoomDunGen::new(ShapeArea::new(ShapePosition::new(0, 0), size)).dun_gen(target);
    }

    fn dun_gen_map_static(map: &mut Box<dyn Room>) {
        let size = *(map.size());
        EmptyRoomDunGen::new(ShapeArea::new(ShapePosition::new(0, 0), size)).dun_gen_map(map);
    }
}

impl<TProvidesShapeArea> DoesDunGenPlacedStatic for EmptyRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    fn dun_gen_placed_static(target: &mut dyn SupportsDunGenPlaced) {
        let size = *(target.get_placed_map().size());
        EmptyRoomDunGen::new(ShapeArea::new(ShapePosition::new(0, 0), size)).dun_gen_placed(target);
    }

    fn dun_gen_placed_map_static(map: &mut Box<dyn PlacedRoom>) {
        let size = *(map.size());
        EmptyRoomDunGen::new(ShapeArea::new(ShapePosition::new(0, 0), size))
            .dun_gen_placed_map(map);
    }
}
