use bevy::prelude::Bundle;
use bevy_ecs_ldtk::LdtkIntCell;

use super::components::{UnPassable, DirtRoad, Terrace};

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct DirtRoadBundle {
    wall: DirtRoad,
    un_passable: UnPassable,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct TerraceBundle {
    wall: Terrace,
    un_passable: UnPassable,
}
