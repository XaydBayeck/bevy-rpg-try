use bevy::prelude::Component;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Component)]
pub struct UnPassable;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Component)]
pub struct DirtRoad;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Component)]
pub struct Terrace;
