// External includes.
pub use dungen_minion_rooms::*;

// Standard includes.

// Internal includes.
mod dun_gen;
mod dun_gen_placed;
mod edge_portals_dun_gen;
mod empty_room_dun_gen;
mod walled_room_dun_gen;

pub use dun_gen::DunGen;
pub use dun_gen_placed::DunGenPlaced;
pub use edge_portals_dun_gen::EdgePortalsDunGen;
pub use empty_room_dun_gen::EmptyRoomDunGen;
pub use walled_room_dun_gen::WalledRoomDunGen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
