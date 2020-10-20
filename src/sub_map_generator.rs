// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A generator for adding one or more instances of [`SubMap`](struct.SubMap`.html) to a map.
///
/// The `SubMapGenerator` is called with a [`ProvidesCount`](geometry/trait/ProvidesCount.html) of the number of maps to be so placed; a set of local [`ProvidesPosition`](geometry/trait.ProvidesPosition.html) to provide the positions at which to place each map; and an optional boxed function to provide those maps. Each set also optionally contains a set of generators to call on those maps, and a validity check to validate the generated sub-maps. A fall-back boxed function to provide maps, should be provided; and, optionally, a set of generators to call on all generated sub-maps can be provided, as well as a validity check to validate all generated sub-maps.
///
/// Will create a map with a `Size` of 40 tiles wide by 30 tiles high, and then generate 4 to 9 sub-maps on it.
/// ```
/// # use dungen_minion::geometry::*;
/// # use dungen_minion::*;
/// for _ in 0..500 {
///     // We could provide CountRange directly to EdgePortalsGenerator, but that would not let us
///     // test that we have the right number of portals.
///     // This CountRange will generate a number in the range [4, 9].
///     let num_sub_maps = CountRange::new(4, 9).provide_count();
///     let map_id =
///         DunGen::new(SparseMap::new())
///         // We want this map to be no more than 40 tiles wide by 30 tiles high.
///         .gen_with(EmptyRoomGenerator::new(Size::new(40, 30)))
///         .gen_with(WalledRoomGenerator::new(Size::zero()))
///         .gen_with(SubMapGenerator::new(
///             &[SubMapGeneratorSet::new(
///                 &num_sub_maps,
///                 // By restricting the position slightly, we unsure that the *minimum* area of
///                 // the generated sub-maps does not exceed the bounds we set.
///                 &Area::new(Position::new(0, 0), Size::new(34, 26)),
///                 Some(Box::new(SparseMap::new)),
///                 Some(&[&EmptyRoomGenerator::new(SizeRange::new(
///                     Size::new(6, 6),
///                     Size::new(12, 12),
///                 ))]),
///                 None,
///             )],
///             Some(Box::new(SparseMap::new)),
///             Some(&[&WalledRoomGenerator::new(Size::zero())]),
///             Some(|position: Position, map_id: MapId| {
///                 let maps = &MAPS.read();
///                 let map = &maps[map_id].read();
///                 let mut area = *map.area();
///                 *area.position_mut() = *area.position() + position;
///                 println!("{}", map.area());
///                 !(
///                     area.left() < 0 ||
///                     area.top() < 0 ||
///                     area.right() >= 40 ||
///                     area.bottom() >= 30
///                 )
///             }),
///         ))
///         .build();
///
///     let maps = MAPS.read();
///     let map = maps[map_id].read();
///
///     println!("Map area: {}", map.area());
///     assert!(*map.size() == Size::new(40, 30));
///     assert!(map.sub_map_count() == num_sub_maps);
///     assert!(map.sub_map_count() >= 4 && map.sub_map_count() <= 9);
///     let mut sub_map_count = 0;
///     for sub_map in map.sub_maps() {
///         let target_map = maps[sub_map.value()].read();
///         assert!(target_map.size().width() >= 6 && target_map.size().width() <= 12);
///         assert!(target_map.size().height() >= 6 && target_map.size().height() <= 12);
///         assert!(target_map.tile_type_at_local(Position::new(0, 0)) == Some(TileType::Wall));
///         assert!(target_map.tile_type_at_local(Position::new(1, 1)) == Some(TileType::Floor));
///         sub_map_count += 1;
///     }
///     assert!(sub_map_count == num_sub_maps);
///     assert!(sub_map_count >= 4 && sub_map_count <= 9);
/// }
/// ```
pub struct SubMapGenerator<'a, TValidityCheck>
where
    TValidityCheck: Fn(Position, MapId) -> bool,
{
    sub_maps_generator_sets: &'a [SubMapGeneratorSet<'a>],
    fallback_map_provider: Option<Box<dyn Fn() -> MapId>>,
    global_sub_map_generators: Option<&'a [&'a dyn DoesDunGen]>,
    validity_check: Option<TValidityCheck>,
}

