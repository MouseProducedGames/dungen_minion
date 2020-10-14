// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// Used to execute a dungeon generator sequentially on both the current map, and through its portals.
///
/// The following chain will generate a room with a [`Size`](geometry/struct.Size.html) of 12 tiles wide by 8 tiles high (including walls), and then add 5 randomly-placed hallways projecting off of it.
///```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map =
///     DunGen::new(Box::new(RoomHashMap::new()))
///     .gen_with(SequentialGenerator::new(&[
///         &EmptyRoomGenerator::new(Size::new(12, 8)),
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
///     .gen_with(TraversePortalsGenerator::new(EmptyRoomGenerator::new(Size::new(3, 10))))
///     .gen_with(TraverseThisAndPortalsGenerator::new(WalledRoomGenerator::new(Size::zero())))
///     .build();
///
/// assert!(*map.size() == Size::new(12, 8));
///
/// assert!(map.tile_type_at_local(ShapePosition::new(0, 0)) == Some(&TileType::Wall));
/// assert!(map.tile_type_at_local(ShapePosition::new(1, 1)) == Some(&TileType::Floor));
///
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
pub struct TraverseThisAndPortalsGenerator<TDunGen>
where
    TDunGen: DoesDunGen,
{
    dun_gen: TDunGen,
}

impl<TDunGen> TraverseThisAndPortalsGenerator<TDunGen>
where
    TDunGen: DoesDunGen,
{
    /// Creates a dungeon generator that traverses the current map and portals.
    pub fn new(dun_gen: TDunGen) -> Self {
        Self { dun_gen }
    }
}

impl<TDunGen> DoesDunGen for TraverseThisAndPortalsGenerator<TDunGen>
where
    TDunGen: DoesDunGen,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let mut target_map_ids = Vec::new();
        {
            self.dun_gen.dun_gen(target);
            let maps = &MAPS.read()[target.get_map_id()];
            let map = &maps.read();
            for portal in map.portals() {
                let map_id = portal.target();
                target_map_ids.push(map_id);
            }
        }

        let target_map_ids = target_map_ids;
        for target_map_id in target_map_ids {
            self.dun_gen.dun_gen_map(target_map_id);
        }
    }

    fn dun_gen_map(&self, map_id: MapId) {
        let mut target_map_ids = Vec::new();
        {
            self.dun_gen.dun_gen_map(map_id);
            let maps = &MAPS.read()[map_id];
            let map = &maps.read();
            for portal in map.portals() {
                let map_id = portal.target();
                target_map_ids.push(map_id);
            }
        }

        for target_map_id in target_map_ids {
            self.dun_gen.dun_gen_map(target_map_id);
        }
    }
}
