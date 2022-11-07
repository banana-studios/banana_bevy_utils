use crate::switch_in_game_state;
use bevy::{ecs::schedule::StateData, prelude::*};
use iyes_loopless::state::CurrentState;

pub trait StateNext: StateData {
    fn next(&self) -> Option<Self>;
}

pub trait StateSetNext {
    fn set_next(&mut self, commands: &mut Commands);
}

impl<T: StateNext> StateSetNext for State<T> {
    fn set_next(&mut self, _: &mut Commands) {
        let current = self.current();

        if let Some(next) = current.next() {
            bevy::log::info!("transitioning state from {:?} to {:?}", current, next);
            // TODO: change back to self.set()
            // Attempted to queue a state change, but there was already a state queued.
            if let Err(error) = self.overwrite_set(next) {
                bevy::log::error!("Error setting next state. {}", error);
            }
        } else {
            bevy::log::error!("no next state for {:?}.", current);
        }
    }
}

impl<T: StateNext> StateSetNext for ResMut<'_, CurrentState<T>> {
    fn set_next(&mut self, commands: &mut Commands) {
        let current = &self.0;

        if let Some(next) = current.next() {
            bevy::log::info!("transitioning state from {:?} to {:?}", current, next);
            switch_in_game_state!(commands, next);
        } else {
            bevy::log::error!("no next state for {:?}.", current);
        }
    }
}
