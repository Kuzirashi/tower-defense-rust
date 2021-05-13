use crate::{core::{config::TILE_PIXEL_SIZE, position::map_pos_to_pixel_pos, Point, Sprite}, entities::Monster, tower::{Tower, TowerType, get_tower_sprite_base_path}};
use std::{cell::RefMut, ops::Add};
use uuid::Uuid;

pub struct Projectile {
    pub position: Point,
    pub damage: i32,
    pub rotation: f64,

    pub active: bool,
    tower_type: TowerType,

    target_position: Point,
    target_id: Uuid,
    speed: f64,
    cooldown: f64,
    time_to_simulate: f64,
}

impl Projectile {
    pub fn new(
        position: Point,
        target_position: Point,
        damage: i32,
        target_id: Uuid,
        tower_type: TowerType,
    ) -> Projectile {
        Projectile {
            position: map_pos_to_pixel_pos(position).add(Point::new(0, 0)),
            damage,
            rotation: 0.0,
            target_position: map_pos_to_pixel_pos(target_position)
                .add(Point::new(TILE_PIXEL_SIZE / 2, TILE_PIXEL_SIZE / 2)),
            tower_type,
            active: true,
            target_id,
            speed: 12.0,
            cooldown: 30.0,
            time_to_simulate: 0.0,
        }
    }

    pub fn get_sprites(&self) -> Vec<Sprite> {
        let mut sprites = vec![];

        sprites.push(Sprite::create_image(
            &self.get_sprite_texture_path(),
            self.position,
            26,
            7,
            self.rotation.to_degrees(),
        ));

        // sprites.push(Sprite::create_rect(
        //     Color::new(255, 0, 0, 255),
        //     self.target_position,
        //     2,
        //     2,
        // ));

        sprites
    }

    pub fn update(&mut self, elapsed_time: f64, monsters: &mut RefMut<Vec<Monster>>) {
        if !self.active {
            return;
        }

        self.time_to_simulate += elapsed_time;

        while self.time_to_simulate >= self.cooldown {
            self.update_movement_single_frame(monsters);
            self.time_to_simulate -= self.cooldown;
        }
    }

    fn update_movement_single_frame(&mut self, monsters: &mut RefMut<Vec<Monster>>) {
        let distance_y_pixels = (self.target_position.y - self.position.y) as f64;
        let distance_x_pixels = (self.target_position.x - self.position.x) as f64;

        self.rotation = distance_y_pixels.atan2(distance_x_pixels);

        let movement_x = self.rotation.cos() * self.speed;
        let movement_y = self.rotation.sin() * self.speed;

        self.position = Point::new(
            self.position.x + movement_x as i32,
            self.position.y + movement_y as i32,
        );

        if (self.position.x - self.target_position.x).abs() < 5
            && (self.position.y - self.target_position.y).abs() < 5
        {
            for monster in monsters.iter_mut() {
                if monster.id == self.target_id {
                    monster.take_damage(20);
                }
            }

            self.active = false;
        }
    }

    fn get_sprite_texture_path(&self) -> String {
        let mut path = get_tower_sprite_base_path(self.tower_type.clone());
        path.push_str("shoot.png");

        path
    }
}
