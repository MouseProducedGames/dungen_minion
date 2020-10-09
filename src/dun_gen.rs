// External includes.

// Standard includes.

// Internal includes.
use super::*;

pub struct DunGen {
    map: Box<dyn Room>,
    marker: std::marker::PhantomData<dyn Room>,
}

impl DunGen {
    pub fn new(map: Box<dyn Room>) -> Self {
        Self {
            map,
            marker: std::marker::PhantomData::default(),
        }
    }

    pub fn build(&mut self) -> Box<dyn Room> {
        self.map.clone()
    }

    pub fn gen<TDoesDunGenStatic>(&mut self) -> &mut Self
    where
        TDoesDunGenStatic: DoesDunGenStatic,
    {
        TDoesDunGenStatic::dun_gen_static(self);

        self
    }

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
