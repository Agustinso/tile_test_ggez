use ggez::graphics::{DrawParam, Rect};
use ggez::glam::{Vec2, vec2};


pub struct TextureAtlas {
    pub texture_size: Vec2,
    pub image_size: Vec2,
    pub rows: u32,
    pub cols: u32,
    pub params: Vec<DrawParam>,
    pub opaque: Vec<bool>
}
impl TextureAtlas {
    pub fn new(texture_size: Vec2, image_size: Vec2) -> TextureAtlas {
        let cols = (image_size.x / texture_size.x) as u32;
        let rows = (image_size.y / texture_size.y) as u32;
        let mut params = vec![];

        let texture_span = vec2(texture_size.x/image_size.x, texture_size.y/image_size.y);

        for i in 0_usize..(rows*cols) as usize {
            let index_x = (i as u32 % cols) as f32;
            let index_y = (i as u32 / cols) as f32;
            let x = index_x * texture_span.x;
            let y = index_y * texture_span.y;
            let rect = Rect{ x, y, w: x+texture_span.x, h: y+texture_span.y };
            params.push(DrawParam::default().src(rect));
        }

        let opaque = vec![false; params.len()];
        TextureAtlas {
            texture_size,
            image_size,
            rows,
            cols,
            params,
            opaque,
        }
    }

    pub fn get_param(&self, index: usize) -> Option<DrawParam> {
        return self.params.get(index).copied()
    }

    pub fn get(&self, index: usize) -> Option<(DrawParam, bool)> {
        if let Some(rect) = self.params.get(index).copied() {
            let opaque = self.opaque.get(index).copied().unwrap();
            Some((rect, opaque))
        }
        else {
            None
        }
    }

    pub fn count(&self) -> u32 {
        self.cols * self.rows
    }
}