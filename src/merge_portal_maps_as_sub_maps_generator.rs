// External includes.
use super::*;
use crate::geometry::*;

// Standard includes.

// Internal includes.

/// Merges the maps connected to this map through [`Portal`](struct.Portal.html)s as sub-maps of this map.
///
/// The following code will generate some rooms and hallways connected through portals, and then merge them as sub-maps of the central map.
///
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// for _ in 0..1_000 {
///     // We could provide CountRange directly to EdgePortalsGenerator, but that would not let us
///     // test that we have the right number of portals.
///     // This CountRange will generate a number in the range [2, 5].
///     let num_portals = CountRange::new(2, 5).provide_count();
///     let map_id =
///         DunGen::new(SparseMap::new())
///         .gen_with(SequentialGenerator::new(&[
///             &EmptyRoomGenerator::new(Size::new(12, 8)),
///             &WalledRoomGenerator::new(Size::zero()),
///             &EdgePortalsGenerator::new(
///                 num_portals,
///                 // A boxed generator which provides the `MapId`'s for the `Map`'s that will be
///                 // placed at the end of the portal.
///                 Box::new(|| SparseMap::new())
///             ),
///         ]))
///         .gen_with(TraversePortalsGenerator::new(SequentialGenerator::new(&[
///             &EmptyRoomGenerator::new(Size::new(8, 6)),
///             &WalledRoomGenerator::new(Size::zero()),
///         ])))
///         .gen_with(TraverseThisAndPortalsGenerator::new(ReciprocatePortalsGenerator::new()))
///         .gen_with(MergePortalMapsAsSubMapsGenerator::new(|portal| true))
///         .build();
///
///     let maps = MAPS.read();
///     let map = maps[map_id].read();
///
///     assert!(*map.size() == Size::new(12, 8));
///     assert!(map.portal_count() == num_portals);
///     assert!(map.portal_count() >= 2 && map.portal_count() <= 5);
///     let mut portal_count = 0;
///     for portal in map.portals() {
///         let target_map = maps[portal.target()].read();
///         assert!(*target_map.size() == Size::new(8, 6));
///         assert!(target_map.tile_type_at_local(Position::new(0, 0)) == Some(TileType::Wall));
///         assert!(target_map.tile_type_at_local(Position::new(1, 1)) == Some(TileType::Floor));
///         let target_local_position = *portal.local_position();
///         let target_other_position = *portal.portal_to_map_position();
///         println!("Target map portal count: {}", target_map.portal_count());
///         assert!(target_map.portal_count() == 1);
///         let mut other_portal_count = 0;
///         for other_portal in target_map.portals() {
///             let other_local_position = *other_portal.local_position();
///             let other_target_position = *other_portal.portal_to_map_position();
///             assert!(other_local_position == target_other_position);
///             assert!(target_local_position == other_target_position);
///             other_portal_count += 1;
///         }
///         assert!(other_portal_count == 1);
///         portal_count += 1;
///     }
///     assert!(portal_count == num_portals);
///     assert!(portal_count >= 2 && portal_count <= 5);
/// }
/// ```
pub struct MergePortalMapsAsSubMapsGenerator<TPortalFilter>
where
    TPortalFilter: Fn(&Portal) -> bool,
{
    portal_filter: TPortalFilter,
}

impl<TPortalFilter> MergePortalMapsAsSubMapsGenerator<TPortalFilter>
where
    TPortalFilter: Fn(&Portal) -> bool,
{
    /// Creates a new MergePortalMapsAsSubMapsGenerator.
    pub fn new(portal_filter: TPortalFilter) -> Self {
        Self { portal_filter }
    }
}

impl<TPortalFilter> DoesDunGen for MergePortalMapsAsSubMapsGenerator<TPortalFilter>
where
    TPortalFilter: Fn(&Portal) -> bool,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        self.dun_gen_map(map_id);
    }

    fn dun_gen_map(&self, map_id: MapId) {
        let maps = &MAPS.read();
        let map = &mut maps[map_id].write();

        let mut positions_map_ids = Vec::new();
        for portal in map.portals() {
            if (self.portal_filter)(portal) {
                let direction = *portal.portal_to_map_facing();
                let rotation = CardinalDirection::North - direction;
                let portal_map_id = portal.target();

                let portal_map_position = {
                    let portal_map = &mut maps[portal_map_id].write();
                    println!("{}", rotation);
                    // portal_map.rotate(rotation);
                    *portal.local_position()
                        + match rotation {
                            CardinalRotation::None => -(*portal.portal_to_map_position()),
                            CardinalRotation::Right90 => -(*portal.portal_to_map_position()),
                            CardinalRotation::Full180 => -(*portal.portal_to_map_position()),
                            CardinalRotation::Left90 => -(*portal.portal_to_map_position()),
                        }
                };

                /* *portal_mut.portal_to_map_position_mut() =
                 *portal_mut.portal_to_map_position() * rotation; */

                positions_map_ids.push((portal_map_position, portal_map_id));
            }
        }

        for (portal_map_position, portal_map_id) in positions_map_ids.iter() {
            map.add_sub_map(*portal_map_position, *portal_map_id);
        }
    }
}
