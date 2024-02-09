use crate::network::client::ClientPlugin;
use crate::network::resource::NetworkWrapper;
use crate::network::server::ServerPlugin;
use bevy::prelude::{App, Plugin};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "client")]
        {
            app.add_plugins(ClientPlugin);
        }

        #[cfg(feature = "server")]
        {
            app.add_event::<NetworkWrapper>().add_plugins(ServerPlugin);
        }
    }
}
