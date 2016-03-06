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
use graphics::GraphicObject;
use math::{Rectangle, Vector2};

use view::game::GRID_CELL_SIZE;

pub struct Ground;

impl Ground {
    pub fn new() -> Ground {
        Ground
    }
}

const GROUND: Color = Color::RGB(64, 155, 0);

impl GraphicObject for Ground {
    fn draw(&self, context: &mut Context) {
        context.sdl_renderer.set_draw_color(GROUND);
        
        context.sdl_renderer.fill_rect(
            SdlRectangle::new(
                0, 0,
                context.screen_size.x as u32, context.screen_size.y as u32,
            ).unwrap().unwrap()
        );
    }
}

pub struct Road {
    pub region: Rectangle,
}

impl Road {
    pub fn new(context: &Context) -> Road {
        Road {
            region: Rectangle::new(
                Vector2::new(0.0, context.rel.height(7.0 * GRID_CELL_SIZE.y)),
                Vector2::new(context.screen_size.x, context.rel.height(5.0 * GRID_CELL_SIZE.y)),
            ).unwrap(),
        }
    }
}

const ROAD: Color = Color::RGB(80, 80, 80);

impl GraphicObject for Road {
    fn draw(&self, context: &mut Context) {
        context.sdl_renderer.set_draw_color(ROAD);
        
        context.sdl_renderer.fill_rect(
            self.region.to_sdl_rectangle()
        );
    }
}

pub struct River {
    pub region: Rectangle,
}

impl River {
    pub fn new(context: &Context) -> River {
        River {
            region: Rectangle::new(
                Vector2::new(0.0, context.rel.height(GRID_CELL_SIZE.y)),
                Vector2::new(context.screen_size.x, context.rel.height(5.0 * GRID_CELL_SIZE.y)),
            ).unwrap(),
        }
    }
}

const RIVER: Color = Color::RGB(0, 0, 128);

impl GraphicObject for River {
    fn draw(&self, context: &mut Context) {
        context.sdl_renderer.set_draw_color(RIVER);
        
        context.sdl_renderer.fill_rect(
            self.region.to_sdl_rectangle()
        );
    }
}
