// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// A new dungeon generator for generating dungeons on a `Room`; in this case, [`map`] #field.map.
pub struct DunGen {
    map: Box<dyn Room>,
    marker: std::marker::PhantomData<dyn Room>,
}

impl DunGen {
    /// Creates a new dungeon generator for generating dungeons on a `Room`.
    pub fn new(map: Box<dyn Room>) -> Self {
        Self {
            map,
            marker: std::marker::PhantomData::default(),
        }
    }

    /// Returns a clone of the generated [`map`] #field.map. The `DunGen` instance should
    /// then be discarded.
    pub fn build(&mut self) -> Box<dyn Room> {
        self.map.clone()
    }

    /// The `DunGen` will apply the provided `TDoesDunGenStatic` to its primary [`map`]: #field.map.
    pub fn gen<TDoesDunGenStatic>(&mut self) -> &mut Self
    where
        TDoesDunGenStatic: DoesDunGenStatic,
    {
        TDoesDunGenStatic::dun_gen_static(self);

        self
    }

    /// The `DunGen` will apply the static `TDoesDunGenStatic` to its primary [`map`]: #field.map
    /// or any room on the end of a portal; provided they, themselves, do not contain any instances
    /// of `Portal`. `TDoesDunGenStatic` must also implement `DoesDunGenPlacedStatic`.
    pub fn gen_leaf_portals_static<TDoesDunGenStatic>(&mut self) -> &mut Self
    where
        TDoesDunGenStatic: DoesDunGenStatic + DoesDunGenPlacedStatic,
    {
        if self.map.portal_count() == 0 {
            TDoesDunGenStatic::dun_gen_static(self);
        } else {
            for portal in self.map.portals_mut() {
                DunGenPlaced::gen_leaf_portals_static_impl::<TDoesDunGenStatic>(
                    portal.target_mut(),
                );
            }
        }

        self
    }

    /// The `DunGenPlaced` will apply the provided `TDoesDunGen` to its primary [`map`]: #field.map
    /// or any room on the end of a portal; provided they, themselves, do not contain any instances
    /// of `Portal`. `TDoesDunGen` must also implement `DoesDunGenPlaced`.
    pub fn gen_leaf_portals_with<TDoesDunGen>(&mut self, with: &TDoesDunGen) -> &mut Self
    where
        TDoesDunGen: DoesDunGen + DoesDunGenPlaced,
    {
        if self.map.portal_count() == 0 {
            with.dun_gen(self);
        } else {
            for portal in self.map.portals_mut() {
                DunGenPlaced::gen_leaf_portals_with_impl::<TDoesDunGen>(portal.target_mut(), with);
            }
        }

        self
    }

    /// The `DunGenPlaced` will apply the provided `TDoesDunGen` to its primary [`map`]: #field.map.
    pub fn gen_with<TDoesDunGen>(&mut self, with: TDoesDunGen) -> &mut Self
    where
        TDoesDunGen: DoesDunGen,
    {
        with.dun_gen(self);

        self
    }
}

impl SupportsDunGen for DunGen {
    fn get_map(&self) -> &Box<dyn Room> {
        &self.map
    }

    fn get_map_mut(&mut self) -> &mut Box<dyn Room> {
        &mut self.map
    }
}
