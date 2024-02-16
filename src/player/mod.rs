pub use component::{CharacterMarker, CurrentDirection, Player};
pub use plugin::PlayerPlugin;
pub use system::{MovementEvent, PlayerCreateEvent, PlayerMapping, PlayerSyncEvent};

mod component;
mod entity;
mod plugin;
mod resource;
mod system;
