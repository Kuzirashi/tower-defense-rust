use crate::core::config::TILE_PIXEL_SIZE;
use crate::core::position::map_pos_to_pixel_pos;
use crate::core::{Point, Sprite};
use crate::entities::Monster;
use crate::projectile::Projectile;
use std::cell::RefMut;
use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr, Clone)]
pub enum TowerType {
    Orc,
}

pub struct Tower {
    pub position: Point,
    pub level: i8,
    pub tower_type: TowerType,

    time_from_last_attack: f64,
    range: i32,
    attack_cooldown: f64,
    damage: i32,
}

impl Tower {
    pub fn new(position: Point, tower_type: TowerType) -> Tower {
        Tower {
            position,
            level: 1,
            time_from_last_attack: 0.0,
            range: 2,
            attack_cooldown: 500.0,
            damage: 20,
            tower_type
        }
    }

    fn get_sprite_texture_path(&self) -> String {
        let mut path = get_tower_sprite_base_path(self.tower_type.clone());

        path.push_str("level ");
        path.push_str(&self.level.to_string());
        path.push_str("/full.png");

        path
    }

    pub fn get_sprites(&self) -> Vec<Sprite> {
        let mut sprites = vec![];

        let x_pixels: i8 = -32;
        let y_pixels: i8 = -32;

        let position = map_pos_to_pixel_pos(self.position);
        let position = Point {
            x: position.x + x_pixels as i32,
            y: position.y + y_pixels as i32,
        };

        sprites.push(Sprite::create_image(
            &self.get_sprite_texture_path(),
            position,
            TILE_PIXEL_SIZE as u32 * 2,
            TILE_PIXEL_SIZE as u32 * 2,
            0.0,
        ));

        sprites
    }

    pub fn update(
        &mut self,
        elapsed_time: f64,
        monsters: &mut RefMut<Vec<Monster>>,
        projectiles: &mut RefMut<Vec<Projectile>>,
    ) {
        self.time_from_last_attack += elapsed_time;

        if self.attack_cooldown <= self.time_from_last_attack {
            for entity in monsters.iter_mut() {
                let abs_x_diff = (entity.position.x - self.position.x).abs();
                let abs_y_diff = (entity.position.y - self.position.y).abs();

                if abs_x_diff <= self.range && abs_y_diff <= self.range {
                    self.send_projectile_towards_creature(entity, projectiles);
                    self.time_from_last_attack = 0.0;
                    break;
                }
            }
        }
    }

    fn send_projectile_towards_creature(
        &self,
        creature: &mut Monster,
        projectiles: &mut RefMut<Vec<Projectile>>,
    ) {
        let projectile = Projectile::new(
            self.position,
            creature.position,
            self.damage,
            creature.id,
            self.tower_type.clone(),
        );
        // if (this.takeMonsterHealthBeforeReaching) {
        //   this.dealDamageToCreature(creature);
        // } else {
        //   projectile.addEventSubscriber('targetReached', () => {
        //     this.dealDamageToCreature(creature);
        //   });
        // }
        projectiles.push(projectile);
    }
}

pub fn get_tower_sprite_base_path(tower_type: TowerType) -> String {
    let mut path = "/assets/towers/".to_owned();

    let name: &str = tower_type.into();
    let name = name.to_lowercase();

    path.push_str(&name);
    path.push_str("/");

    path
}
