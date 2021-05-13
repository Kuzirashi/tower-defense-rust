use std::ops::Add;

use crate::{
    core::{
        config::TILE_PIXEL_SIZE,
        position::{pixel_pos_to_map_pos, Rectangle},
        Color, Point, Sprite,
    },
    game::MouseState,
    game_commands::GameCommand,
    tower::{get_tower_sprite_base_path, TowerType},
};

const TOWERS: [TowerType; 1] = [TowerType::Orc];

pub struct TowerBuilder {
    mouse_state: MouseState,
    ui_start_position: Point,
    chosen_tower: Option<TowerType>,
}

impl TowerBuilder {
    pub fn new(ui_start_position: Point) -> TowerBuilder {
        TowerBuilder {
            mouse_state: MouseState::new(false, Point::new(0, 0)),
            ui_start_position,
            chosen_tower: None,
        }
    }

    pub fn update(&mut self, mouse_state: MouseState) -> Option<GameCommand> {
        let previous_mouse_state = self.mouse_state.clone();
        self.mouse_state = mouse_state;

        if self.mouse_state.left_button_pressed && !previous_mouse_state.left_button_pressed {
            if let Some(tower_type) = self.chosen_tower.clone() {
                self.chosen_tower = None;

                return Some(GameCommand::BuildTower {
                    tower_type,
                    position: pixel_pos_to_map_pos(self.mouse_state.position),
                });
            } else {
                for (tower_type, rect) in self.get_towers_rectangles().iter() {
                    if Rectangle::from(self.mouse_state.position).intersects(rect.clone()) {
                        self.chosen_tower = Some(tower_type.clone());
                    }
                }
            }
        }

        None
    }

    pub fn get_sprites(&self) -> Vec<Sprite> {
        let mut sprites = vec![];

        sprites.append(&mut self.get_towers_builder_menu());

        if let Some(tower_type) = &self.chosen_tower {
            let mut path = get_tower_sprite_base_path(tower_type.clone());
            path.push_str("level 1/icon.png");
            sprites.push(Sprite::create_image(
                &path,
                self.mouse_state.position,
                30,
                30,
                0.0,
            ));

            sprites.push(Sprite::create_rect(
                Color::new(255, 255, 255, 30),
                pixel_pos_to_map_pos(self.mouse_state.position)
                    * Point {
                        x: TILE_PIXEL_SIZE,
                        y: TILE_PIXEL_SIZE,
                    },
                TILE_PIXEL_SIZE as u32,
                TILE_PIXEL_SIZE as u32,
            ));
        }

        sprites
    }

    pub fn get_towers_builder_menu(&self) -> Vec<Sprite> {
        let mut sprites = vec![];

        for tower_type in TOWERS.iter() {
            let mut path = get_tower_sprite_base_path(tower_type.clone());
            path.push_str("level 1/icon.png");

            sprites.push(Sprite::create_image(
                &"/assets/interface/slot.png",
                self.ui_start_position,
                32,
                32,
                0.0,
            ));

            if Rectangle::from(self.mouse_state.position).intersects(Rectangle::new(
                self.ui_start_position,
                32,
                32,
            )) {
                sprites.push(Sprite::create_rect(
                    Color::new(255, 255, 255, 30),
                    self.ui_start_position,
                    30,
                    30,
                ));
            }

            sprites.push(Sprite::create_image(
                &path,
                self.ui_start_position.add(Point::new(-2, -2)),
                30,
                30,
                0.0,
            ));
        }

        sprites
    }

    fn get_towers_rectangles(&self) -> Vec<(TowerType, Rectangle)> {
        let mut rectangles = vec![];

        for tower_type in TOWERS.iter() {
            rectangles.push((
                tower_type.clone(),
                Rectangle::new(self.ui_start_position, 32, 32),
            ));
        }

        rectangles
    }
}
