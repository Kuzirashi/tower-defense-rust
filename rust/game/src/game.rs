use crate::entities::{CreatureEventType, Monster};
use crate::map::GameMap;
use crate::pathfinding::find_path_to_end;
use crate::{core::config::MAP_TILES_AMOUNT_X, entities::MonsterType};
use crate::{
    core::config::{SPAWN_POINT_X, SPAWN_POINT_Y, TILE_PIXEL_SIZE},
    wave::Wave,
};
use crate::{
    core::{config::MAP_TILES_AMOUNT_Y, position::Rectangle, Color, Point, Sprite},
    game_commands::GameCommand,
    tower::TowerType,
    tower_builder::TowerBuilder,
};
use crate::{projectile::Projectile, tower::Tower};
use std::{
    cell::{Cell, RefCell},
    ops::Add,
};

#[derive(Clone)]
pub struct MouseState {
    pub left_button_pressed: bool,
    pub position: Point,
}

impl MouseState {
    pub fn new(left_button_pressed: bool, position: Point) -> MouseState {
        MouseState {
            left_button_pressed,
            position,
        }
    }
}

pub struct Game {
    entities: RefCell<Vec<Monster>>,
    towers: RefCell<Vec<Tower>>,
    projectiles: RefCell<Vec<Projectile>>,
    waves: Vec<Wave>,
    mouse_state: MouseState,
    map: GameMap,
    last_update_call_time_elapsed_since_start: f64,
    time_since_spawning_last_monster: f64,
    monsters_to_spawn: Cell<i32>,
    monster_path: Vec<Point>,
    lifes: u8,
    delay_between_spawning_monsters: f64,
    tower_builder: TowerBuilder,
    score: u8
}

impl Game {
    pub fn new() -> Game {
        let map = GameMap::new();

        let monster_path = find_path_to_end(
            map.tiles,
            Point {
                x: SPAWN_POINT_X,
                y: SPAWN_POINT_Y,
            },
        )
        .unwrap();

        Game {
            monsters_to_spawn: Cell::new(1),
            entities: RefCell::new(vec![]),
            towers: RefCell::new(vec![]),
            projectiles: RefCell::new(vec![]),
            waves: vec![Wave {
                delay_between_spawning_monsters: 1000.0,
                monster_type: MonsterType::Creeper,
                monsters_count: 10,
            }],
            last_update_call_time_elapsed_since_start: 0.0,
            time_since_spawning_last_monster: 0.0,
            map,
            monster_path,
            lifes: 10,
            score: 0,
            delay_between_spawning_monsters: 0.0,
            mouse_state: MouseState::new(false, Point::new(0, 0)),
            tower_builder: TowerBuilder::new(Point::new(750, 200)),
        }
    }

    fn spawn_monster(&self) {
        let spawn_point: Point = Point::new(SPAWN_POINT_X, SPAWN_POINT_Y);

        let monster = Monster::new(MonsterType::Creeper, spawn_point, self.monster_path.clone());

        self.entities.borrow_mut().push(monster);
    }

    pub fn get_sprites(&self) -> Vec<Sprite> {
        let mut sprites = self.map.get_sprites();

        let towers = self.entities.borrow();
        let entities = self.towers.borrow();
        let projectiles = self.projectiles.borrow();

        for monster in entities.iter() {
            sprites.append(&mut monster.get_sprites());
        }

        for tower in towers.iter() {
            sprites.append(&mut tower.get_sprites());
        }

        for projectile in projectiles.iter() {
            sprites.append(&mut projectile.get_sprites());
        }

        sprites.push(Sprite::create_image(
            "/assets/interface/background.png",
            Point {
                x: (MAP_TILES_AMOUNT_X * TILE_PIXEL_SIZE as usize) as i32,
                y: 0,
            },
            107 * 2,
            450 * 2,
            0.0,
        ));

        sprites.push(Sprite::create_image(
            "/assets/interface/icon_lifes.png",
            Point {
                x: (MAP_TILES_AMOUNT_X * TILE_PIXEL_SIZE as usize) as i32 + 32,
                y: 32,
            },
            25,
            32,
            0.0,
        ));

        sprites.push(Sprite::create_text(
            &self.lifes.to_string(),
            Point {
                x: (MAP_TILES_AMOUNT_X * TILE_PIXEL_SIZE as usize) as i32 + 32 + 25 + 10,
                y: 30,
            },
            32,
        ));

        sprites.push(Sprite::create_image(
            "/assets/interface/icon_score.png",
            Point {
                x: (MAP_TILES_AMOUNT_X * TILE_PIXEL_SIZE as usize) as i32 + 32,
                y: 80,
            },
            17,
            16,
            0.0,
        ));

        sprites.push(Sprite::create_text(
            &self.score.to_string(),
            Point {
                x: (MAP_TILES_AMOUNT_X * TILE_PIXEL_SIZE as usize) as i32 + 67,
                y: 80,
            },
            16,
        ));

        sprites.append(&mut self.get_next_wave_display());
        sprites.append(&mut self.tower_builder.get_sprites());

        sprites
    }

