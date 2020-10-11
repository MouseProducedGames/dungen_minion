// External includes.

// Standard includes.

// Internal includes.
use super::{
    DoesDunGen, DoesDunGenPlaced, DoesDunGenPlacedStatic, DoesDunGenStatic, PlacedRoom, Room,
    SupportsDunGen, SupportsDunGenPlaced, TileType,
};
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
///     .gen::<WalledRoomDunGen>()
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
pub struct WalledRoomDunGen {
    size: Size,
    marker: std::marker::PhantomData<dyn Room>,
}

impl WalledRoomDunGen {
    /// Creates a new generator for walling in a room.
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
        self.dun_gen_map(map);
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        // Convenience.
        let size = self.size;
        if size.width() == 0 || size.height() == 0 {
            return;
        }

        for x in 0..size.width() {
            map.tile_type_at_local_set(ShapePosition::new(x as Coord, 0), TileType::Wall);
        }
        for y in 0..size.height() {
            map.tile_type_at_local_set(ShapePosition::new(0, y as Coord), TileType::Wall);
            map.tile_type_at_local_set(
                ShapePosition::new(size.width() as Coord - 1, y as Coord),
                TileType::Wall,
            );
        }
        for x in 0..size.width() {
            map.tile_type_at_local_set(
                ShapePosition::new(x as Coord, size.height() as Coord - 1),
                TileType::Wall,
            );
        }
    }
}

impl DoesDunGenPlaced for WalledRoomDunGen {
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced) {
        // Convenience.
        let size = self.size;
        if size.width() == 0 || size.height() == 0 {
            return;
        }

        let map = target.get_placed_map_mut();
        self.dun_gen_placed_map(map);
    }

    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>) {
        // Convenience.
        let size = self.size;
        if size.width() == 0 || size.height() == 0 {
            return;
        }

        for x in 0..size.width() {
            map.tile_type_at_local_set(ShapePosition::new(x as Coord, 0), TileType::Wall);
        }
        for y in 0..size.height() {
            map.tile_type_at_local_set(ShapePosition::new(0, y as Coord), TileType::Wall);
            map.tile_type_at_local_set(
                ShapePosition::new(size.width() as Coord - 1, y as Coord),
                TileType::Wall,
            );
        }
        for x in 0..size.width() {
            map.tile_type_at_local_set(
                ShapePosition::new(x as Coord, size.height() as Coord - 1),
                TileType::Wall,
            );
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

impl DoesDunGenPlacedStatic for WalledRoomDunGen {
    fn dun_gen_placed_static(target: &mut dyn SupportsDunGenPlaced) {
        let size = *target.get_placed_map().size();
        WalledRoomDunGen::new(size).dun_gen_placed(target);
    }

    fn dun_gen_placed_map_static(map: &mut Box<dyn PlacedRoom>) {
        let size = *(map.size());
        WalledRoomDunGen::new(size).dun_gen_placed_map(map);
    }
}
