// External includes.

// Standard includes.

// Internal includes.
use super::Room;
use crate::traits::{DoesDunGen, DoesDunGenStatic, SupportsDunGen};

pub struct DunGen<'a, TRoom>
where
    TRoom: Room<'a> + Clone,
{
    map: TRoom,
    marker: std::marker::PhantomData<dyn Room<'a>>,
}

impl<'a, TRoom> DunGen<'a, TRoom>
where
    TRoom: Room<'a> + Clone,
{
    pub fn build(&mut self) -> TRoom {
        self.map.clone()
    }

    pub fn gen<TDoesDunGenStatic>(&'a mut self)
    where
        TDoesDunGenStatic: DoesDunGenStatic<'a>,
    {
        TDoesDunGenStatic::dun_gen_static(self)
    }

    pub fn gen_with<TDoesDunGen>(&'a mut self, with: &'a TDoesDunGen)
    where
        TDoesDunGen: DoesDunGen<'a>,
    {
        with.dun_gen(self)
    }
}

impl<'a, TRoom> SupportsDunGen<'a> for DunGen<'a, TRoom>
where
    TRoom: Room<'a> + Clone,
{
    fn get_map(&'a self) -> &'a dyn Room<'a> {
        &self.map
    }

    fn get_map_mut(&'a mut self) -> &'a mut dyn Room<'a> {
        &mut self.map
    }
}
