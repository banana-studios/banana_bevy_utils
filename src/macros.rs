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
    ($r:expr) => {
        |mut commands: Commands| {
            commands.insert_resource($r);
        }
    };

    ($commands:ident, $r:expr) => {
        $commands.insert_resource($r);
    };
}

#[macro_export]
macro_rules! switch_in_game_state {
    ($e:expr) => {
        |mut commands: Commands| {
            commands.insert_resource(iyes_loopless::prelude::NextState($e));
        }
    };

    ($commands:ident, $s:expr) => {
        $commands.insert_resource(iyes_loopless::prelude::NextState($s));
    };
}

#[macro_export]
macro_rules! spawn_component {
    ($c:expr) => {
        |mut commands: Commands| {
            commands.spawn($c);
        }
    };

    ($commands:ident, $c:expr) => {
        $commands.spawn($c);
    };
}

#[cfg(test)]
mod tests {

    mod stageless {
        use bevy::{
            ecs::system::{CommandQueue, Commands},
            prelude::World,
        };
        use iyes_loopless::prelude::NextState;

        #[derive(PartialEq, Debug)]
        pub enum GameState {
            A,
            B,
        }

        #[test]
        fn insert_resource() {
            let mut world = World::default();
            world.insert_resource(GameState::A);
            assert_eq!(GameState::A, *world.get_resource::<GameState>().unwrap());

            let mut queue = CommandQueue::default();
            let commands = Commands::new(&mut queue, &world);
            insert_resource!(GameState::B)(commands);
            queue.apply(&mut world);

            assert_eq!(GameState::B, *world.get_resource::<GameState>().unwrap());
        }

        #[test]
        fn insert_resource_with_commands() {
            let mut world = World::default();
            world.insert_resource(GameState::A);
            assert_eq!(GameState::A, *world.get_resource::<GameState>().unwrap());

            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, &world);
            insert_resource!(commands, GameState::B);
            queue.apply(&mut world);

            assert_eq!(GameState::B, *world.get_resource::<GameState>().unwrap());
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
            let commands = Commands::new(&mut queue, &world);
            switch_in_game_state!(GameState::B)(commands);
            queue.apply(&mut world);

            assert_eq!(
                NextState(GameState::B),
                *world.get_resource::<NextState<GameState>>().unwrap()
            );
        }

        #[test]
        fn switch_in_game_state_with_commands() {
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
