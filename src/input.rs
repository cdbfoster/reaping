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

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::Mouse;

use math::{Rectangle, Vector2};

#[derive(PartialEq)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Enter,
    MouseMotion(Vector2),
    JoystickMotion(Vector2),
    Pause,
    Exit,
}

pub struct InputTranslator;

impl InputTranslator {
    pub fn new() -> InputTranslator {
        InputTranslator
    }

    pub fn translate(&self, event: &Event) -> Option<Input> {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::W), .. } |
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                Some(Input::Up)
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } |
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                Some(Input::Down)
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } |
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                Some(Input::Left)
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } |
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                Some(Input::Right)
            },
            Event::MouseButtonDown { mouse_btn: Mouse::Left, .. } |
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                Some(Input::Enter)
            },
            Event::MouseMotion { x, y, .. } => {
                Some(Input::MouseMotion(Vector2::new(x as f32, y as f32)))
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Some(Input::Pause)
            },
            Event::Quit {..} => Some(Input::Exit),
            _ => None,
        }
    }
}

pub trait Button {
    fn get_region(&self) -> Rectangle;
}

/// Returns the new highlighted item and the selected item
pub fn menu(items: &Vec<&Button>, mut highlighted: Option<usize>, input: &Vec<Input>) -> (Option<usize>, Option<usize>) {
    let mouse_position = input.iter().filter_map(|input| match *input {
        Input::MouseMotion(position) => Some(position),
        _ => None,
    }).last();

    let mut mouse_highlighted = None;
    for (index, button) in items.iter().enumerate() {
        match mouse_position {
            Some(ref position) => {
                if button.get_region().contains(*position) {
                    mouse_highlighted = Some(index);
                }
            },
            None => (),
        }
    }

    if mouse_position.is_some() {
        highlighted = mouse_highlighted;
    }

    let mut selected = None;

    for i in input.iter() {
        match *i {
            Input::Up => {
                highlighted = match highlighted {
                    Some(index) => {
                        if index == 0 {
                            Some(items.len() - 1)
                        } else {
                            Some(index - 1)
                        }
                    },
                    None => { Some(0) },
                };
            },
            Input::Down => {
                highlighted = match highlighted {
                    Some(index) => {
                        if index == items.len() - 1 {
                            Some(0)
                        } else {
                            Some(index + 1)
                        }
                    },
                    None => { Some(0) },
                };
            },
            Input::Enter => {
                selected = highlighted;
            }
            _ => (),
        }
    }

    (highlighted, selected)
}
