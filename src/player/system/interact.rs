use crate::item::{create_item_entity, Interactable};
use crate::player::component::{CurrentDirection, Direction, Player};
use crate::player::resource::PlayerAttributes;
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn interact(
    mut commands: Commands,
    player_attributes: Res<PlayerAttributes>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &CurrentDirection), With<Player>>,
    item_query: Query<(&Transform, Entity), With<Interactable>>,
) {
    let player_radius = player_attributes.size / 2.;

    for (player_transform, current_direction) in &mut player_query {
        let player_position = player_transform.translation;

        if keyboard_input.pressed(KeyCode::E) {
            collect_item(player_position, player_radius, &mut commands, &item_query)
        } else if keyboard_input.pressed(KeyCode::R) {
            spawn_item(
                player_position,
                player_radius,
                &mut commands,
                &current_direction.0,
            );
            keyboard_input.reset(KeyCode::R);
        }
    }
}

fn spawn_item(
    player_position: Vec3,
    player_radius: f32,
    commands: &mut Commands,
    direction: &Direction,
) {
    let item_spawn_position: Vec2 = match direction {
        Direction::Up => Vec2::new(player_position.x, player_position.y + player_radius),
        Direction::Down => Vec2::new(player_position.x, player_position.y - player_radius),
        Direction::Left => Vec2::new(player_position.x - player_radius, player_position.y),
        Direction::Right => Vec2::new(player_position.x + player_radius, player_position.y),
    };

    commands.spawn(create_item_entity(item_spawn_position));
}

fn collect_item(
    player_position: Vec3,
    player_radius: f32,
    commands: &mut Commands,
    item_query: &Query<(&Transform, Entity), With<Interactable>>,
) {
    let player_y_perimeter =
        player_position.y - (player_radius * 2.)..player_position.y + (player_radius * 2.);

    let player_x_perimeter =
        player_position.x - (player_radius * 2.)..player_position.x + (player_radius * 2.);

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
