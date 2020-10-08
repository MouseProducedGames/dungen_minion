// External includes.
mod traits {
    pub use generic_dungen_traits::*;
}

pub use generic_dungen_traits::*;

// Standard includes.

// Internal includes.
mod dun_gen;

pub use dun_gen::DunGen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