    fn get_next_wave_display(&self) -> Vec<Sprite> {
        let mut sprites = vec![];
        match self.get_current_wave() {
            Some(wave) => {
                let mut msg = "Next wave: ".to_string();
                let monster_type = &wave.monster_type.to_owned();
                let monster_name: &str = monster_type.into();
                msg.push_str(&monster_name);

                sprites.push(Sprite::create_text(
                    &msg,
                    Point {
                        x: 30,
                        y: (MAP_TILES_AMOUNT_Y * TILE_PIXEL_SIZE as usize) as i32 + 32 + 25 + 10,
                    },
                    16,
                ));

                let mut msg = "Number of monsters: ".to_string();
                msg.push_str(&wave.monsters_count.to_string());

                sprites.push(Sprite::create_text(
                    &msg,
                    Point {
                        x: 30,
                        y: (MAP_TILES_AMOUNT_Y * TILE_PIXEL_SIZE as usize) as i32
                            + 32
                            + 25
                            + 10
                            + 20,
                    },
                    16,
                ));

                let mut path = "/assets/creatures/".to_string();
                path.push_str(&monster_name.to_lowercase());
                path.push_str("/bottom_0.png");

                sprites.push(Sprite::create_image(
                    &path,
                    Point {
                        x: 30,
                        y: (MAP_TILES_AMOUNT_Y * TILE_PIXEL_SIZE as usize) as i32 + 110,
                    },
                    32,
                    32,
                    0.0,
                ));
            }
            _ => {}
        }

        sprites
    }

    pub fn start_round(&mut self) {
        match self.get_current_wave() {
            Some(wave) => {
                self.monsters_to_spawn.set(wave.monsters_count);
                self.delay_between_spawning_monsters = wave.delay_between_spawning_monsters;
            }
            _ => {}
        }
    }

    fn get_current_wave(&self) -> Option<&Wave> {
        self.waves.get(0)
    }

    pub fn update(&mut self, time_elapsed_since_start: f64, mouse_state: MouseState) {
        self.mouse_state = mouse_state;

        let time_elapsed =
            time_elapsed_since_start - self.last_update_call_time_elapsed_since_start;

        self.time_since_spawning_last_monster += time_elapsed;

        {
            let mut monsters = self.entities.borrow_mut();

            for entity in monsters.iter_mut() {
                match entity.update(time_elapsed) {
                    Some(CreatureEventType::Killed) => {
                        self.score += 1;
                    }
                    Some(CreatureEventType::ReachedFinalDestination) => {
                        self.lifes = self
                            .lifes
                            .saturating_sub(1)
                    }
                    _ => {}
                }
            }

            monsters.retain(|x| !x.reached_final_destination && x.alive);


            let mut towers = self.towers.borrow_mut();
            let mut projectiles = self.projectiles.borrow_mut();

            for tower in towers.iter_mut() {
                tower.update(time_elapsed, &mut monsters, &mut projectiles);
            }

            for item in projectiles.iter_mut() {
                item.update(time_elapsed, &mut monsters);
            }

            projectiles.retain(|x| x.active);
        }

        if let Some(GameCommand::BuildTower { position, .. }) =
            self.tower_builder.update(self.mouse_state.clone())
        {
            self.build_tower(position);
        }

        if self.time_since_spawning_last_monster > self.delay_between_spawning_monsters
            && self.monsters_to_spawn.get() > 0
        {
            self.spawn_monster();
            &self
                .monsters_to_spawn
                .set(&self.monsters_to_spawn.get() - 1);

            self.time_since_spawning_last_monster = 0.0;
        }

        self.last_update_call_time_elapsed_since_start = time_elapsed_since_start;
    }

    pub fn build_tower(&self, position: Point) {
        let tower = Tower::new(position, TowerType::Orc);

        self.towers.borrow_mut().push(tower);
    }
}
