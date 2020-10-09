// External includes.

// Standard includes.

// Internal includes.
use super::*;

pub struct DunGenPlaced {
    map: Box<dyn PlacedRoom>,
    marker: std::marker::PhantomData<dyn PlacedRoom>,
}

impl DunGenPlaced {
    pub fn new(map: Box<dyn PlacedRoom>) -> Self {
        Self {
            map,
            marker: std::marker::PhantomData::default(),
        }
    }

    pub fn build(&mut self) -> Box<dyn PlacedRoom> {
        self.map.clone()
    }

    pub fn gen<TDoesDunGenStatic>(&mut self) -> &mut Self
    where
        TDoesDunGenStatic: DoesDunGenPlacedStatic,
    {
        TDoesDunGenStatic::dun_gen_placed_static(self);

        self
    }

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
