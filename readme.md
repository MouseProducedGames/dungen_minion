# Basic usage:

```rust
// External includes.
use dungen_minion::geometry::*;
use dungen_minion::*;

// Standard includes.

// Internal includes.

fn main() {
    // Create a dungeon generator using RoomHashMap.
    // RoomHashMap is expandable, and has no explicit size restrictions.
    let dungen = DunGen::new(RoomHashMap::default())
        // Expand the room to a width of 40, and a height of 30.
        // TileType::Floor will be placed.
        .gen_with(EmptyRoomDunGen::new(Size::new(40, 30)))
        // Create walls for the room.
        .gen::<WalledRoomDunGen>()
        .build();

    // A simple drawing routine.
    for y in 0..dungen.size().height() {
        for x in 0..dungen.size().width() {
            let tile_type = dungen.tile_type_at_local(LocalPosition::new(x, y));
            if tile_type.is_none() {
                continue;
            }

            // The selection of tiles is deliberately limited, for now.
            // Theming is included in future plans for dungen_minion.
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
}
```