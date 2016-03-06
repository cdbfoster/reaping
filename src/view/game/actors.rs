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

use context::Context;
use graphics::{GraphicObject, Sprite};
use math::{Transform, Vector2};

use view::game::GRID_CELL_SIZE;

pub struct Frog {
    sprite: Sprite,
}

impl Frog {
    pub fn new(context: &Context) -> Frog {
        Frog {
            sprite: {
                let mut sprite = Sprite::from_file(context, "assets/graphics/frog.png");
                let scale = context.rel.height(GRID_CELL_SIZE.y) / sprite.get_region().get_size().y;
                sprite.transform.scale = Vector2::new(scale, scale);
                sprite
            },
        }
    }
    
    pub fn get_transform(&self) -> &Transform {
        &self.sprite.transform
    }
    
    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.sprite.transform
    }
}

impl GraphicObject for Frog {
    fn draw(&self, context: &mut Context) {
        self.sprite.draw(context);
    }
}
