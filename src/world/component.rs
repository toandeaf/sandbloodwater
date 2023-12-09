use std::io::Error;
use std::str::FromStr;

use bevy::prelude::Component;

#[derive(Component)]
pub struct World;

#[derive(Debug)]
pub enum TileType {
    Land,
    Mountain,
    Water,
    Building,
    Unsupported,
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
