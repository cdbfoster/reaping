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

use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureQuery};
use sdl2_image::LoadTexture;
use sdl2_ttf::Font as SdlFont;
use sdl2_ttf::Sdl2TtfContext;

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

pub struct RelativeCoordinator {
    screen_size: Vector2,
}

impl RelativeCoordinator {
    pub fn new(screen_size: Vector2) -> RelativeCoordinator {
        RelativeCoordinator {
            screen_size: screen_size,
        }
    }

    /// Returns the value relative to the width of the screen
    pub fn width(&self, value: f32) -> f32 {
        self.screen_size.x * value
    }

    /// Returns the value relative to the height of the screen
    pub fn height(&self, value: f32) -> f32 {
        self.screen_size.y * value
    }

    /// Returns the X coordinate of an object of width pixels, when centered horizontally
    pub fn center_width(&self, width: f32) -> f32 {
        (self.screen_size.x - width) / 2.0
    }

    /// Returns the Y coordinate of an object of height pixels, when centered vertically
    pub fn center_height(&self, height: f32) -> f32 {
        (self.screen_size.y - height) / 2.0
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

    pub fn from_file(context: &Context, path: &str) -> Sprite {
        match context.sdl_renderer.load_texture(Path::new(path)).ok() {
            Some(texture) => Sprite::new(texture, None, None),
            None => panic!("Could not load sprite {}!", path),
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

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Font {
    pub path: &'static str,
    pub size: u16,
}

pub struct FontRenderer {
    sdl_ttf_context: Sdl2TtfContext,

    cached_fonts: HashMap<Font, SdlFont>,
}

impl FontRenderer {
    pub fn new(sdl_ttf_context: Sdl2TtfContext) -> FontRenderer {
        FontRenderer {
            sdl_ttf_context: sdl_ttf_context,

            cached_fonts: HashMap::new(),
        }
    }

    pub fn load_font(&mut self, font_path: &'static str, font_size: u16) -> Font{
        let font = Font {
            path: font_path,
            size: font_size,
        };

        if let Some(_) = self.cached_fonts.get(&font) {
            return font;
        }

        match self.sdl_ttf_context.load_font(Path::new(font.path), font.size).ok()
        {
            Some(sdl_font) => {
                self.cached_fonts.insert(font.clone(), sdl_font);
                font
            },
            None => {
                panic!("Could not load font {}, {}!", font.path, font.size);
            }
        }
    }

    pub fn render_sprite(&self, context: &Context, font: &Font, text: &str, color: Color) -> Option<Sprite> {
        if let Some(sdl_font) = self.cached_fonts.get(&font) {
            return sdl_font.render(text).blended(color).ok()
                .and_then(|surface| context.sdl_renderer.create_texture_from_surface(&surface).ok())
                .map(|texture| Sprite::new(texture, None, None))
        } else {
            None
        }
    }
}
