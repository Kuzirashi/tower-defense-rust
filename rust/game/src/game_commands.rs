use crate::{core::Point, tower::TowerType};

pub enum GameCommand {
    BuildTower {
        tower_type: TowerType,
        position: Point,
    },
}
