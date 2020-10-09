// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// A new dungeon generator for generating dungeons based on a starting [`Room`](trait.Room.html).
pub struct DunGen {
    map: Box<dyn Room>,
    marker: std::marker::PhantomData<dyn Room>,
}

impl DunGen {
    /// Creates a new dungeon generator for generating dungeons based on a starting [`Room`](trait.Room.html).
    ///
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map =
    ///     // The new DunGen generator is created, and given a primary room.
    ///     DunGen::new(Box::new(RoomHashMap::new()))
    ///     // Call generation methods, giving them appropriate generators.
    ///     .build();
    ///```
    pub fn new(map: Box<dyn Room>) -> Self {
        Self {
            map,
            marker: std::marker::PhantomData::default(),
        }
    }

    /// Returns a clone of the generated [`Room`](trait.Room.html).
    ///
    /// After the dungeon has been generated, the `DunGen` instance can be safely discarded.
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map =
    ///     DunGen::new(Box::new(RoomHashMap::new()))
    ///     .gen_with(EmptyRoomDunGen::new(Size::new(8, 6)))
    ///     .gen::<WalledRoomDunGen>()
    ///     // At this point, the generator will return a walled room 8 tiles wide by 6 tiles high.
    ///     .build();
    ///```
    pub fn build(&mut self) -> Box<dyn Room> {
        self.map.clone()
    }

    /// The `DunGen` will apply the provided `TDoesDunGenStatic` to its primary map.
    ///
    /// The given generator chain will craete a `Room` 8 tiles wide and 6 tiles high, including walls.
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map =
    ///     DunGen::new(Box::new(RoomHashMap::new()))
    ///     .gen_with(EmptyRoomDunGen::new(Size::new(8, 6)))
    ///     // WalledRoomDunGen can be called statically, as it can take its `Size` from the `Room`
    ///     // it is called on.
    ///     .gen::<WalledRoomDunGen>()
    ///     // Other generaton.
    ///     .build();
    ///```
    pub fn gen<TDoesDunGenStatic>(&mut self) -> &mut Self
    where
        TDoesDunGenStatic: DoesDunGenStatic,
    {
        TDoesDunGenStatic::dun_gen_static(self);

        self
    }

    /// The `DunGen` will apply the static `TDoesDunGenStatic` to its primary map or any room on the end of a portal; provided they, themselves, do not contain any instances of `Portal`.
    ///
    /// The following chain will generate a room 12 tiles wide by 8 tiles high (including walls), and then placed 5 random hallways projecting off of it.
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map =
    ///     DunGen::new(Box::new(RoomHashMap::new()))
    ///     .gen_with(EmptyRoomDunGen::new(Size::new(12, 8)))
    ///     .gen::<WalledRoomDunGen>()
    ///     .gen_leaf_portals_with(&EdgePortalsDunGen::new(
    ///         5,
    ///         Box::new(|| {
    ///             Box::new(PlacedRoomWrapper::new(
    ///                 Position::new(0, 0),
    ///                 RoomHashMap::default(),
    ///             ))
    ///         }),
    ///     ))
    ///     .gen_leaf_portals_with::<EmptyRoomDunGen>(&EmptyRoomDunGen::new(Size::new(3, 10)))
    ///     // Information does not need to be provided to the WalledRoomDunGen at this point, as
    ///     // it can take its Size information from the maps it is called on.
    ///     .gen_leaf_portals_static::<WalledRoomDunGen>()
    ///     .build();
    ///```
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

    /// The `DunGenPlaced` will apply the provided `TDoesDunGen` to its primary map
    /// or any room on the end of a portal; provided they, themselves, do not contain any instances
    /// of `Portal`.
    ///
    /// The following chain will generate a room 12 tiles wide by 8 tiles high (including walls), and then placed 5 random hallways projecting off of it.
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map =
    ///     DunGen::new(Box::new(RoomHashMap::new()))
    ///     .gen_with(EmptyRoomDunGen::new(Size::new(12, 8)))
    ///     .gen::<WalledRoomDunGen>()
    ///     // EdgePortalsDunGen is called as an instance, as it needs information about how many
    ///     // portals to generate, and a function that generates new boxed `PlacedRoom` instances
    ///     // to place at the end of portals.
    ///     // Since the primary room does not (yet!) have any portals, it will have portals added
    ///     // to it.
    ///     .gen_leaf_portals_with(&EdgePortalsDunGen::new(
    ///         5,
    ///         Box::new(|| {
    ///             Box::new(PlacedRoomWrapper::new(
    ///                 Position::new(0, 0),
    ///                 RoomHashMap::default(),
    ///             ))
    ///         }),
    ///     ))
    ///     // Since the added rooms do not yet have portals (nor a size), they will be given a
    ///     // size of 3 tiles wide by 10 tiles long. We don't need to worry about the rotation of
    ///     // the generated rooms - that's entirely handled through Portal and EdgePortalsDunGen.
    ///     .gen_leaf_portals_with::<EmptyRoomDunGen>(&EmptyRoomDunGen::new(Size::new(3, 10)))
    ///     .gen_leaf_portals_static::<WalledRoomDunGen>()
    ///     .build();
    ///```
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

    /// The `DunGenPlaced` will apply the provided `TDoesDunGen` to its primary map.
    ///
    /// The following chain will generate a room 12 tiles wide by 8 tiles high (including walls), and then place 5 random hallways projecting off of it.
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map =
    ///     DunGen::new(Box::new(RoomHashMap::new()))
    ///     // EmptyRoomDunGen is called as an instance, as it needs information about how large a
    ///     // room to generate.
    ///     .gen_with(EmptyRoomDunGen::new(Size::new(12, 8)))
    ///     .gen::<WalledRoomDunGen>()
    ///     .gen_leaf_portals_with(&EdgePortalsDunGen::new(
    ///         5,
    ///         Box::new(|| {
    ///             Box::new(PlacedRoomWrapper::new(
    ///                 Position::new(0, 0),
    ///                 RoomHashMap::default(),
    ///             ))
    ///         }),
    ///     ))
    ///     .gen_leaf_portals_with::<EmptyRoomDunGen>(&EmptyRoomDunGen::new(Size::new(3, 10)))
    ///     .gen_leaf_portals_static::<WalledRoomDunGen>()
    ///     .build();
    ///```
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
