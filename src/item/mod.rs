mod component;
mod entity;
mod plugin;
mod system;

pub use component::{Interactable, Solid};
pub use plugin::ItemPlugin;
pub use system::create_item;
