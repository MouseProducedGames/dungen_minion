// External includes.

// Standard includes.
use std::collections::HashSet;
use std::sync::RwLock;

// Internal includes.
use super::*;

/// Used to conditionally execute a dungeon generator.
///
/// The following chain will generate a room with a [`Size`](geometry/struct.Size.html) of 12 tiles wide by 8 tiles high (including walls), and then add 1 randomly-placed room and 4 randomly-placed hallways projecting off of it.
///```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// let map =
///     DunGen::new(Box::new(RoomHashMap::new()))
///     .gen_with(SequentialGenerator::new(&[
///         &EmptyRoomGenerator::new(Size::new(12, 8)),
///         &WalledRoomGenerator::new(Size::zero()),
///         &EdgePortalsGenerator::new(
///             1,
///             Box::new(|| {
///                 Box::new(PlacedRoomWrapper::new(
///                     Position::new(0, 0),
///                     RoomHashMap::default(),
///                 ))
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
///                 Box::new(PlacedRoomWrapper::new(
///                     Position::new(0, 0),
///                     RoomHashMap::default(),
///                 ))
///             }),
///     ))
///     // The if-check ensures that we only generate on maps that haven't already been generated.
///     .gen_with(TraversePortalsGenerator::new(IfMapThenGenerator::new(
///         |map| *map.size() == Size::zero(),
///         |placed_map| *placed_map.size() == Size::zero(),
///         SequentialGenerator::new(&[
///             &EmptyRoomGenerator::new(Size::new(3, 10)),
///             &WalledRoomGenerator::new(Size::zero()),
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
pub struct VisitMapOnceGenerator<TDunGen>
where
    TDunGen: DoesDunGen,
{
    visited_maps: RwLock<HashSet<MapId>>,
    dun_gen: TDunGen,
}

impl<TDunGen> VisitMapOnceGenerator<TDunGen>
where
    TDunGen: DoesDunGen,
{
    /// Creates a new conditional dungeon generator.
    pub fn new(dun_gen: TDunGen) -> Self {
        Self {
            visited_maps: RwLock::new(HashSet::new()),
            dun_gen,
        }
    }
}

impl<TDunGen> DoesDunGen for VisitMapOnceGenerator<TDunGen>
where
    TDunGen: DoesDunGen,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        {
            let mut visited_maps = self.visited_maps.write().unwrap();
            if visited_maps.contains(&target.get_map_id()) {
                return;
            }

            visited_maps.insert(target.get_map_id());
        }
        self.dun_gen.dun_gen(target);
    }

    fn dun_gen_map(&self, map_id: MapId) {
        {
            let mut visited_maps = self.visited_maps.write().unwrap();
            if visited_maps.contains(&map_id) {
                return;
            }

            visited_maps.insert(map_id);
        }
        self.dun_gen.dun_gen_map(map_id);
    }
}
