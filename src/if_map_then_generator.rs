// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// Used to conditionally execute a dungeon generator.
///
/// The following chain will generate a map with a [`Size`](geometry/struct.Size.html) of 12 tiles wide by 8 tiles high (including walls), and then add 1 randomly-placed room and 4 randomly-placed hallways projecting off of it.
///```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map_id =
///     DunGen::new(MapSparse::new())
///     .gen_with(SequentialGenerator::new(&[
///         &EmptyRoomGenerator::new(Size::new(12, 8)),
///         &WalledRoomGenerator::new(Size::zero()),
///         &EdgePortalsGenerator::new(
///             1,
///             Box::new(|| {
///                 MapSparse::new()
///             }),
///         )
///     ]))
///     .gen_with(TraversePortalsGenerator::new(SequentialGenerator::new(&[
///         &EmptyRoomGenerator::new(Size::new(8, 6)),
///         &WalledRoomGenerator::new(Size::zero()),
///     ])))
///     .gen_with(
///         EdgePortalsGenerator::new(
///             4,
///             Box::new(|| {
///                 MapSparse::new()
///             }),
///     ))
///     // The if-check ensures that we only generate on maps that haven't already been generated.
///     .gen_with(TraversePortalsGenerator::new(IfMapThenGenerator::new(
///         |map_id| *MAPS.read()[map_id].read().size() == Size::zero(),
///         SequentialGenerator::new(&[
///             &EmptyRoomGenerator::new(Size::new(3, 10)),
///             &WalledRoomGenerator::new(Size::zero()),
///         ])
///     )))
///     .build();
///
/// let maps = MAPS.read();
/// let map = maps[map_id].read();
///
/// assert!(*map.size() == Size::new(12, 8));
/// assert!(map.portal_count() == 5);
/// let mut count = 0;
/// for (i, portal) in map.portals().into_iter().enumerate() {
///     // The first room we generated has a size of 8 tiles wide by 6 tiles high.
///     let target_map = maps[portal.target()].read();
///     if i == 0 {
///         assert!(*target_map.size() == Size::new(8, 6));
///     } else {
///         assert!(*target_map.size() == Size::new(3, 10));
///     }
///     assert!(
///         target_map.tile_type_at_local(Position::new(0, 0)) == Some(&TileType::Wall));
///     assert!(
///         target_map.tile_type_at_local(Position::new(1, 1)) == Some(&TileType::Floor));
///     count += 1;
/// }
/// assert!(count == 5);
///```
pub struct IfMapThenGenerator<TDunGen, TMapFunc>
where
    TDunGen: DoesDunGen,
    TMapFunc: Fn(MapId) -> bool,
{
    map_func: TMapFunc,
    dun_gen: TDunGen,
}

impl<TDunGen, TMapFunc> IfMapThenGenerator<TDunGen, TMapFunc>
where
    TDunGen: DoesDunGen,
    TMapFunc: Fn(MapId) -> bool,
{
    /// Creates a new conditional dungeon generator.
    pub fn new(map_func: TMapFunc, dun_gen: TDunGen) -> Self {
        Self { map_func, dun_gen }
    }
}

impl<TDunGen, TMapFunc> DoesDunGen for IfMapThenGenerator<TDunGen, TMapFunc>
where
    TDunGen: DoesDunGen,
    TMapFunc: Fn(MapId) -> bool,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        if (self.map_func)(map_id) {
            self.dun_gen.dun_gen(target);
        }
    }

    fn dun_gen_map(&self, map_id: MapId) {
        if (self.map_func)(map_id) {
            self.dun_gen.dun_gen_map(map_id);
        }
    }
}
