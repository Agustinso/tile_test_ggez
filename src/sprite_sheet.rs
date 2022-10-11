use ggez::graphics::{Image, DrawParam};
use ggez::glam::Vec2;
use crate::texture_atlas::TextureAtlas;

pub struct SpriteSheet {
    pub atlas: TextureAtlas,
    pub image: Image
}
impl SpriteSheet {
    pub fn new(texture_size: Vec2, image_size: Vec2, image: Image) -> SpriteSheet {
        let atlas = TextureAtlas::new(texture_size, image_size);
        SpriteSheet {
            atlas,
            image
        }
    }

    pub fn get_sprite(&self, index: usize) -> Option<DrawParam> {
        let sprite = &self.atlas.get_param(index);
        *sprite
    }
}