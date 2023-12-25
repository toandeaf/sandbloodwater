use std::io::Error;
use std::str::FromStr;

use bevy::prelude::Component;

#[derive(Component)]
pub struct World;

#[derive(Component, Debug, Clone, Copy)]
pub enum TileType {
    Land,
    Mountain,
    Water,
    Building,
    Unsupported,
}

impl TileType {
    pub fn speed_modifier(&self) -> f32 {
        match *self {
            TileType::Building => 0.,
            TileType::Water => 0.3,
            TileType::Mountain => 0.,
            // Default is just keep the speed the same
            _ => 1.,
        }
    }
}

impl FromStr for TileType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "L" => Ok(TileType::Land),
            "M" => Ok(TileType::Mountain),
            "W" => Ok(TileType::Water),
            "B" => Ok(TileType::Building),
            _ => Ok(TileType::Unsupported),
        }
    }
}
