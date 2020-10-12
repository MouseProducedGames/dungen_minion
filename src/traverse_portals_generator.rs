// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// Used to execute a dungeon generator by traversing portals.
///
/// The following chain will generate a room with a [`Size`](geometry/struct.Size.html) of 12 tiles wide by 8 tiles high (including walls), and then add 5 randomly-placed hallways projecting off of it.
///```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map =
///     DunGen::new(Box::new(RoomHashMap::new()))
///     .gen_with(SequentialGenerator::new(&[
///         &EmptyRoomGenerator::new(Size::new(12, 8)),
///         &WalledRoomGenerator::new(Size::zero()),
///         &EdgePortalsGenerator::new(
///             5,
///             Box::new(|| {
///                 Box::new(PlacedRoomWrapper::new(
///                     Position::new(0, 0),
///                     RoomHashMap::default(),
///                 ))
///             }),
///         ),
///     ]))
///     .gen_with(TraversePortalsGenerator::new(SequentialGenerator::new(&[
///         &EmptyRoomGenerator::new(Size::new(3, 10)),
///         &WalledRoomGenerator::new(Size::zero()),
///     ])))
///     .build();
///
/// assert!(*map.size() == Size::new(12, 8));
/// assert!(map.portal_count() == 5);
/// let mut count = 0;
/// for portal in map.portals() {
///     assert!(*portal.target().size() == Size::new(3, 10));
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
pub struct TraversePortalsGenerator<TDunGen>
where
    TDunGen: DoesAllInstancedDunGen,
{
    dun_gen: TDunGen,
}

impl<TDunGen> TraversePortalsGenerator<TDunGen>
where
    TDunGen: DoesAllInstancedDunGen,
{
    /// Creates a dungeon generator that traverses portals.
    pub fn new(dun_gen: TDunGen) -> Self {
        Self { dun_gen }
    }
}

impl<TDunGen> DoesDunGen for TraversePortalsGenerator<TDunGen>
where
    TDunGen: DoesAllInstancedDunGen,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map = target.get_map_mut();
        for portal_mut in map.portals_mut() {
            let map = portal_mut.target_mut();
            self.dun_gen_placed_map(map);
            self.dun_gen.dun_gen_placed_map(map);
        }
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        for portal_mut in map.portals_mut() {
            let map = portal_mut.target_mut();
            self.dun_gen_placed_map(map);
            self.dun_gen.dun_gen_placed_map(map);
        }
    }
}

impl<TDunGen> DoesDunGenPlaced for TraversePortalsGenerator<TDunGen>
where
    TDunGen: DoesAllInstancedDunGen,
{
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced) {
        let map = target.get_placed_map_mut();
        for portal_mut in map.portals_mut() {
            let map = portal_mut.target_mut();
            self.dun_gen_placed_map(map);
            self.dun_gen.dun_gen_placed_map(map);
        }
    }

    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>) {
        for portal_mut in map.portals_mut() {
            let map = portal_mut.target_mut();
            self.dun_gen_placed_map(map);
            self.dun_gen.dun_gen_placed_map(map);
        }
    }
}

impl<TDunGen> DoesAllInstancedDunGen for TraversePortalsGenerator<TDunGen> where
    TDunGen: DoesAllInstancedDunGen
{
}