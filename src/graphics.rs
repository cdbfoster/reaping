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

use context::Context;
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

            for graphic_object in view_data.graphic_objects.iter().rev() {
                graphic_object.draw(context);
            }
        }

        context.sdl_renderer.present();
    }
}
