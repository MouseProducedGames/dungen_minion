#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! A dungeon generation library focused on 2D roguelikes.
//!
//! `dungen_minion` is in the very early release stage, with most features being new or unstable.
//! It is not (yet!) recommend for general use, although you can test it if you're feeling really
//! adventurous.
//!
//! ```
//! use dungen_minion::geometry::*;
//! use dungen_minion::*;
//! // Create a dungeon generator using RoomHashMap.
//! // RoomHashMap is expandable, and has no explicit size restrictions.
//! let dungen = DunGen::new(Box::new(RoomHashMap::default()))
//!     // Expand the room to a width of 40, and a height of 30.
//!     .gen_with(EmptyRoomGenerator::new(SizeRange::new(Size::new(24, 18), Size::new(40, 30))))
//!     // TileType::Floor will be placed.
//!     // You may also give it a SizeRange to generate a randomly-sized room.
//!     // .gen_with(EmptyRoomGenerator::new(SizeRange::new(Size::new(24, 18), Size::new(40, 30))))
//!     // Create walls for the room.
//!     .gen::<WalledRoomGenerator::<Size>>()
//!     .build();
//!
//! // A simple drawing routine.
//! for y in 0..dungen.size().height() {
//!     for x in 0..dungen.size().width() {
//!         let tile_type = dungen.tile_type_at_local(ShapePosition::new(x as i32, y as i32));
//!         if tile_type.is_none() {
//!             continue;
//!         }
//!
//!         // The selection of tiles is deliberately limited, for now.
//!         // Theming is included in future plans for dungen_minion.
//!         let tile_type = tile_type.unwrap();
//!         let ch = match tile_type {
//!             TileType::Void => ' ',
//!             TileType::Floor => '.',
//!             TileType::Wall => '#',
//!             TileType::Portal => '+',
//!         };
//!
//!         print!("{}", ch);
//!     }
//!     println!();
//! }
//! ```
// External includes.
pub use dungen_minion_rooms::*;

// Standard includes.

// Internal includes.
mod dun_gen;
mod dun_gen_placed;
mod edge_portals_generator;
mod empty_room_generator;
mod fill_tiles_generator;
mod if_map_then_generator;
mod sequential_generator;
mod traverse_portals_generator;
mod traverse_this_and_portals_generator;
mod walled_room_generator;

pub use dun_gen::DunGen;
pub use dun_gen_placed::DunGenPlaced;
pub use edge_portals_generator::EdgePortalsGenerator;
pub use empty_room_generator::EmptyRoomGenerator;
pub use fill_tiles_generator::FillTilesGenerator;
pub use if_map_then_generator::IfMapThenGenerator;
pub use sequential_generator::SequentialGenerator;
pub use traverse_portals_generator::TraversePortalsGenerator;
pub use traverse_this_and_portals_generator::TraverseThisAndPortalsGenerator;
pub use walled_room_generator::WalledRoomGenerator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
