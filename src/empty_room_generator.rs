// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for creating an area of [`TileType`](enum.TileType.html)::Floor.
///
/// The `EmptyRoomGenerator` can be called with a `Size` of (0, 0) to generate `TileType::Floor` across the entire area of the map, or with an explicit area to add internal `TileType::Floor`.
///
/// The floors will be generated as a rectangle defined by an [`Area`](geometry/struct.Area.html), or by a type implementing [`ProvidesArea`](geometry/trait.ProvidesArea.hmtl).
///
/// Will generate an empty map with a 'Size' 8 tiles wide, and 6 tiles high; its internal area will consist of `TileType::Floor` and be 8 tiles wide, and 6 tiles high, with no remainder.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map_id =
///     DunGen::new(SparseMap::new())
///     .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
///     .build();
///
/// let maps = MAPS.read();
/// let map = maps[map_id].read();
///
/// assert!(*map.size() == Size::new(8, 6));
/// let mut tile_count = 0;
/// for y in 0..map.size().height() {
///     for x in 0..map.size().width() {
///         let local_position = Position::new(x as i32, y as i32);
///         assert!(map.tile_type_at_local(local_position) == Some(TileType::Floor));
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
pub struct EmptyRoomGenerator<TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape + Sized,
{
    forward_to: FillTilesGenerator<TProvidesPlacedShape>,
}

impl<TProvidesPlacedShape> EmptyRoomGenerator<TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape + Sized,
{
    /// Creates a new generator for adding flooring to a map.
    pub fn new(provides_placed_shape: TProvidesPlacedShape) -> Self {
        Self {
            forward_to: FillTilesGenerator::new(provides_placed_shape, TileType::Floor),
        }
    }
}

impl<TProvidesPlacedShape> DoesDunGen for EmptyRoomGenerator<TProvidesPlacedShape>
where
    TProvidesPlacedShape: ProvidesPlacedShape + Sized,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        self.forward_to.dun_gen(target)
    }

    fn dun_gen_map(&self, map_id: MapId) {
        self.forward_to.dun_gen_map(map_id)
    }
}
