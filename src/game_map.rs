use ggez::Context;
use ggez::context::Has;
use ggez::graphics::{self, DrawParam, InstanceArray, Drawable, Image, Rect};
use ggez::mint::Point2;

use crate::texture_atlas::TextureAtlas;

/// A tile in a layer.
/// Contains the rect for drawing
/// also the data for the game logic 
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TileData {
    /// Rect for drawing the tile
    pub param: DrawParam,
    /// Represents if the layers bellow are not visible 
    pub opaque: bool

}

impl TileData {
    pub fn new_with_index(index: usize, atlas: &TextureAtlas) -> Option<Self> {
        if let Some((param, opaque)) = atlas.get(index) {
            Some(Self {
                    param, 
                    opaque
            })
        }
        else {
            None
        }
    }
}



/// A single layer in the map.
/// Each item is a TileData, or None
/// if there is nothing to be drawn for that location,
/// Tiles are stored in row-major order.
#[derive(Clone, Debug, PartialEq)]
pub struct MapLayer {
    pub tiles: Vec<Option<TileData>>,
}

impl MapLayer {
    pub fn new(size: usize) -> Self {
        let tiles = vec![None; size];
        Self {
            tiles
        }
    }

    /// Returns the tile DrawParam at the given tile coordinate.
    fn get_tile(&self, x: usize, y: usize, width: usize) -> Option<TileData> {
        let offset = (y * width) + x;
        self.tiles[offset]
    }
    /// Sets the tile TileData at the given coordinate.
    fn set_tile(&mut self, tile: TileData, x: usize, y: usize, width: usize) {
        let offset = (y * width) + x;
        self.tiles[offset] = Some(tile);
    }
}

pub struct GameMap {
    /// Marks the map for reupdate
    changed: bool,

    pub layers: Vec<MapLayer>,
    /// Width of the map, in tiles
    pub width: usize,
    /// Height of the map, in tiles
    pub height: usize,

    /// A map from arbitrary ID's to `Tile`'s.
    pub tileset: TextureAtlas,

    /// The batching array for drawing visible tiles
    pub draw_array: InstanceArray,
}


impl GameMap {
    /// Low-level constructor for creating a `Map`.  You give it a set
    /// of layers and a `TileMap` you have already created.
    pub fn new(
        ctx: &mut ggez::Context,
        width: usize,
        height: usize,
        tileset: TextureAtlas,
        image: Image,
        layers: Vec<MapLayer>,
    ) -> Self {
        let draw_array = InstanceArray::new_ordered(ctx, image, (width*height).try_into().unwrap());

        Self {
            layers,
            changed: true,
            width,
            height,
            tileset,
            draw_array
        }
    }

    /// Given screen coords, returns map column and row
    pub fn coord_to_tile(&self, ctx: &Context, x: f32, y: f32) -> Option<(usize, usize)> {
        let (win_width, win_height) = Has::retrieve(&ctx).gfx.drawable_size();
        if (x >= 0. && x <= win_width) && (y >= 0. && y <= win_height) {
            let tile_x = x as usize / self.tileset.texture_size.x as usize;
            let tile_y = y as usize / self.tileset.texture_size.y as usize;
            Some((tile_x, tile_y))
        }
        else {
            None
        }
    }

    /// Given a tile col and row, return the coords of the top left corner of the tile
    pub fn tile_to_coord(&self, tile_col: usize, tile_row: usize) -> Option<Point2<f32>> {
        if tile_col <= self.width && tile_row <= self.height {
            let x:f32 = tile_col as f32 * self.tileset.texture_size.x;
            let y:f32 = tile_row as f32 * self.tileset.texture_size.y;
            Some(Point2{x,y})
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, layer: usize, param: DrawParam) {
        if let Some(dest) = self.tile_to_coord(x,y) {
            self.changed = true;
            let move_param = param.dest(dest);
            let tile =TileData{
                param: move_param, 
                opaque: true,
            };
            self.layers[layer].set_tile(tile, x, y, self.width);
        }
    }

    pub fn update(&mut self) {
        if !self.changed {return};
        self.draw_array.clear();
        for index in 0..(self.width*self.height) {
            for layer in self.layers.iter().rev() {
                if let Some(tile) = layer.tiles[index] {
                    self.draw_array.push(tile.param);
                    if tile.opaque { break }
                }
            }
        }
        self.changed = false;
    }
}

impl Drawable for GameMap {
    fn draw(&self, canvas: &mut graphics::Canvas, param: impl Into<DrawParam>) {
        self.draw_array.draw(canvas, param);
    }

    fn dimensions(&self, gfx: &impl ggez::context::Has<graphics::GraphicsContext>) -> Option<Rect> {
        self.draw_array.dimensions(gfx)
    }
}