use crate::core::config::{MAP_TILES_AMOUNT_X, MAP_TILES_AMOUNT_Y, MAP_TILES_TOTAL, TILE_PIXEL_SIZE};
use crate::core::position::map_pos_to_pixel_pos;
use crate::core::{Point, Sprite, SpriteType};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Road,
    None,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub position: Point,
    pub tile_type: TileType,
}

static NONE: TileType = TileType::None; // 436
static ROAD: TileType = TileType::Road; // 93 + 436

#[rustfmt::skip]
static DEFAULT_MAP: [[TileType; MAP_TILES_AMOUNT_X]; MAP_TILES_AMOUNT_Y] = [
    [NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, ROAD, ROAD, ROAD, ROAD, NONE, NONE, NONE, ROAD, ROAD, ROAD, ROAD, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, ROAD, ROAD, ROAD, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, ROAD, ROAD, ROAD, ROAD, ROAD, ROAD, ROAD, ROAD, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, ROAD, ROAD, ROAD, ROAD, ROAD, NONE, NONE, NONE, ROAD, ROAD, ROAD, ROAD, ROAD, ROAD, ROAD, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, NONE, NONE],
    [NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, ROAD, NONE, NONE, NONE, NONE, NONE],
  ];

pub struct GameMap {
    pub tiles_map: [[TileType; MAP_TILES_AMOUNT_X]; MAP_TILES_AMOUNT_Y],
    pub tiles: [Tile; MAP_TILES_TOTAL],
}

impl GameMap {
    pub fn new() -> GameMap {
        let tiles_map = DEFAULT_MAP;
        let mut tiles: [Tile; MAP_TILES_TOTAL] = [Tile {
            position: Point { x: 0, y: 0 },
            tile_type: TileType::None,
        }; MAP_TILES_TOTAL];

        let mut tile_index = 0;

        for (row_index, row) in tiles_map.iter().enumerate() {
            for (column_index, tile_type) in row.iter().enumerate() {
                tiles[tile_index] = Tile {
                    position: Point {
                        x: column_index as i32,
                        y: row_index as i32, //CONFIG_MAP_TILES_AMOUNT_Y - row_index as i32 - 1,
                    },
                    tile_type: *tile_type,
                };

                tile_index += 1;
            }
        }

        GameMap { tiles_map, tiles }
    }

    pub fn get_sprites(&self) -> Vec<Sprite> {
        let mut sprites = vec![];
        // let size = TILE_PIXEL_SIZE as u32;

        sprites.push(Sprite::create_image(
            &"/assets/tiles/map.png",
            Point::new(0, 0),
            735,
            740,
            0.0
        ));

        // for tile in self.tiles.iter() {
        //     let mut texture_path = "/assets/tiles/".to_owned();
        //     texture_path.push_str(if tile.tile_type == TileType::None {
        //         "ice_1.png"
        //     } else {
        //         "ground_1.png"
        //     });

        //     sprites.push(Sprite::create_image(
        //         &texture_path,
        //         map_pos_to_pixel_pos(tile.position),
        //         size,
        //         size,
        //         0.0
        //     ));
        // }

        sprites
    }
}
