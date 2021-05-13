use uuid::Uuid;
use crate::core::config::{SIMULATION_TIMESTEP, TILE_PIXEL_SIZE};
use crate::core::position::map_pos_to_pixel_pos;
use crate::core::{Color, Direction, Point, Sprite};
use std::ops::Add;
use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr, Clone)]
pub enum MonsterType {
    Creeper,
}

#[derive(PartialEq)]
pub enum CreatureEventType {
    ReachedFinalDestination,
    Killed
}

pub struct Monster {
    pub id: Uuid,
    pub name: String,
    pub position: Point,
    pub direction_facing: Direction,
    pub health: i32,
    pub alive: bool,
    pub reached_final_destination: bool,

    path_to_follow: Vec<Point>,
    position_wanting_to_reach: Option<Point>,
    final_position: Point,
    is_moving: bool,
    time_elapsed_since_last_move: f64,
    delay_between_moves: f64,
    time_from_last_change_of_frame: f64,
    change_frame_speed: f64,
    movement_frame: i8,
    transitional_position: i8,
    speed_in_pixels: i8,
    time_to_simulate: f64,
}

impl Monster {
    pub fn new(monster_type: MonsterType, position: Point, path_to_follow: Vec<Point>) -> Monster {
        let name: &str = monster_type.into();
        let name = name.to_lowercase();

        Monster {
            id: Uuid::new_v4(),
            name,
            position,
            direction_facing: Direction::Bottom,
            path_to_follow,
            position_wanting_to_reach: None,
            is_moving: false,
            time_elapsed_since_last_move: 0.0,
            delay_between_moves: 10.0,
            time_from_last_change_of_frame: 0.0,
            change_frame_speed: 200.0,
            movement_frame: 0,
            transitional_position: 0,
            speed_in_pixels: 2,
            final_position: Point { x: 17, y: 22 },
            reached_final_destination: false,
            health: 100,
            alive: true,
            time_to_simulate: 0.0,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.direction_facing != direction {
            self.direction_facing = direction;
        }
    }

    fn update_position(&mut self, new_position: Point) {
        self.position = new_position;
    }

    fn change_position(&mut self, new_position: Point) {
        self.update_position(new_position);
        self.path_to_follow = self
            .path_to_follow
            .iter()
            .filter(|&&p| p != new_position)
            .map(|x| *x)
            .collect();

        self.position_wanting_to_reach = None;
    }

    pub fn move_left(&mut self) {
        if self.is_moving {
            return;
        }

        self.change_direction(Direction::Left);

        self.position_wanting_to_reach = Some(Point {
            x: self.position.x - 1,
            y: self.position.y,
        });
        self.is_moving = true;
    }

    pub fn move_right(&mut self) {
        if self.is_moving {
            return;
        }

        self.change_direction(Direction::Right);

        self.position_wanting_to_reach = Some(Point {
            x: self.position.x + 1,
            y: self.position.y,
        });
        self.is_moving = true;
    }

    pub fn move_down(&mut self) {
        if self.is_moving {
            return;
        }

        self.change_direction(Direction::Bottom);

        self.position_wanting_to_reach = Some(Point {
            x: self.position.x,
            y: self.position.y + 1,
        });
        self.is_moving = true;
    }

    pub fn move_up(&mut self) {
        if self.is_moving {
            return;
        }

        self.change_direction(Direction::Top);

        self.position_wanting_to_reach = Some(Point {
            x: self.position.x,
            y: self.position.y - 1,
        });
        self.is_moving = true;
    }

    fn get_sprite_texture_path(&self) -> String {
        let mut path = "/assets/creatures/".to_owned();

        path.push_str(&self.name);
        path.push_str("/");
        path.push_str(&self.direction_facing.get_lowercase());
        path.push_str("_");
        path.push_str(&self.movement_frame.to_string());
        path.push_str(".png");

        path
    }

    fn move_to_desired_position(&mut self) {
        if self.position_wanting_to_reach.is_none() {
            return;
        }

        self.change_position(self.position_wanting_to_reach.unwrap());

        self.transitional_position = 0;

        self.is_moving = false;
    }

    pub fn update(&mut self, elapsed_time: f64) -> Option<CreatureEventType> {
        if self.health <= 0 {
            self.alive = false;

            return Some(CreatureEventType::Killed);
        }
        
        if self.position == self.final_position {
            self.reached_final_destination = true;

            return Some(CreatureEventType::ReachedFinalDestination);
        }

        self.update_movement(elapsed_time);

        None
    }

    pub fn update_movement(&mut self, elapsed_time: f64) {
        self.time_to_simulate += elapsed_time;

        while self.time_to_simulate >= SIMULATION_TIMESTEP {
            self.update_movement_single_frame();
            self.time_to_simulate -= SIMULATION_TIMESTEP;
        }
    }

    fn update_movement_single_frame(&mut self) {
        let next_move = self.path_to_follow.get(0);

        match next_move {
            Some(_next_move) => match self.position_wanting_to_reach {
                Some(_) => {}
                None => match self.position.direction_towards(_next_move) {
                    Direction::Bottom => self.move_down(),
                    Direction::Right => self.move_right(),
                    Direction::Top => self.move_up(),
                    Direction::Left => self.move_left(),
                },
            },
            None => {}
        }

        if !self.is_moving {
            return;
        }

        self.time_elapsed_since_last_move += SIMULATION_TIMESTEP;

        if self.delay_between_moves <= self.time_elapsed_since_last_move {
            self.handle_movement_frames(self.delay_between_moves);

            if (self.transitional_position as i32) < TILE_PIXEL_SIZE {
                self.move_by_pixels(self.speed_in_pixels);
            } else {
                self.move_to_desired_position();
            }

            self.time_elapsed_since_last_move = 0.0;
        }
    }

    fn handle_movement_frames(&mut self, elapsed_time: f64) {
        self.time_from_last_change_of_frame += elapsed_time;

        if self.time_from_last_change_of_frame >= self.change_frame_speed {
            if self.movement_frame == 0 || self.movement_frame == 2 {
                self.movement_frame = 1;
            } else if self.movement_frame == 1 {
                self.movement_frame = 2;
            }

            self.time_from_last_change_of_frame = 0.0;
        }
    }

    fn move_by_pixels(&mut self, pixels: i8) {
        self.transitional_position += pixels;
    }

    pub fn get_sprites(&self) -> Vec<Sprite> {
        let mut sprites = vec![];

        let mut x_pixels: i8 = 0;
        let mut y_pixels: i8 = 0;

        match self.direction_facing {
            Direction::Bottom => y_pixels += self.transitional_position,
            Direction::Top => y_pixels -= self.transitional_position,
            Direction::Left => x_pixels -= self.transitional_position,
            Direction::Right => x_pixels += self.transitional_position,
        }

        let position = map_pos_to_pixel_pos(self.position);
        let position = Point {
            x: position.x + x_pixels as i32,
            y: position.y + y_pixels as i32,
        };

        sprites.push(Sprite::create_image(
            &self.get_sprite_texture_path(),
            position,
            TILE_PIXEL_SIZE as u32,
            TILE_PIXEL_SIZE as u32,
            0.0,
        ));

        sprites.push(Sprite::create_rect(
            Color::new(0, 0, 0, 255),
            position.add(Point { x: 0, y: -7 }),
            28,
            4,
        ));

        let full_hp = 100;

        sprites.push(Sprite::create_rect(
            Color::new(15, 96, 39, 255),
            position.add(Point { x: 1, y: -6 }),
            (26 * self.health / full_hp) as u32,
            2,
        ));

        sprites.push(Sprite::create_text(
            &self.name,
            position.add(Point { x: 0, y: -16 }),
            8,
        ));

        sprites
    }

    pub fn take_damage(&mut self, damage: i32) {
        if !self.alive {
            return;
        }

        self.health -= damage;
    }
}
