use bevy::prelude::{BuildChildrenTransformExt, Commands, EventReader, Query, Transform, With};

use crate::item::{InteractionType, Interactive, Item};
use crate::player::component::{Activity, CurrentActivity, Player};
use crate::player::system::interact::InteractionEvent;

pub fn process_interaction(
    mut commands: Commands,
    mut event_reader: EventReader<InteractionEvent>,
    mut player_query: Query<&mut CurrentActivity, With<Player>>,
    mut item_query: Query<(&Interactive, &mut Transform), With<Item>>,
) {
    for event in event_reader.read() {
        let mut player_activity = player_query.get_mut(event.0).unwrap();

        if let Activity::Carrying = player_activity.0 {
            return;
        }

        // TODO implement the "picked up" transform
        let (interactive, mut transform) = item_query.get_mut(event.1).unwrap();

        match interactive.0 {
            InteractionType::Collect => {
                commands
                    .get_entity(event.1)
                    .iter_mut()
                    .for_each(|entity| entity.despawn());
            }
            InteractionType::Carry => {
                let mut item_entity = commands.entity(event.1);

                item_entity.set_parent_in_place(event.0);

                player_activity.0 = Activity::Carrying;
            }
        }
    }
}
