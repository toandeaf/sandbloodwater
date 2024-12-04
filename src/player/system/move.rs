use bevy::prelude::*;
use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

use crate::common::EventWrapper;
use crate::network::Client;
use crate::player::component::{AnimationTimer, CharacterMarker, Direction, Player};
use crate::world::TileType;

#[derive(Event, Serialize, Deserialize, Copy, Clone)]
pub struct MovementEvent(pub Uuid, pub Direction);

// TODO work out how to properly abstract those bundles to reduce complexity
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn move_player(
    mut event_writer: EventWriter<MovementEvent>,
    mut player_query: Query<
        (&mut AnimationTimer, &CharacterMarker),
        (With<Player>, Without<TileType>),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut client: ResMut<Client>,
) {
    let player_bundle_res = player_query.get_single_mut();

    if let Ok((mut timer, uuid)) = player_bundle_res {
        timer.tick(time.delta());

        if timer.just_finished() {
            keyboard_input.get_pressed().for_each(|key_pressed| {
                let new_direction_opt = match key_pressed {
                    KeyCode::KeyW => Some(Direction::Up),
                    KeyCode::KeyS => Some(Direction::Down),
                    KeyCode::KeyA => Some(Direction::Left),
                    KeyCode::KeyD => Some(Direction::Right),
                    _ => None,
                };

                if let Some(direction) = new_direction_opt {
                    let movement_event = MovementEvent(uuid.0, direction);
                    event_writer.send(movement_event);
                    client.send_event(EventWrapper::Movement(movement_event));
                }
            });
        }
    }
}

