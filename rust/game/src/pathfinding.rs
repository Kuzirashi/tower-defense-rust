use crate::core::Point;
use crate::map::{Tile, TileType};

pub fn find_path_to_end(tiles: [Tile; 529], start: Point) -> Result<Vec<Point>, String> {
    let movable_tiles = tiles
        .iter()
        .filter(|&&tile| tile.tile_type == TileType::Road);

    let movable_positions: Vec<Point> = movable_tiles.into_iter().map(|x| x.position).collect();

    let mut path: Vec<Point> = vec![];
    let mut open_fields: Vec<&Point> = movable_positions
        .iter()
        .filter(|&&field| field != start)
        .collect();

    let mut current_working_position: Option<Point> = Some(start);

    while open_fields.len() > 0 {
        match current_working_position {
            Some(_current_working_position) => {
                let sibling_positions: Vec<&Point> = open_fields
                    .iter()
                    .filter(|x| x.is_straight_neighbour_of(&_current_working_position))
                    .map(|&x| x)
                    .collect();

                let next_move = sibling_positions.get(0);

                match next_move {
                    Some(&&_next_move) => {
                        path.push(_next_move);

                        open_fields = open_fields
                            .iter()
                            .filter(|&&&field| field != _next_move)
                            .map(|&x| x)
                            .collect();

                        current_working_position = Some(_next_move);
                    }
                    None => {
                        println!("Break in match next_move");
                        break;
                    }
                }
            }
            None => {
                println!("Break in match CWP");

                break;
            }
        }
    }

    Ok(path)
}
