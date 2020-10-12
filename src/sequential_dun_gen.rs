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
///             5,
///             Box::new(|| {
///                 Box::new(PlacedRoomWrapper::new(
///                     Position::new(0, 0),
///                     RoomHashMap::default(),
///                 ))
///             }),
///         )
///     ]))
///     .gen_leaf_portals_with(&SequentialDunGen::new(&[
///         &EmptyRoomDunGen::new(Size::new(3, 10)),
///         &WalledRoomDunGen::new(Size::new(3, 10)),
///     ]))
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
pub struct SequentialDunGen<'a> {
    dun_gens: &'a [&'a (dyn DoesAllInstancedDunGen)],
}

impl<'a> SequentialDunGen<'a> {
    /// Creates a new sequential set of dungeon generators.
    pub fn new(dun_gens: &'a [&'a (dyn DoesAllInstancedDunGen)]) -> Self {
        Self { dun_gens }
    }
}

impl<'a> DoesDunGen for SequentialDunGen<'a> {
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        for dun_gen in self.dun_gens {
            dun_gen.dun_gen(target);
        }
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        for dun_gen in self.dun_gens {
            dun_gen.dun_gen_map(map);
        }
    }
}

impl<'a> DoesDunGenPlaced for SequentialDunGen<'a> {
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced) {
        for dun_gen in self.dun_gens {
            dun_gen.dun_gen_placed(target);
        }
    }

    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>) {
        for dun_gen in self.dun_gens {
            dun_gen.dun_gen_placed_map(map);
        }
    }
}

impl<'a> DoesAllInstancedDunGen for SequentialDunGen<'a> {}
