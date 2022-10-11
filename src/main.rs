//! An example of how to use an `InstanceArray`.
//!
//! You really want to run this one in release mode.
#![allow(clippy::unnecessary_wraps)]

mod texture_atlas;
mod sprite_sheet;
mod game_state;
mod game_map;

use game_state::GameState;
use ggez::event;
use ggez::GameResult;

use std::env;
use std::path;

// Creating a gamestate depends on having an SDL context to load resources.
// Creating a context depends on loading a config file.
// Loading a config file depends on having FS (or we can just fake our way around it
// by creating an FS and then throwing it away; the costs are not huge.)
pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };

    let cb = ggez::ContextBuilder::new("spritebatch", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}