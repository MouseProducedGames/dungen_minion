// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for filling an area with a [`TileType`](enum.TileType.html).
///
/// `FillTilesGenerator` can be called as an instance with an explicit `TileType` and [`ShapeArea`](geometry/struct.ShapeArea.html) to add that `TileType` to the given `ShapeArea`.
///
/// The tiles will be generated as a rectangle defined by the `ShapeArea` .
///
/// Will generate a walled area inside an empty room with a 'Size' 12 tiles wide, and 8 tiles high; its internal area will consist of `TileType::Wall` and be 6 tiles wide, and 4 tiles high, with the remainder being 'TileType::Floor'.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map =
///     DunGen::new(Box::new(RoomHashMap::new()))
///     .gen_with(FillTilesGenerator::new(Size::new(12, 8), TileType::Floor))
///     .gen_with(FillTilesGenerator::new(
///         ShapeArea::new(
///             ShapePosition::new(3, 2),
///             Size::new(6, 4)),
///         TileType::Wall))
///     .build();
///
/// assert!(*map.size() == Size::new(12, 8));
/// let mut floor_tile_count = 0;
/// let mut wall_tile_count = 0;
/// let mut tile_count = 0;
/// for y in 0..map.size().height() {
///     for x in 0..map.size().width() {
///         let shape_position = ShapePosition::new(x as i32, y as i32);
///         if (x <= 2 || y <= 1 ||
///             x >= (map.size().width() - 3) || y >= (map.size().height() - 2)) {
///             assert!(map.tile_type_at_local(shape_position) == Some(&TileType::Floor));
///             floor_tile_count += 1;
///         } else {
///             assert!(map.tile_type_at_local(shape_position) == Some(&TileType::Wall));
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
pub struct FillTilesGenerator<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    provides_shape_area: TProvidesShapeArea,
    tile_type_fill: TileType,
}

impl<TProvidesShapeArea> FillTilesGenerator<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    /// Creates a new generator for filling an area of the room with the specified `TileType`.
    pub fn new(provides_shape_area: TProvidesShapeArea, tile_type_fill: TileType) -> Self {
        Self {
            provides_shape_area,
            tile_type_fill,
        }
    }
}

impl<TProvidesShapeArea> DoesDunGen for FillTilesGenerator<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map = target.get_map_mut();
        self.dun_gen_map(map);
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        let shape_area = self.provides_shape_area.provide_shape_area();
        let shape_area = if shape_area.width() > 0 || shape_area.height() > 0 {
            shape_area
        } else {
            ShapeArea::from(*map.size())
        };

        if *shape_area.size() == Size::zero() {
            return;
        }

        for y in shape_area.shape_position().y()..=shape_area.bottom() {
            for x in shape_area.shape_position().x()..=shape_area.right() {
                println!("{}, {}", x, y);
                map.tile_type_at_local_set(ShapePosition::new(x, y), self.tile_type_fill);
            }
        }
    }
}

impl<TProvidesShapeArea> DoesDunGenPlaced for FillTilesGenerator<TProvidesShapeArea>
where
    TProvidesShapeArea: ProvidesShapeArea + Sized,
{
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced) {
        let map = target.get_placed_map_mut();
        self.dun_gen_placed_map(map);
    }

    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>) {
        let shape_area = self.provides_shape_area.provide_shape_area();
        let shape_area = if shape_area.width() > 0 || shape_area.height() > 0 {
            shape_area
        } else {
            ShapeArea::from(*map.size())
        };

        if *shape_area.size() == Size::zero() {
            return;
        }

        for y in shape_area.shape_position().y()..=shape_area.bottom() {
            for x in shape_area.shape_position().x()..=shape_area.right() {
                map.tile_type_at_local_set(ShapePosition::new(x, y), self.tile_type_fill);
            }
        }
    }
}

impl<TProvidesShapeArea> DoesAllInstancedDunGen for FillTilesGenerator<TProvidesShapeArea> where
    TProvidesShapeArea: ProvidesShapeArea + Sized
{
}