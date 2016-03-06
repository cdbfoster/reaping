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
use graphics::GraphicObject;
use input::Input;
use math::Vector2;
use view;
use view::{View, ViewAction, ViewData};

use self::actors::Frog;
use self::terrain::{Ground, River, Road};

const GRID_SIZE: Vector2 = Vector2 {
    x: 1.2,
    y: 1.0,
};

const GRID_DIM: Vector2 = Vector2 {
    x: 15.0,
    y: 13.0,
};

const GRID_CELL_SIZE: Vector2 = Vector2 {
    x: GRID_SIZE.x / GRID_DIM.x,
    y: GRID_SIZE.y / GRID_DIM.y,
};

pub struct GameView {
    ground: Ground,
    road: Road,
    river: River,
    
    frog: Frog,
}

impl GameView {
    pub fn new(context: &mut Context) -> GameView {
        GameView {
            ground: Ground::new(),
            road: Road::new(context),
            river: River::new(context),
            
            frog: Frog::new(context),
        }
    }
}

impl View for GameView {
    fn get_view_data(&self) -> ViewData {
        let mut graphic_objects = vec![
            &self.ground as &GraphicObject,
            &self.road,
            &self.river,
            
            &self.frog,
        ];
        
        ViewData {
            graphic_objects: graphic_objects,
        }
    }
    
    fn update(&mut self, context: &mut Context, input: Vec<Input>, elapsed_ns: i64) -> (Option<ViewAction>, Vec<Input>) {
        if input.iter().find(|input| **input == Input::Pause).is_some() {
            return (Some(ViewAction::SetView(Box::new(view::MainMenuView::new(context)))), Vec::new());
        }
        
        (None, Vec::new())
    }
}

mod actors;
mod terrain;
