use crate::item::{create_item_entity, Interactable};
use crate::player::component::{AnimationTimer, CurrentDirection, Direction, Player};
use crate::player::resource::PlayerAttributes;
use bevy::prelude::*;

// TODO move to a global constant?
const ITEM_Z_INDEX: f32 = 2.;

// TODO tidy and abstract
#[allow(clippy::type_complexity)]
pub fn interact(
    mut commands: Commands,
    time: Res<Time>,
    player_attributes: Res<PlayerAttributes>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &CurrentDirection, &mut AnimationTimer), With<Player>>,
    item_query: Query<(&Transform, Entity), With<Interactable>>,
) {
    let player_radius = player_attributes.size / 2.;

    for (player_transform, state, mut timer) in &mut player_query {
        let player_position = player_transform.translation;

        timer.tick(time.delta());

        if timer.just_finished() {
            // TODO This should be a single click, not sustained?
            keyboard_input
                .get_pressed()
                .for_each(|key_pressed| match key_pressed {
                    KeyCode::E => {
                        let player_y_perimeter = player_position.y - (player_radius * 2.)
                            ..player_position.y + (player_radius * 2.);

                        let player_x_perimeter = player_position.x - (player_radius * 2.)
                            ..player_position.x + (player_radius * 2.);

                        for (transform, entity) in item_query.iter() {
                            if player_x_perimeter.contains(&transform.translation.x)
                                && player_y_perimeter.contains(&transform.translation.y)
                            {
                                commands
                                    .get_entity(entity)
                                    .iter_mut()
                                    .for_each(|entity| entity.despawn());
                            }
                        }
                    }
                    KeyCode::R => {
                        let item_spawn_position: Vec2 = match state.0 {
                            Direction::Up => {
                                Vec2::new(player_position.x, player_position.y + player_radius)
                            }
                            Direction::Down => {
                                Vec2::new(player_position.x, player_position.y - player_radius)
                            }
                            Direction::Left => {
                                Vec2::new(player_position.x - player_radius, player_position.y)
                            }
                            Direction::Right => {
                                Vec2::new(player_position.x + player_radius, player_position.y)
                            }
                        };

                        commands.spawn(create_item_entity(Vec3::from((
                            item_spawn_position,
                            ITEM_Z_INDEX,
                        ))));
                    }
                    _ => {}
                });
        }
    }
}
