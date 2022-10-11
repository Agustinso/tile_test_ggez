use ggez::Context;
use ggez::GameResult;
use ggez::event;
use ggez::graphics::Color;
use ggez::graphics::Drawable;
use ggez::graphics::{self, DrawParam, Text};
use ggez::glam::*;

use crate::game_map::GameMap;
use crate::game_map::MapLayer;
use crate::game_map::TileData;
use crate::texture_atlas::TextureAtlas;

pub struct GameState {
    map: GameMap,
    curr_mouse_tile: (usize, usize),
    mouse_params: Vec<DrawParam>,
}

impl GameState {
    // Load images and create meshes.
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let tileset_img = graphics::Image::from_path(ctx, "/tile1.png")?;

        let tile_size = vec2(32., 32.);
        let image_size = vec2(64., 64.);

        let tile_set = TextureAtlas::new(tile_size, image_size);
        
        // Map size in tiles
        let map_size = (100, 100);

        

        // Make test layer with tile0 that fill the map
        let mut tiles = vec![];

        for index in 0..(map_size.0 * map_size.1) {
            let tile_col = index % map_size.0;
            let tile_row = index / map_size.0;
            let x:f32 = tile_col as f32 * tile_size.x;
            let y:f32 = tile_row as f32 * tile_size.y;

            tiles.push(Some(TileData{
                param: tile_set.get_param(0).unwrap().dest([x, y]), 
                opaque: true
            }));
        }

        let layer = MapLayer {
            tiles,
        };


        let layers = vec![layer];

        let map = GameMap::new(ctx, map_size.0, map_size.1, tile_set, tileset_img, layers);

        let state = GameState {
            map,
            curr_mouse_tile: (0,0),
            mouse_params: vec![],
        }; 
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        
        // Add mouse over tile
        if let Some(p) = self.map.tileset.get_param(3) {
            if let Some(dest) = self.map.tile_to_coord(self.curr_mouse_tile.0, self.curr_mouse_tile.1) {
                if self.mouse_params.is_empty() {
                    self.mouse_params.push(p.dest(dest));
                } else {
                    self.mouse_params[0] = p.dest(dest);
                }
                
            }
        }

        self.map.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgba(0x00, 0x00, 0x00, 0x00));


        canvas.draw(&self.map.draw_array, DrawParam::default().dest(vec2(0., 0.)));
        canvas.draw(&self.map.draw_array.image(), self.mouse_params[0]);

        Text::new(format!("{:.2}", ctx.time.fps())).draw(&mut canvas, DrawParam::new().dest(vec2(300.0, 10.0)));


        canvas.finish(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: event::MouseButton, _x: f32, _y: f32)  -> GameResult {
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: event::MouseButton, _x: f32, _y: f32)  -> GameResult {
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32)  -> GameResult {
        if let Some(pos) = self.map.coord_to_tile(ctx, x, y) {
            if pos != self.curr_mouse_tile {
                self.curr_mouse_tile = pos;
            }
        }
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32)  -> GameResult {
        println!("{:?}", (width, height));
        GameResult::Ok(())
    }
}