// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// Used to sequentially execute a series of dungeon generators..
///
/// The following chain will generate a room with a [`Size`](geometry/struct.Size.html) of 12 tiles wide by 8 tiles high (including walls), and then add 5 randomly-placed hallways projecting off of it.
///```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map =
///     DunGen::new(Box::new(RoomHashMap::new()))
///     .gen_with(SequentialDunGen::new(&[
///         &EmptyRoomDunGen::new(Size::new(12, 8)),
///         &WalledRoomDunGen::new(Size::new(12, 8)),
///         &EdgePortalsDunGen::new(
///             1,
///             Box::new(|| {
///                 Box::new(PlacedRoomWrapper::new(
///                     Position::new(0, 0),
///                     RoomHashMap::default(),
///                 ))
///             }),
///         )
///     ]))
///     .gen_with(TraversePortalsDunGen::new(SequentialDunGen::new(&[
///         &EmptyRoomDunGen::new(Size::new(8, 6)),
///         &WalledRoomDunGen::new(Size::new(8, 6)),
///     ])))
///     .gen_with(
///         EdgePortalsDunGen::new(
///             4,
///             Box::new(|| {
///                 Box::new(PlacedRoomWrapper::new(
///                     Position::new(0, 0),
///                     RoomHashMap::default(),
///                 ))
///             }),
///     ))
///     // The if-check ensures that we only generate on maps that haven't already been generated.
///     .gen_with(TraversePortalsDunGen::new(IfMapDunGen::new(
///         |map| *map.size() == Size::zero(),
///         |placed_map| *placed_map.size() == Size::zero(),
///         SequentialDunGen::new(&[
///             &EmptyRoomDunGen::new(Size::new(3, 10)),
///             &WalledRoomDunGen::new(Size::new(3, 10)),
///         ])
///     )))
///     .build();
///
/// assert!(*map.size() == Size::new(12, 8));
/// assert!(map.portal_count() == 5);
/// let mut count = 0;
/// for (i, portal) in map.portals().into_iter().enumerate() {
///     // The first room we generated has a size of 8 tiles wide by 6 tiles high.
///     if i == 0 {
///         assert!(*portal.target().size() == Size::new(8, 6));
///     } else {
///         assert!(*portal.target().size() == Size::new(3, 10));
///     }
///     assert!(
///         portal.target().tile_type_at_local(
///             ShapePosition::new(0, 0)
///         ) == Some(&TileType::Wall));
///     assert!(
///         portal.target().tile_type_at_local(
///             ShapePosition::new(1, 1)
///         ) == Some(&TileType::Floor));
///     count += 1;
/// }
/// assert!(count == 5);
///```
pub struct IfMapDunGen<TDunGen, TMapFunc, TPlacedMapFunc>
where
    TDunGen: DoesAllInstancedDunGen,
    TMapFunc: Fn(&Box<dyn Room>) -> bool,
    TPlacedMapFunc: Fn(&Box<dyn PlacedRoom>) -> bool,
{
    map_func: TMapFunc,
    placed_map_func: TPlacedMapFunc,
    dun_gen: TDunGen,
}

impl<TDunGen, TMapFunc, TPlacedMapFunc> IfMapDunGen<TDunGen, TMapFunc, TPlacedMapFunc>
where
    TDunGen: DoesAllInstancedDunGen,
    TMapFunc: Fn(&Box<dyn Room>) -> bool,
    TPlacedMapFunc: Fn(&Box<dyn PlacedRoom>) -> bool,
{
    /// Creates a new sequential set of dungeon generators.
    pub fn new(map_func: TMapFunc, placed_map_func: TPlacedMapFunc, dun_gen: TDunGen) -> Self {
        Self {
            map_func,
            placed_map_func,
            dun_gen,
        }
    }
}

impl<TDunGen, TMapFunc, TPlacedMapFunc> DoesDunGen
    for IfMapDunGen<TDunGen, TMapFunc, TPlacedMapFunc>
where
    TDunGen: DoesAllInstancedDunGen,
    TMapFunc: Fn(&Box<dyn Room>) -> bool,
    TPlacedMapFunc: Fn(&Box<dyn PlacedRoom>) -> bool,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map = target.get_map_mut();
        if (self.map_func)(map) {
            self.dun_gen.dun_gen(target);
        }
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        if (self.map_func)(map) {
            self.dun_gen.dun_gen_map(map);
        }
    }
}

impl<TDunGen, TMapFunc, TPlacedMapFunc> DoesDunGenPlaced
    for IfMapDunGen<TDunGen, TMapFunc, TPlacedMapFunc>
where
    TDunGen: DoesAllInstancedDunGen,
    TMapFunc: Fn(&Box<dyn Room>) -> bool,
    TPlacedMapFunc: Fn(&Box<dyn PlacedRoom>) -> bool,
{
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced) {
        let map = target.get_placed_map_mut();
        if (self.placed_map_func)(map) {
            self.dun_gen.dun_gen_placed(target);
        }
    }

    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>) {
        if (self.placed_map_func)(map) {
            self.dun_gen.dun_gen_placed_map(map);
        }
    }
}

impl<TDunGen, TMapFunc, TPlacedMapFunc> DoesAllInstancedDunGen
    for IfMapDunGen<TDunGen, TMapFunc, TPlacedMapFunc>
where
    TDunGen: DoesAllInstancedDunGen,
    TMapFunc: Fn(&Box<dyn Room>) -> bool,
    TPlacedMapFunc: Fn(&Box<dyn PlacedRoom>) -> bool,
{
}
