pub mod eventqueue;
pub mod gamemanager; // tells Rust that util has a submodule input.rs
pub mod gamepayload;

#[path = "input/input.rs"]
pub mod input;
pub mod physics;
pub mod scoremanager;
pub mod spawnmanager;

#[path = "state/state.rs"]
pub mod state;

#[path = "state/world.rs"]
pub mod world;

pub mod util;
