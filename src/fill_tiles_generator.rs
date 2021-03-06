// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for filling an area with a [`TileType`](enum.TileType.html).
///
/// `FillTilesGenerator` can be called with a `Size` of (0, 0) to generate `TileType::Floor` across the entire area of the map, or with an explicit area to add internal `TileType::Floor`.
///
/// The tiles will be generated as a rectangle defined by an [`Area`](geometry/struct.Area.html), or by a type implementing [`ProvidesArea`](geometry/trait.ProvidesArea.hmtl).
///
/// Will generate a walled area inside an empty map with a 'Size' 12 tiles wide, and 8 tiles high; its internal area will consist of `TileType::Wall` and be 6 tiles wide, and 4 tiles high, with the remainder being 'TileType::Floor'.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map_id =
///     DunGen::new(SparseMap::new())
///     .gen_with(FillTilesGenerator::new(Size::new(12, 8), TileType::Floor))
///     .gen_with(FillTilesGenerator::new(
///         Area::new(Position::new(3, 2), Size::new(6, 4)),
///         TileType::Wall))
///     .build();
///
/// let maps = MAPS.read();
/// let map = maps[map_id].read();
///
/// assert!(*map.size() == Size::new(12, 8));
///
/// let mut floor_tile_count = 0;
/// let mut wall_tile_count = 0;
/// let mut tile_count = 0;
/// for y in 0..map.size().height() {
///     for x in 0..map.size().width() {
///         let local_position = Position::new(x as i32, y as i32);
///         if (x <= 2 || y <= 1 ||
///             x >= (map.size().width() - 3) || y >= (map.size().height() - 2)) {
///             assert!(map.tile_type_at_local(local_position) == Some(TileType::Floor));
///             floor_tile_count += 1;
///         } else {
///             assert!(map.tile_type_at_local(local_position) == Some(TileType::Wall));
///             wall_tile_count += 1;
///         }
///     }    
/// }
/// // Area of the inner rectangle.
/// assert!(wall_tile_count == (6 * 4));
/// // Area of a rectangle, minus an inner rectangle.
/// assert!(floor_tile_count == (12 * 8) - (6 * 4));
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
pub struct FillTilesGenerator<TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape + Sized,
{
    provides_placed_shape: TProvidesPlacedShape,
    tile_type_fill: TileType,
}

impl<TProvidesPlacedShape> FillTilesGenerator<TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape + Sized,
{
    /// Creates a new generator for filling an area of the map with the specified `TileType`.
    pub fn new(provides_placed_shape: TProvidesPlacedShape, tile_type_fill: TileType) -> Self {
        Self {
            provides_placed_shape,
            tile_type_fill,
        }
    }
}

impl<TProvidesPlacedShape> DoesDunGen for FillTilesGenerator<TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape + Sized,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        self.dun_gen_map(map_id);
    }

    fn dun_gen_map(&self, map_id: MapId) {
        let shape = self.provides_placed_shape.provide_placed_shape();
        let maps = &MAPS.read();
        let map = &mut maps[map_id].write();
        let possible_area = Area::from(*map.size());
        let shape = if shape.width() > 0 || shape.height() > 0 {
            shape
        } else {
            possible_area.provide_placed_shape()
        };

        if *shape.size() == Size::zero() {
            return;
        }

        for y in shape.top()..=shape.bottom() {
            for x in shape.left()..=shape.right() {
                let position = Position::new(x, y);
                if shape.intersects_position(position) {
                    map.tile_type_at_local_set(position, self.tile_type_fill);
                }
            }
        }
    }
}
