// External includes.
mod traits {
    pub use generic_dungen_traits::*;
}

pub use generic_dungen_traits::*;

// Standard includes.

// Internal includes.
mod dun_gen;
mod empty_room_dun_gen;
mod walled_room_dun_gen;

pub use dun_gen::DunGen;
pub use empty_room_dun_gen::EmptyRoomDunGen;
pub use walled_room_dun_gen::WalledRoomDunGen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
