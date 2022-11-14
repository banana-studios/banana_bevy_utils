use bevy::prelude::*;

mod chain;
mod macros;

// #[cfg(feature = "2d")]
// pub mod d2;
#[cfg(feature = "rng")]
pub mod rng;
#[cfg(feature = "states")]
pub mod state;
#[cfg(feature = "bevy_ui")]
pub mod ui;

pub mod prelude {
    pub use crate::chain::*;

    pub use crate::despawn_with;
    pub use crate::despawn_with_recursive;
    pub use crate::remove_from_all;
    pub use crate::remove_from_all_with;
    pub use crate::remove_resource;
    pub use crate::{
        despawn_children, impl_default, impl_new, insert_resource, spawn_component,
        switch_in_game_state,
    };

    #[cfg(feature = "states")]
    pub use crate::state::*;
}

//////////////////////////////////////////////////////////////////////////////////////////
// Query Manipulation
//////////////////////////////////////////////////////////////////////////////////////////

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).despawn();
    }
}

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_children<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).despawn_descendants();
    }
}

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with_recursive<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).despawn_recursive();
    }
}

/// Remove a component type from all entities that have it
pub fn remove_from_all<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).remove::<T>();
    }
}

/// Remove a component type from any entities with some other component
pub fn remove_from_all_with<T: Component, W: Component>(
    mut cmd: Commands,
    q: Query<Entity, (With<T>, With<W>)>,
) {
    for e in q.iter() {
        cmd.entity(e).remove::<T>();
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// Resources
//////////////////////////////////////////////////////////////////////////////////////////

/// Remove a resource using Commands
pub fn remove_resource<T: Resource>(mut cmd: Commands) {
    cmd.remove_resource::<T>();
}
