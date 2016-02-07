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
use graphics::Sprite;
use input::Input;
use math::{Transform, Vector2};
use view::{View, ViewAction, ViewData};

pub struct MainMenuView;

impl MainMenuView {
    pub fn new(context: &Context) -> MainMenuView {
        MainMenuView
    }
}

impl View for MainMenuView {
    fn get_view_data(&self) -> ViewData {
        ViewData {
            graphic_objects: Vec::new(),
        }
    }

    fn update(&mut self, context: &Context, input: Vec<Input>) -> (Option<ViewAction>, Vec<Input>) {
        (None, Vec::new())
    }
}
