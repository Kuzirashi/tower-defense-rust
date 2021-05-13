use crate::entities::MonsterType;

#[derive(Clone)]
pub struct Wave {
    pub monsters_count: i32,
    pub delay_between_spawning_monsters: f64,
    pub monster_type: MonsterType,
}

impl Wave {
    pub fn empty() -> Wave {
        Wave {
            monsters_count: 0,
            delay_between_spawning_monsters: 0.0,
            monster_type: MonsterType::Creeper,
        }
    }
}
