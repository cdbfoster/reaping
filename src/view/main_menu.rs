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

use sdl2::pixels::Color;
use sdl2::rect::Rect as SdlRectangle;

use context::Context;
use graphics::{GraphicObject, Sprite};
use input::Input;
use math::{Transform, Vector2};
use view::{View, ViewAction, ViewData};

pub struct MainMenuView {
    backdrop: Backdrop,
}

impl MainMenuView {
    pub fn new(context: &Context) -> MainMenuView {
        MainMenuView {
            backdrop: Backdrop::new(Color::RGB(64, 155, 0)),
        }
    }
}

impl View for MainMenuView {
    fn get_view_data(&self) -> ViewData {
        ViewData {
            graphic_objects: vec![
                &self.backdrop,
            ],
        }
    }

    fn update(&mut self, context: &Context, input: Vec<Input>, elapsed_ns: i64) -> (Option<ViewAction>, Vec<Input>) {
        (None, Vec::new())
    }
}

struct Backdrop {
    pub color: Color,
}

impl Backdrop {
    pub fn new(color: Color) -> Backdrop {
        Backdrop {
            color: color,
        }
    }
}

impl GraphicObject for Backdrop {
    fn draw(&self, context: &mut Context) {
        context.sdl_renderer.set_draw_color(self.color);

        let (width, height) = context.sdl_renderer.logical_size();
        context.sdl_renderer.fill_rect(
            SdlRectangle::new(
                0, 0,
                width, height,
            ).unwrap().unwrap()
        );
    }
}
