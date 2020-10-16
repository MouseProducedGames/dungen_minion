// External includes.

// Standard includes.

// Internal includes.
use super::*;

/// A new dungeon generator for generating dungeons based on a starting [`Map`](trait.Map.html).
pub struct DunGen {
    map_id: MapId,
}

impl DunGen {
    /// Creates a new dungeon generator for generating dungeons based on a starting [`Map`](trait.Map.html) implementation.
    ///
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map_id =
    ///     // The new DunGen generator is created, and given a primary map.
    ///     DunGen::new(SparseMap::new())
    ///     // Call generation methods, giving them appropriate generators.
    ///     .build();
    ///```
    pub fn new(map_id: MapId) -> Self {
        Self { map_id }
    }

    /// Returns the `MapId` of the generated [`Map`](trait.Map.html) implementation.
    ///
    /// After the map has been generated, the `DunGen` instance can be safely discarded.
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map_id =
    ///     DunGen::new(SparseMap::new())
    ///     .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
    ///     .gen_with(WalledRoomGenerator::new(Size::zero()))
    ///     // At this point, the generator will return a walled map 8 tiles wide by 6 tiles high.
    ///     .build();
    ///
    /// let maps = MAPS.read();
    /// let map = maps[map_id].read();
    /// assert!(*map.size() == Size::new(8, 6));
    ///```
    pub fn build(&mut self) -> MapId {
        self.map_id
    }

    /// The `DunGenPlaced` will apply the provided `TDoesDunGen` to its primary map.
    ///
    /// The following chain will generate a map with a [`Size`](geometry/struct.Size.html) of 8 tiles wide by 6 tiles high, with no remainder.
    ///```
    /// # use dungen_minion::geometry::*;
    /// # use dungen_minion::*;
    /// let map_id =
    ///     DunGen::new(SparseMap::new())
    ///     // EmptyRoomGenerator is called as an instance, as it needs information about how large a
    ///     // map to generate.
    ///     .gen_with(EmptyRoomGenerator::new(Size::new(8, 6)))
    ///     .build();
    ///
    /// let maps = MAPS.read();
    /// let map = maps[map_id].read();
    ///
    /// assert!(*map.size() == Size::new(8, 6));
    /// assert!(map.tile_type_at_local(Position::new(0, 0)) == Some(TileType::Floor));
    /// assert!(map.tile_type_at_local(Position::new(1, 1)) == Some(TileType::Floor));
    /// assert!(map.portal_count() == 0);
    /// let mut count = 0;
    /// for portal in map.portals() {
    ///     // Test will error out if it enters this loop (ie., any portals exist).
    ///     assert!(false);
    ///     count += 1;
    /// }
    /// assert!(count == 0);
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
    fn get_map_id(&self) -> MapId {
        self.map_id
    }
}
