pub use drop::{drop, DropEvent};
pub use drop_processor::process_drop;
pub use init::{
    initialise_player, PlayerCreateEvent, PlayerMapping, PlayerSyncEvent, PlayerTextureAtlas,
};
pub use init_dispatcher::init_player_dispatcher;
pub use init_processor::process_init;

pub use interact::{interact, InteractionEvent};
pub use interact_processor::process_interact;
pub use move_dispatcher::dispatch_position_change;
pub use move_processor::{process_direction_change, process_position_change};
pub use r#move::{move_player, MovementEvent};

pub use sync_processor::process_sync;

mod drop;
mod drop_processor;
mod init;
mod init_dispatcher;
mod init_processor;
mod interact;
mod interact_processor;
mod r#move;
mod move_dispatcher;
mod move_processor;
mod sync_processor;
