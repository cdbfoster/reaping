//
// This file is part of The Reaping.
//
// The Reaping is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// The Reaping is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with The Reaping. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright 2016 Chris Foster
//

use std::path::Path;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureQuery};
use sdl2_image::LoadTexture;

use context::Context;
use math::{Rectangle, Transform, Vector2};
use view::View;

pub trait GraphicObject {
    fn draw(&self, context: &mut Context);
}

pub struct GraphicRenderer;

impl GraphicRenderer {
    pub fn new() -> GraphicRenderer {
        GraphicRenderer
    }

    pub fn render(&self, context: &mut Context, views: &Vec<Box<View>>) {
        context.sdl_renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.sdl_renderer.clear();

        for view in views.iter().rev() {
            let view_data = view.get_view_data();

            for graphic_object in view_data.graphic_objects.iter() {
                graphic_object.draw(context);
            }
        }

        context.sdl_renderer.present();
    }
}

#[derive(Clone)]
pub struct Sprite {
    pub transform: Transform,

    sdl_texture: Rc<Texture>,
    region: Rectangle,
}

impl Sprite {
    pub fn new(sdl_texture: Texture, transform: Option<Transform>, region: Option<Rectangle>) -> Sprite {
        Sprite {
            transform: match transform {
                Some(t) => t,
                None => Transform::zero(),
            },
            region: match region {
                Some(r) => r,
                None => {
                    // If no region is given, grab the size out of the texture
                    let TextureQuery { width, height, .. } = sdl_texture.query();

                    Rectangle::new(
                        Vector2::zero(),
                        Vector2::new(width as f32, height as f32)
                    ).unwrap()
                },
            },
            sdl_texture: Rc::new(sdl_texture),
        }
    }

    pub fn from_sprite(sprite: &Sprite, transform: Option<Transform>, region: Option<Rectangle>) -> Sprite {
        Sprite {
            transform: match transform {
                Some(t) => t,
                None => Transform::zero(),
            },
            region: match region {
                Some(r) => r,
                None => {
                    // If no region is given, grab the size out of the texture
                    let TextureQuery { width, height, .. } = sprite.sdl_texture.query();

                    Rectangle::new(
                        Vector2::zero(),
                        Vector2::new(width as f32, height as f32)
                    ).unwrap()
                },
            },
            sdl_texture: sprite.sdl_texture.clone(),
        }
    }

    pub fn from_file(context: &Context, path: &str) -> Option<Sprite> {
        match context.sdl_renderer.load_texture(Path::new(path)).ok() {
            Some(texture) => Some(Sprite::new(texture, None, None)),
            None => None,
        }
    }

    pub fn get_region(&self) -> Rectangle {
        self.region
    }

    pub fn get_output_region(&self) -> Rectangle {
        let (rectangle, _) = self.region.transform(&self.transform);

        rectangle
    }
}

impl GraphicObject for Sprite {
    fn draw(&self, context: &mut Context) {
        let (output_region, flip) = self.region.transform(&self.transform);

        context.sdl_renderer.copy_ex(
            &self.sdl_texture,
            Some(self.region.to_sdl_rectangle()),
            Some(output_region.to_sdl_rectangle()),
            self.transform.rotation as f64,
            None,
            flip,
        );
    }
}