/// Contains information about generating a sub-map for SubMapGenerator.
pub struct SubMapGeneratorSet<'a> {
    provides_count: &'a dyn ProvidesCount,
    provides_position: &'a dyn ProvidesPosition,
    provides_map: Option<Box<dyn Fn() -> MapId>>,
    sub_maps_generators: Option<&'a [&'a dyn DoesDunGen]>,
    validity_check: Option<Box<dyn Fn(Position, MapId) -> bool>>,
}

impl<'a> SubMapGeneratorSet<'a> {
    /// Creates a struct which contains information about generating a sub-map for SubMapGenerator.
    pub fn new(
        provides_count: &'a dyn ProvidesCount,
        provides_position: &'a dyn ProvidesPosition,
        provides_map: Option<Box<dyn Fn() -> MapId>>,
        sub_maps_generators: Option<&'a [&'a dyn DoesDunGen]>,
        validity_check: Option<Box<dyn Fn(Position, MapId) -> bool>>,
    ) -> Self {
        Self {
            provides_count,
            provides_position,
            provides_map,
            sub_maps_generators,
            validity_check,
        }
    }
}

impl<'a, TValidityCheck> SubMapGenerator<'a, TValidityCheck>
where
    TValidityCheck: Fn(Position, MapId) -> bool,
{
    /// Creates a new generator for adding portals to a map.
    pub fn new(
        sub_maps_generator_sets: &'a [SubMapGeneratorSet<'a>],
        fallback_map_provider: Option<Box<dyn Fn() -> MapId>>,
        global_sub_map_generators: Option<&'a [&'a dyn DoesDunGen]>,
        validity_check: Option<TValidityCheck>,
    ) -> Self {
        Self {
            sub_maps_generator_sets,
            fallback_map_provider,
            global_sub_map_generators,
            validity_check,
        }
    }
}

impl<'a, TValidityCheck> DoesDunGen for SubMapGenerator<'a, TValidityCheck>
where
    TValidityCheck: Fn(Position, MapId) -> bool,
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        self.dun_gen_map(map_id);
    }

    fn dun_gen_map(&self, map_id: MapId) {
        for sub_maps_generator_set in self.sub_maps_generator_sets {
            let (
                provides_count,
                provides_position,
                map_provider,
                sub_map_generators,
                validity_check,
            ) = (
                sub_maps_generator_set.provides_count,
                sub_maps_generator_set.provides_position,
                &sub_maps_generator_set.provides_map,
                sub_maps_generator_set.sub_maps_generators,
                sub_maps_generator_set.validity_check.as_ref(),
            );

            let count = provides_count.provide_count();
            for _ in 0..count {
                let (mut position, mut new_map_id);
                loop {
                    position = provides_position.provide_position();
                    new_map_id = if let Some(map_provider) = map_provider {
                        map_provider()
                    } else if let Some(map_provider) = &self.fallback_map_provider {
                        map_provider()
                    } else {
                        panic!("Could not find map provider to generate map with.");
                    };

                    // *MAPS.read()[new_map_id].write().position_mut() = position;

                    if let Some(sub_map_generators) = sub_map_generators {
                        for sub_map_generator in sub_map_generators {
                            sub_map_generator.dun_gen_map(new_map_id);
                        }
                    }

                    if let Some(sub_map_generators) = self.global_sub_map_generators {
                        for sub_map_generator in sub_map_generators {
                            sub_map_generator.dun_gen_map(new_map_id);
                        }
                    }

                    if validity_check.is_none() && self.validity_check.is_none() {
                        break;
                    }

                    if let Some(validity_check) = validity_check {
                        if !validity_check(position, new_map_id) {
                            invalidate_map(new_map_id)
                        } else {
                            break;
                        }
                    }

                    if let Some(validity_check) = &self.validity_check {
                        if !validity_check(position, new_map_id) {
                            invalidate_map(new_map_id)
                        } else {
                            break;
                        }
                    }
                }

                MAPS.read()[map_id]
                    .write()
                    .add_sub_map(position, new_map_id);
            }
        }
    }
}
