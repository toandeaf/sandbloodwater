use bevy::prelude::{Commands, Component, Query, Res, Time, Timer, Transform, With};
use bevy::time::TimerMode;

use crate::player::{CharacterMarker, CurrentDirection};

#[derive(Component)]
pub struct SyncTimer(pub Timer);

pub fn insert_sync_config(mut commands: Commands) {
    commands.spawn(SyncTimer(Timer::from_seconds(10., TimerMode::Repeating)));
}

pub fn sync(
    mut time_query: Query<&mut SyncTimer>,
    player_query: Query<(&Transform, &CharacterMarker, &CurrentDirection), With<CharacterMarker>>,
    time: Res<Time>,
) {
    // for mut timer in time_query.iter_mut() {
    //     timer.0.tick(time.delta());
    //
    //     if timer.0.finished() {
    //         for (transform, marker, direction) in player_query.iter() {
    //             let player_sync_event = EventWrapper::PlayerSync(PlayerSyncEvent(
    //                 marker.0,
    //                 transform.translation.xy(),
    //                 direction.0,
    //             ));
    //             dispatch_all(&player_sync_event);
    //         }
    //     }
    // }
}
