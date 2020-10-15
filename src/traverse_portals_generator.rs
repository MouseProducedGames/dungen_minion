// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// Used to execute a dungeon generator by traversing portals.
///
/// The following chain will generate a map with a [`Size`](geometry/struct.Size.html) of 12 tiles wide by 8 tiles high (including walls), and then add 5 randomly-placed hallways projecting off of it.
///```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map_id =
///     DunGen::new(SparseMap::new())
///     .gen_with(EmptyRoomGenerator::new(Size::new(12, 8)))
///     .gen_with(EmptyRoomGenerator::new(Size::new(12, 8)))
///     .gen_with(EdgePortalsGenerator::new(
///         5,
///         Box::new(|| {
///             SparseMap::new()
///         }),
///     ))
///     .gen_with(TraversePortalsGenerator::new(EmptyRoomGenerator::new(Size::new(3, 10))))
///     .gen_with(TraversePortalsGenerator::new(WalledRoomGenerator::new(Size::zero())))
///     .build();
///
/// let maps = MAPS.read();
/// let map = maps[map_id].read();
///
/// assert!(*map.size() == Size::new(12, 8));
/// assert!(map.portal_count() == 5);
/// let mut count = 0;
/// for portal in map.portals() {
///     let target_map = maps[portal.target()].read();
///     assert!(*target_map.size() == Size::new(3, 10));
///     assert!(target_map.tile_type_at_local(Position::new(0, 0)) == Some(&TileType::Wall));
///     assert!(target_map.tile_type_at_local(Position::new(1, 1)) == Some(&TileType::Floor));
///     count += 1;
/// }
/// assert!(count == 5);
///```
pub struct TraversePortalsGenerator<TDunGen>
where
    TDunGen: DoesDunGen,
{
    dun_gen: TDunGen,
}

impl<TDunGen> TraversePortalsGenerator<TDunGen>
where
    TDunGen: DoesDunGen,
{
    /// Creates a dungeon generator that traverses portals.
    pub fn new(dun_gen: TDunGen) -> Self {
        Self { dun_gen }
    }
}

impl<TDunGen> DoesDunGen for TraversePortalsGenerator<TDunGen>
where
    TDunGen: DoesDunGen,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let mut target_map_ids = Vec::new();
        {
            let maps = &MAPS.read()[target.get_map_id()];
            let map = &maps.read();
            for portal in map.portals() {
                let target_map_id = portal.target();
                target_map_ids.push(target_map_id);
            }
        }

        for target_map_id in target_map_ids {
            self.dun_gen_map(target_map_id);
            self.dun_gen.dun_gen_map(target_map_id);
        }
    }

    fn dun_gen_map(&self, map_id: MapId) {
        let mut target_map_ids = Vec::new();
        {
            let maps = &MAPS.read()[map_id];
            let map = &maps.read();
            for portal in map.portals() {
                let target_map_id = portal.target();
                target_map_ids.push(target_map_id);
            }
        }

        for target_map_id in target_map_ids {
            self.dun_gen_map(target_map_id);
            self.dun_gen.dun_gen_map(target_map_id);
        }
    }
}
