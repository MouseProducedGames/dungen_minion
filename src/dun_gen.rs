// External includes.

// Standard includes.

// Internal includes.
use super::{DoesDunGen, DoesDunGenStatic, Room, SupportsDunGen};

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
    pub fn new(map: TRoom) -> Self {
        Self {
            map,
            marker: std::marker::PhantomData::default(),
        }
    }

    pub fn build(&self) -> TRoom {
        self.map.clone()
    }

    pub fn gen<TDoesDunGenStatic>(&mut self) -> &mut Self
    where
        TDoesDunGenStatic: DoesDunGenStatic<'a>,
    {
        TDoesDunGenStatic::dun_gen_static(self);

        self
    }

    pub fn gen_with<'b, TDoesDunGen: 'b>(&mut self, with: TDoesDunGen) -> &mut Self
    where
        TDoesDunGen: DoesDunGen<'b>,
    {
        with.dun_gen(self);

        self
    }
}

impl<'a, TRoom> SupportsDunGen<'a> for DunGen<'a, TRoom>
where
    TRoom: Room<'a> + Clone,
{
    fn get_map(&self) -> &dyn Room<'a> {
        &self.map
    }

    fn get_map_mut(&mut self) -> &mut dyn Room<'a> {
        &mut self.map
    }
}
