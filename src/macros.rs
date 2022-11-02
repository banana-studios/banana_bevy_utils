#[macro_export]
macro_rules! impl_new{
  ($to:ty,$($v:ident: $t:ty),*)  => {
      impl $to {
          pub fn new($($v: $t),*) -> $to
          {
              Self {
                  $($v),*
              }
          }
      }
  };
}

#[macro_export]
macro_rules! impl_default {
    ($to:ty) => {
        impl Default for $to {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

#[macro_export]
macro_rules! insert_resource {
    ($e:expr) => {
        |mut commands: Commands| {
            commands.insert_resource($e);
        }
    };
}

#[macro_export]
macro_rules! switch_in_game_state {
    ($e:expr) => {
        |mut commands: Commands| {
            commands.insert_resource(NextState($e));
        }
    };

    ($commands:ident, $s:expr) => {
        $commands.insert_resource(NextState($s));
    };
}

#[cfg(test)]
mod tests {

    mod stageless {
        use super::*;
        use bevy::{
            ecs::system::{CommandQueue, Commands},
            prelude::World,
        };
        use iyes_loopless::prelude::NextState;

        #[derive(PartialEq, Debug)]
        pub enum GameState {
            A,
            B,
            C,
        }

        #[test]
        fn switch_in_game_state() {
            let mut world = World::default();
            world.insert_resource(NextState(GameState::A));
            assert_eq!(
                NextState(GameState::A),
                *world.get_resource::<NextState<GameState>>().unwrap()
            );

            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);
            switch_in_game_state!(GameState::B)(commands);
            queue.apply(&mut world);

            assert_eq!(
                NextState(GameState::B),
                *world.get_resource::<NextState<GameState>>().unwrap()
            );
        }

        #[test]
        fn switch_in_game_state_2() {
            let mut world = World::default();
            world.insert_resource(NextState(GameState::A));
            assert_eq!(
                NextState(GameState::A),
                *world.get_resource::<NextState<GameState>>().unwrap()
            );

            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);
            switch_in_game_state!(commands, GameState::B);
            queue.apply(&mut world);
            assert_eq!(
                NextState(GameState::B),
                *world.get_resource::<NextState<GameState>>().unwrap()
            );
        }
    }
}
