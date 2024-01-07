use crate::player::component::{Activity, CurrentActivity, Player};
use crate::player::system::drop::DropEvent;
use bevy::prelude::*;

pub fn process_drop(
    mut commands: Commands,
    mut event_reader: EventReader<DropEvent>,
    mut player_query: Query<(&mut CurrentActivity, &Children), With<Player>>,
) {
    for event in event_reader.read() {
        // Getting the player's current activity and entity children
        let (mut current_activity, children) = player_query.get_mut(event.0).unwrap();

        // If the player is already carrying something, pressing this button will make them "drop" it.
        if let Activity::Carrying = current_activity.0 {
            for child in children.iter() {
                // Detach the item child from its player parent, but maintain the global transform
                // of the item.
                commands.entity(*child).remove_parent_in_place();
            }

            // Update the player's activity to reflect that they're no longer carrying.
            current_activity.0 = Activity::Idle;
        };

        // TODO re-implement dropping of "equipped"/collected items once inventory/equipped is implemented.
    }
}
