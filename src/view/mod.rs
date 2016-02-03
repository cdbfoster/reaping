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
//use sound::SoundObject;

pub trait View {
    fn get_view_data(&self) -> &ViewData;
    fn get_view_data_mut(&mut self) -> &mut ViewData; // XXX Necessary?

    /// Any input returned will be passed to the next view.
    fn update(&mut self, context: &Context, input: Vec<Input>) -> (Option<ViewAction>, Vec<Input>);
}

pub struct ViewData {
    pub graphic_objects: Vec<Box<GraphicObject>>,
    //pub sound_objects: Vec<SoundObject>,
}

pub enum ViewAction {
    SetView(Box<View>),
    AddView(Box<View>),
    RemoveSelf,
    ExitGame,
}
