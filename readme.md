# Basic usage:

```rust
// External includes.
use dungen_minion::geometry::*;
use dungen_minion::*;

// Standard includes.

// Internal includes.

#[allow(clippy::borrowed_box)]
fn draw_map(map: &Box<dyn Room>) {
    for y in 0..map.size().height() {
        for x in 0..map.size().width() {
            let tile_type = map.tile_type_at_local(LocalPosition::new(x, y));
            if tile_type.is_none() {
                continue;
            }

            let tile_type = tile_type.unwrap();
            let ch = match tile_type {
                TileType::Void => ' ',
                TileType::Floor => '.',
                TileType::Wall => '#',
                TileType::Portal => '+',
            };

            print!("{}", ch);
        }
        println!();
    }
    
    
    for portal in map.portals() {
        println!("Map");
        draw_placed_map(portal.target());
    }
}

#[allow(clippy::borrowed_box)]
fn draw_placed_map(map: &Box<dyn PlacedRoom>) {
    for y in 0..map.size().height() {
        for x in 0..map.size().width() {
            let tile_type = map.tile_type_at_local(LocalPosition::new(x, y));
            if tile_type.is_none() {
                continue;
            }

            let tile_type = tile_type.unwrap();
            let ch = match tile_type {
                TileType::Void => ' ',
                TileType::Floor => '.',
                TileType::Wall => '#',
                TileType::Portal => '+',
            };

            print!("{}", ch);
        }
        println!();
    }
    
    
    for portal in map.portals() {
        println!("Map");
        draw_placed_map(portal.target());
    }
}

fn main() {
    let map = DunGen::new(Box::new(RoomHashMap::default()))
        .gen_with(EmptyRoomDunGen::new(Size::new(12, 8)))
        .gen::<WalledRoomDunGen>()
        .gen_leaf_portals_with(&EdgePortalsDunGen::new(
            5,
            Box::new(|| {
                Box::new(PlacedRoomWrapper::new(
                    Position::new(0, 0),
                    RoomHashMap::default(),
                ))
            }),
        ))
        .gen_leaf_portals_with::<EmptyRoomDunGen>(&EmptyRoomDunGen::new(Size::new(3, 10)))
        .gen_leaf_portals_static::<WalledRoomDunGen>()
        .gen_leaf_portals_with(&EdgePortalsDunGen::new(
            5,
            Box::new(|| {
                Box::new(PlacedRoomWrapper::new(
                    Position::new(0, 0),
                    RoomHashMap::default(),
                ))
            }),
        ))
        .gen_leaf_portals_with::<EmptyRoomDunGen>(&EmptyRoomDunGen::new(Size::new(6, 4)))
        .gen_leaf_portals_static::<WalledRoomDunGen>()
        .build();

    println!("Map");
    draw_map(&map);
}

```