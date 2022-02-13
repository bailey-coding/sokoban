use ggez::event::KeyCode;
use specs::World;

use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::time::Duration;

use crate::audio::AudioStore;
use crate::events::Event;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: VecDeque<KeyCode>,
}

#[derive(PartialEq)]
pub enum GameplayState {
  Playing,
  Won
}

impl Default for GameplayState {
  fn default() -> Self {
      Self::Playing
  }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32
}

impl Display for GameplayState {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
      fmt.write_str(match self {
          GameplayState::Playing => "Playing",
          GameplayState::Won => "Won"
      })?;
      Ok(())
  }
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
}
