// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for walling in a map.
///
/// The `WalledRoomGenerator` can be called statically to generate [`TileType`](enum.TileType.html)::Wall around the perimeter of the map, or with an explicit size to add internal `TileType::Wall`.
///
/// The walls will be generated as a rectangle defined by an [`Area`](geometry/struct.Area.html) starting from the [0, 0] [`LocalPosition`](geometry/struct.LocalPosition.html).
///
/// Will generate a walled map 8 tiles wide, and 6 tiles high; its internal area will consist of `TileType::Floor` and be 6 tiles wide, and 4 tiles high, with the remainder being walls.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map_id =
///     DunGen::new(SparseMap::new())
///     .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
///     .gen_with(WalledRoomGenerator::new(Size::zero()))
///     .build();
///
/// let maps = MAPS.read();
/// let map = maps[map_id].read();
///
/// assert!(*map.size() == Size::new(8, 6));
/// let mut floor_tile_count = 0;
/// let mut wall_tile_count = 0;
/// for y in 0..map.size().height() {
///     for x in 0..map.size().width() {
///         let local_position = Position::new(x as i32, y as i32);
///         if (x == 0 || y == 0 ||
///             x == (map.size().width() - 1) || y == (map.size().height() - 1)) {
///             assert!(map.tile_type_at_local(local_position) == Some(TileType::Wall));
///             wall_tile_count += 1;
///         } else {
///             assert!(map.tile_type_at_local(local_position) == Some(TileType::Floor));
///             floor_tile_count += 1;
///         }
///     }    
/// }
/// let mut floor_tile_count = 0;
/// let mut wall_tile_count = 0;
/// let mut tile_count = 0;
/// for y in 0..map.size().height() {
///     for x in 0..map.size().width() {
///         let local_position = Position::new(x as i32, y as i32);
///         if (x <= 0 || y <= 0 ||
///             x >= (map.size().width() - 1) || y >= (map.size().height() - 1)) {
///             assert!(map.tile_type_at_local(local_position) == Some(TileType::Wall));
///             floor_tile_count += 1;
///         } else {
///             assert!(map.tile_type_at_local(local_position) == Some(TileType::Floor));
///             wall_tile_count += 1;
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
/// assert!(map.portal_count() == 0);
/// let mut count = 0;
/// for portal in map.portals() {
///     // Test will error out if it enters this loop (ie., any portals exist).
///     assert!(false);
///     count += 1;
/// }
/// assert!(count == 0);
/// ```
pub struct WalledRoomGenerator<'a, TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape + Sized,
{
    provides_placed_shape: TProvidesPlacedShape,
    dont_replace: &'a [Option<TileType>],
}

impl<'a, TProvidesPlacedShape> WalledRoomGenerator<'a, TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape + Sized,
{
    /// Creates a new generator for walling in a map. By default, will not replace `TileType::Portal`s.
    pub fn new(provides_placed_shape: TProvidesPlacedShape) -> Self {
        Self {
            provides_placed_shape,
            dont_replace: &[Some(TileType::Portal)],
        }
    }

    /// Creates a new generator for walling in a map, with a specific filter of `Option<TileType>` options that won't be replaced.
    pub fn with_filter(
        provides_placed_shape: TProvidesPlacedShape,
        dont_replace: &'a [Option<TileType>],
    ) -> Self {
        Self {
            provides_placed_shape,
            dont_replace,
        }
    }

    fn dont_replace(&self, check: &Option<TileType>) -> bool {
        self.dont_replace.contains(check)
    }
}

impl<'a, TProvidesPlacedShape> DoesDunGen for WalledRoomGenerator<'a, TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape + Sized,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        self.dun_gen_map(map_id);
    }

    fn dun_gen_map(&self, map_id: MapId) {
        let maps = &MAPS.read();
        let map = &mut maps[map_id].write();

        let shape = self.provides_placed_shape.provide_placed_shape();
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
                if !self.dont_replace(&map.tile_type_at_local(position))
                    && shape.contains_position(position) == Containment::Intersects
                    && map.contains_position(position) == Containment::Intersects
                {
                    map.tile_type_at_local_set(position, TileType::Wall);
                }
            }
        }
    }
}
