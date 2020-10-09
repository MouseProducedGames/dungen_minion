// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// A new dungeon generator for generating dungeons on a room that has a specific position; that
/// is, a `PlacedRoom`; in this case, [`map`] #field.map.
pub struct DunGenPlaced {
    map: Box<dyn PlacedRoom>,
    marker: std::marker::PhantomData<dyn PlacedRoom>,
}

impl DunGenPlaced {
    /// Creates a new dungeon generator for generating dungeons on a room that has a specific
    /// position; that is, a `PlacedRoom`.
    pub fn new(map: Box<dyn PlacedRoom>) -> Self {
        Self {
            map,
            marker: std::marker::PhantomData::default(),
        }
    }

    /// Returns a clone of the generated [`map`] #field map. The `DunGenPlaced` instance should
    /// then be discarded.
    pub fn build(&mut self) -> Box<dyn PlacedRoom> {
        self.map.clone()
    }

    /// The `DunGenPlaced` will apply the provided `TDoesDunGenStatic` to its primary [`map`]: #field.map.
    pub fn gen<TDoesDunGenStatic>(&mut self) -> &mut Self
    where
        TDoesDunGenStatic: DoesDunGenPlacedStatic,
    {
        TDoesDunGenStatic::dun_gen_placed_static(self);

        self
    }

    /// The `DunGenPlaced` will apply the static `TDoesDunGenStatic` to its primary [`map`]: #field.map
    /// or any room on the end of a portal; provided they, themselves, do not contain any instances
    /// of `Portal`.
    pub fn gen_leaf_portals_static<TDoesDunGenStatic>(&mut self) -> &mut Self
    where
        TDoesDunGenStatic: DoesDunGenPlacedStatic,
    {
        Self::gen_leaf_portals_static_impl::<TDoesDunGenStatic>(&mut self.map);

        self
    }

    pub(crate) fn gen_leaf_portals_static_impl<TDoesDunGenStatic>(
        current_map: &mut Box<dyn PlacedRoom>,
    ) where
        TDoesDunGenStatic: DoesDunGenPlacedStatic,
    {
        if current_map.portal_count() == 0 {
            TDoesDunGenStatic::dun_gen_placed_map_static(current_map);
        } else {
            for portal in current_map.portals_mut() {
                Self::gen_leaf_portals_static_impl::<TDoesDunGenStatic>(portal.target_mut());
            }
        }
    }

    /// The `DunGenPlaced` will apply the provided `TDoesDunGenPlaced` to its primary [`map`]: #field.map
    /// or any room on the end of a portal; provided they, themselves, do not contain any instances
    /// of `Portal`.
    pub fn gen_leaf_portals_with<TDoesDunGenPlaced>(
        &mut self,
        with: &TDoesDunGenPlaced,
    ) -> &mut Self
    where
        TDoesDunGenPlaced: DoesDunGenPlaced,
    {
        Self::gen_leaf_portals_with_impl::<TDoesDunGenPlaced>(&mut self.map, with);

        self
    }

    pub(crate) fn gen_leaf_portals_with_impl<TDoesDunGenPlaced>(
        current_map: &mut Box<dyn PlacedRoom>,
        with: &TDoesDunGenPlaced,
    ) where
        TDoesDunGenPlaced: DoesDunGenPlaced,
    {
        if current_map.portal_count() == 0 {
            with.dun_gen_placed_map(current_map);
        } else {
            for portal in current_map.portals_mut() {
                DunGenPlaced::gen_leaf_portals_with_impl::<TDoesDunGenPlaced>(
                    portal.target_mut(),
                    with,
                );
            }
        }
    }

    /// The `DunGenPlaced` will apply the provided `TDoesDunGen` to its primary [`map`]: #field.map.
    pub fn gen_with<TDoesDunGen>(&mut self, with: TDoesDunGen) -> &mut Self
    where
        TDoesDunGen: DoesDunGenPlaced,
    {
        with.dun_gen_placed(self);

        self
    }
}

impl SupportsDunGenPlaced for DunGenPlaced {
    fn get_placed_map(&self) -> &Box<dyn PlacedRoom> {
        &self.map
    }

    fn get_placed_map_mut(&mut self) -> &mut Box<dyn PlacedRoom> {
        &mut self.map
    }
}
