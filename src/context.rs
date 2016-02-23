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

use sdl2;
use sdl2_image;
use sdl2_ttf;

use graphics::{FontRenderer, RelativeCoordinator};
use math::Vector2;

pub struct Context {
    pub sdl_context: sdl2::Sdl,
    pub sdl_event_pump: sdl2::EventPump,
    pub sdl_video: sdl2::VideoSubsystem,
    pub sdl_renderer: sdl2::render::Renderer<'static>,

    pub sdl_image_context: sdl2_image::Sdl2ImageContext,

    pub font_renderer: FontRenderer,

    pub screen_size: Vector2,
    pub rel: RelativeCoordinator,

    pub fps: u32,
}

impl Context {
    pub fn new() -> Context {
        let sdl_context = sdl2::init().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        let sdl_video = sdl_context.video().unwrap();

        // XXX Read window options out of a config file
        let (width, height) = (1366, 768);
        let mut sdl_renderer = {
            let mut builder = sdl_video.window("The Reaping", width, height);

            builder.opengl();

            //builder.fullscreen();

            let sdl_window = builder.build().unwrap();
            sdl_window.renderer().accelerated().build().unwrap()
        };

        sdl_renderer.set_logical_size(width, height).ok();

        let sdl_image_context = sdl2_image::init(sdl2_image::INIT_PNG).unwrap();

        let font_renderer = {
            let sdl_ttf_context = sdl2_ttf::init().unwrap();

            FontRenderer::new(sdl_ttf_context)
        };

        let screen_size = {
            let (width, height) = sdl_renderer.logical_size();
            Vector2::new(width as f32, height as f32)
        };

        let rel = RelativeCoordinator::new(screen_size);

        Context {
            sdl_context: sdl_context,
            sdl_event_pump: sdl_event_pump,
            sdl_video: sdl_video,
            sdl_renderer: sdl_renderer,

            sdl_image_context: sdl_image_context,

            font_renderer: font_renderer,

            screen_size: screen_size,
            rel: rel,

            fps: 0,
        }
    }
}
