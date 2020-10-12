// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for walling in a room.
///
/// The `WalledRoomDunGen` can be called statically to generate [`TileType`](enum.TileType.html)::Wall around the perimeter of the room, or with an explicit size to add internal `TileType::Wall`.
///
/// The walls will be generated as a rectangle defined by a [`Size`](geometry/struct.Size.html) starting from the [0, 0] [`LocalPosition`](geometry/struct.LocalPosition.html).
///
/// Will generate a walled room 8 tiles wide, and 6 tiles high; its internal area will consist of `TileType::Floor` and be 6 tiles wide, and 4 tiles high, with the remainder being walls.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map =
///     DunGen::new(Box::new(RoomHashMap::new()))
///     .gen_with(EmptyRoomDunGen::new(Size::new(8, 6)))
///     .gen::<WalledRoomDunGen::<Size>>()
///     .build();
///
/// assert!(*map.size() == Size::new(8, 6));
/// let mut floor_tile_count = 0;
/// let mut wall_tile_count = 0;
/// for y in 0..map.size().height() {
///     for x in 0..map.size().width() {
///         let shape_position = ShapePosition::new(x as i32, y as i32);
///         if (x == 0 || y == 0 ||
///             x == (map.size().width() - 1) || y == (map.size().height() - 1)) {
///             assert!(map.tile_type_at_local(shape_position) == Some(&TileType::Wall));
///             wall_tile_count += 1;
///         } else {
///             assert!(map.tile_type_at_local(shape_position) == Some(&TileType::Floor));
///             floor_tile_count += 1;
///         }
///     }    
/// }
/// // Area of a rectangle.
/// assert!(floor_tile_count == (6 * 4));
/// // Perimeter of a tiled rectangle; the corners are only included on two of the edges, and so
/// // we subtract the four corner end tiles of the other two edges (this can be worked out on a
/// // graph).
/// assert!(wall_tile_count == ((8 * 2) + ((6 * 2) - 4)));
///
/// assert!(map.tile_type_at_local(ShapePosition::new(0, 0)) == Some(&TileType::Wall));
/// assert!(map.tile_type_at_local(ShapePosition::new(1, 1)) == Some(&TileType::Floor));
/// assert!(map.portal_count() == 0);
/// let mut count = 0;
/// for portal in map.portals() {
///     // Test will error out if it enters this loop (ie., any portals exist).
///     assert!(false);
///     count += 1;
/// }
/// assert!(count == 0);
/// ```
pub struct WalledRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    provides_shape_area: TProvidesShapeArea,
}

impl<TProvidesShapeArea> WalledRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    /// Creates a new generator for walling in a room.
    pub fn new(provides_shape_area: TProvidesShapeArea) -> Self {
        Self {
            provides_shape_area,
        }
    }
}

impl<TProvidesShapeArea> DoesDunGen for WalledRoomDunGen<TProvidesShapeArea>
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

        for x in shape_area.shape_position().x()..=shape_area.right() {
            map.tile_type_at_local_set(ShapePosition::new(x, 0), TileType::Wall);
        }
        for y in shape_area.shape_position().y()..=shape_area.bottom() {
            map.tile_type_at_local_set(ShapePosition::new(0, y), TileType::Wall);
            map.tile_type_at_local_set(ShapePosition::new(shape_area.right(), y), TileType::Wall);
        }
        for x in shape_area.shape_position().x()..=shape_area.right() {
            map.tile_type_at_local_set(ShapePosition::new(x, shape_area.bottom()), TileType::Wall);
        }
    }
}

impl<TProvidesShapeArea> DoesDunGenPlaced for WalledRoomDunGen<TProvidesShapeArea>
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

        for x in shape_area.shape_position().x()..=shape_area.right() {
            map.tile_type_at_local_set(ShapePosition::new(x, 0), TileType::Wall);
        }
        for y in shape_area.shape_position().y()..=shape_area.bottom() {
            map.tile_type_at_local_set(ShapePosition::new(0, y), TileType::Wall);
            map.tile_type_at_local_set(ShapePosition::new(shape_area.right(), y), TileType::Wall);
        }
        for x in shape_area.shape_position().x()..=shape_area.right() {
            map.tile_type_at_local_set(ShapePosition::new(x, shape_area.bottom()), TileType::Wall);
        }
    }
}

impl<TProvidesShapeArea> DoesDunGenStatic for WalledRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    fn dun_gen_static(target: &mut dyn SupportsDunGen) {
        let size = *target.get_map().size();
        WalledRoomDunGen::new(size).dun_gen(target);
    }

    fn dun_gen_map_static(map: &mut Box<dyn Room>) {
        let size = *(map.size());
        WalledRoomDunGen::new(size).dun_gen_map(map);
    }
}

impl<TProvidesShapeArea> DoesDunGenPlacedStatic for WalledRoomDunGen<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    fn dun_gen_placed_static(target: &mut dyn SupportsDunGenPlaced) {
        let size = *target.get_placed_map().size();
        WalledRoomDunGen::new(size).dun_gen_placed(target);
    }

    fn dun_gen_placed_map_static(map: &mut Box<dyn PlacedRoom>) {
        let size = *(map.size());
        WalledRoomDunGen::new(size).dun_gen_placed_map(map);
    }
}

impl<TProvidesShapeArea> DoesAllDunGen for WalledRoomDunGen<TProvidesShapeArea> where
    TProvidesShapeArea: ProvidesShapeArea + Sized
{
}
impl<TProvidesShapeArea> DoesAllInstancedDunGen for WalledRoomDunGen<TProvidesShapeArea> where
    TProvidesShapeArea: ProvidesShapeArea + Sized
{
}
impl<TProvidesShapeArea> DoesAllStaticDunGen for WalledRoomDunGen<TProvidesShapeArea> where
    TProvidesShapeArea: ProvidesShapeArea + Sized
{
}
