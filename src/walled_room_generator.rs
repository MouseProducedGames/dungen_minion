// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for walling in a room.
///
/// The `WalledRoomGenerator` can be called statically to generate [`TileType`](enum.TileType.html)::Wall around the perimeter of the room, or with an explicit size to add internal `TileType::Wall`.
///
/// The walls will be generated as a rectangle defined by a [`Size`](geometry/struct.Size.html) starting from the [0, 0] [`LocalPosition`](geometry/struct.LocalPosition.html).
///
/// Will generate a walled room 8 tiles wide, and 6 tiles high; its internal area will consist of `TileType::Floor` and be 6 tiles wide, and 4 tiles high, with the remainder being walls.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map =
///     DunGen::new(Box::new(RoomHashMap::new()))
///     .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
///     .gen::<WalledRoomGenerator::<Size>>()
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
pub struct WalledRoomGenerator<TProvidesArea>
where
    TProvidesArea: ProvidesArea + Sized,
{
    provides_area: TProvidesArea,
}

impl<TProvidesArea> WalledRoomGenerator<TProvidesArea>
where
    TProvidesArea: ProvidesArea + Sized,
{
    /// Creates a new generator for walling in a room.
    pub fn new(provides_area: TProvidesArea) -> Self {
        Self { provides_area }
    }
}

impl<TProvidesArea> DoesDunGen for WalledRoomGenerator<TProvidesArea>
where
    TProvidesArea: ProvidesArea + Sized,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        self.dun_gen_map(map_id);
    }

    fn dun_gen_map(&self, map_id: MapId) {
        let maps = &MAPS.read()[map_id];
        let map = &mut maps.write();

        let area = self.provides_area.provide_area();
        let area = if area.width() > 0 || area.height() > 0 {
            area
        } else {
            Area::from(*map.size())
        };

        if *area.size() == Size::zero() {
            return;
        }

        for x in area.position().x()..=area.right() {
            map.tile_type_at_local_set(Position::new(x, 0), TileType::Wall);
        }
        for y in area.position().y()..=area.bottom() {
            map.tile_type_at_local_set(Position::new(0, y), TileType::Wall);
            map.tile_type_at_local_set(Position::new(area.right(), y), TileType::Wall);
        }
        for x in area.position().x()..=area.right() {
            map.tile_type_at_local_set(Position::new(x, area.bottom()), TileType::Wall);
        }
    }
}
