use bevy::prelude::*;

use crate::item::Item;
use crate::player::component::{Activity, CurrentDirection, Direction, Player};
use crate::player::resource::PlayerAttributes;

#[derive(Event)]
pub struct InteractionEvent(pub Entity, pub Entity, pub Vec3);

#[allow(clippy::type_complexity)]
pub fn interact(
    mut event_writer: EventWriter<InteractionEvent>,
    player_attributes: Res<PlayerAttributes>,
    keyboard_input: ResMut<Input<KeyCode>>,
    mut player_query: Query<(&Transform, Entity, &CurrentDirection), With<Player>>,
    item_query: Query<(&Transform, Entity), With<Item>>,
) {
    // TODO Player radius should go into player attributes as it won't fluctuate wildly.
    let player_radius = player_attributes.radius;

    for (player_transform, player_entity, current_direction) in &mut player_query {
        let player_data = (
            player_transform.translation,
            player_radius,
            player_entity,
            &current_direction.0,
        );

        if keyboard_input.just_pressed(KeyCode::E) {
            interact_with_item(&mut event_writer, &item_query, player_data);
        }
    }
}

fn interact_with_item(
    event_writer: &mut EventWriter<InteractionEvent>,
    item_query: &Query<(&Transform, Entity), With<Item>>,
    player_data: (Vec3, f32, Entity, &Direction),
) -> Option<Activity> {
    let (player_position, player_radius, player_entity, player_direction) = player_data;

    // TODO rework interaction box, should be able to pick up stuff within the front of your sprite.
    let interaction_box = player_direction.interaction_box(player_position, player_radius);

    for (transform, entity) in item_query.iter() {
        if interaction_box.x.contains(&transform.translation.x)
            && interaction_box.y.contains(&transform.translation.y)
        {
            let contact_position =
                player_direction.contact_position(player_position, player_radius);
            event_writer.send(InteractionEvent(player_entity, entity, contact_position));
        }
    }

    None
}
