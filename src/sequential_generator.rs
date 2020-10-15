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
/// let map_id =
///     DunGen::new(MapSparse::new())
///     .gen_with(SequentialGenerator::new(&[
///         &EmptyRoomGenerator::new(Size::new(12, 8)),
///         &WalledRoomGenerator::new(Size::zero()),
///         &EdgePortalsGenerator::new(
///             5,
///             Box::new(|| {
///                 MapSparse::new()
///             }),
///         )
///     ]))
///     .gen_with(TraversePortalsGenerator::new(SequentialGenerator::new(&[
///         &EmptyRoomGenerator::new(Size::new(3, 10)),
///         &WalledRoomGenerator::new(Size::zero()),
///     ])))
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
pub struct SequentialGenerator<'a> {
    dun_gens: &'a [&'a (dyn DoesDunGen)],
}

impl<'a> SequentialGenerator<'a> {
    /// Creates a new sequential set of dungeon generators.
    pub fn new(dun_gens: &'a [&'a (dyn DoesDunGen)]) -> Self {
        Self { dun_gens }
    }
}

impl<'a> DoesDunGen for SequentialGenerator<'a> {
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        for dun_gen in self.dun_gens {
            dun_gen.dun_gen(target);
        }
    }

    fn dun_gen_map(&self, map_id: MapId) {
        for dun_gen in self.dun_gens {
            dun_gen.dun_gen_map(map_id);
        }
    }
}
