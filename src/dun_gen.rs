// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// A new dungeon generator for generating dungeons based on a starting [`Room`](trait.Room.html).
pub struct DunGen {
    map_id: MapId,
}

impl DunGen {
    /// Creates a new dungeon generator for generating dungeons based on a starting boxed `Room`.
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
    pub fn new(map_id: MapId) -> Self {
        Self { map_id }
    }

    /// Returns a boxed clone of the generated `Room`.
    ///
    /// After the dungeon has been generated, the `DunGen` instance can be safely discarded.
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map =
    ///     DunGen::new(Box::new(RoomHashMap::new()))
    ///     .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
    ///     .gen::<WalledRoomGenerator::<Size>>()
    ///     // At this point, the generator will return a walled room 8 tiles wide by 6 tiles high.
    ///     .build();
    ///
    /// assert!(*map.size() == Size::new(8, 6));
    ///```
    pub fn build(&mut self) -> MapId {
        self.map_id.clone()
    }

    /// The `DunGenPlaced` will apply the provided `TDoesDunGen` to its primary map.
    ///
    /// The following chain will generate a room with a [`Size`](geometry/struct.Size.html) of 8 tiles wide by 6 tiles high, with no remainder.
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map =
    ///     DunGen::new(Box::new(RoomHashMap::new()))
    ///     // EmptyRoomGenerator is called as an instance, as it needs information about how large a
    ///     // room to generate.
    ///     .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
    ///     .build();
    ///
    /// assert!(*map.size() == Size::new(8, 6));
    /// assert!(map.tile_type_at_local(ShapePosition::new(0, 0)) == Some(&TileType::Floor));
    /// assert!(map.tile_type_at_local(ShapePosition::new(1, 1)) == Some(&TileType::Floor));
    /// assert!(map.portal_count() == 0);
    /// let mut count = 0;
    /// for portal in map.portals() {
    ///     // Test will error out if it enters this loop (ie., any portals exist).
    ///     assert!(false);
    ///     count += 1;
    /// }
    /// assert!(count == 0);
    ///```
    pub fn gen_with<TDoesDunGen>(&mut self, mut with: TDoesDunGen) -> &mut Self
    where
        TDoesDunGen: DoesDunGen,
    {
        with.dun_gen(self);

        self
    }
}

impl SupportsDunGen for DunGen {
    fn get_map_id(&self) -> MapId {
        self.map_id
    }
}
