mod component;
mod entity;
mod plugin;
mod system;

pub use component::{InteractionType, Interactive, Item, Solid};
pub use plugin::ItemPlugin;
pub use system::create_item;
