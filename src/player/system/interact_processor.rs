use bevy::prelude::{BuildChildrenTransformExt, Commands, EventReader, Query, Transform, With};

use crate::item::{InteractionType, Interactive, Item};
use crate::player::component::{Activity, CurrentActivity, Player};
use crate::player::system::interact::InteractionEvent;

pub fn process_interact(
    mut commands: Commands,
    mut event_reader: EventReader<InteractionEvent>,
    mut player_query: Query<&mut CurrentActivity, With<Player>>,
    mut item_query: Query<(&Interactive, &mut Transform), With<Item>>,
) {
    for event in event_reader.read() {
        let mut player_activity = player_query.get_mut(event.0).unwrap();

        // If the player is carrying something, they can't interact. So yeet out.
        if let Activity::Carrying = player_activity.0 {
            return;
        }

        // Get the item components from the target item entity supplied by the event.
        let (interactive, mut transform) = item_query.get_mut(event.1).unwrap();

        // Match what happens to the item based on how it is interactive.
        // Might revisit this abstraction in the future.
        match interactive.0 {
            InteractionType::Collect => {
                // Collecting currently just involves despawning the selected entity.
                // In the future not only will we despawn it, but we'll update the player's inventory
                // with the data of said item.
                commands
                    .get_entity(event.1)
                    .iter_mut()
                    .for_each(|entity| entity.despawn());
            }
            InteractionType::Carry => {
                // Get it item entity from the event
                let mut item_entity = commands.entity(event.1);

                // Assign "carrying" location (relative to parent) to item.
                transform.translation = event.2;

                // Attach to player component
                item_entity.set_parent_in_place(event.0);

                // Update player's current activity to reflect carrying state.
                player_activity.0 = Activity::Carrying;
            }
        }
    }
}
