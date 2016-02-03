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

extern crate sdl2;
extern crate time;

mod context;
mod graphics;
mod input;
mod math;
//mod sound;
mod timer;
mod view;

use context::Context;
use graphics::GraphicRenderer;
use input::{Input, InputTranslator};
use timer::Timer;
use view::{View, ViewAction};

fn main() {
    let mut context = Context::new();

    let input_translator = InputTranslator::new();

    let graphic_renderer = GraphicRenderer::new();

    let mut views = Vec::<Box<View>>::new();
    let mut input = Vec::new();

    // Seed the timers
    let mut input_timer = Timer::new(1.0 / 30.0);
    let mut logic_timer = Timer::new(1.0 / 60.0);
    let mut render_timer = Timer::new(1.0 / 60.0);
    let mut fps_timer = Timer::new(1.0);

    let mut frames_rendered = 0;
    fps_timer.reset();

    let mut old_time = time::precise_time_ns();
    'main: loop {
        // Update the timers
        let elapsed_time = (time::precise_time_ns() - old_time) as i64;
        input_timer.elapsed(elapsed_time);
        logic_timer.elapsed(elapsed_time);
        render_timer.elapsed(elapsed_time);
        fps_timer.elapsed(elapsed_time);
        old_time += elapsed_time as u64;

        // Gather input
        if input_timer.sprung() {
            for event in context.sdl_event_pump.poll_iter() {
                match input_translator.translate(&event) {
                    Some(Input::Exit) => { break 'main; },
                    Some(i) => { input.push(i); },
                    None => {},
                }
            }

            input_timer.reset_with_overflow();
        }

        // Update views
        if logic_timer.sprung() {
            views = {
                let mut new_views = Vec::new();

                'view: for mut view in views {
                    let (result, pass_input) = view.update(&context, input);
                    input = pass_input;

                    match result {
                        Some(action) => {
                            match action {
                                ViewAction::SetView(set_view) => {
                                    new_views.clear();
                                    new_views.push(set_view);
                                    break 'view;
                                },
                                ViewAction::AddView(add_view) => {
                                    new_views.insert(0, add_view);
                                    new_views.push(view);
                                },
                                ViewAction::RemoveSelf => {},
                                ViewAction::ExitGame => { break 'main; },
                            }
                        },
                        None => {
                            new_views.push(view);
                        },
                    }
                }

                new_views
            };

            logic_timer.reset_with_overflow();
        }

        // Render graphics
        if render_timer.sprung() {
            graphic_renderer.render(&mut context, &views);
            frames_rendered += 1;

            render_timer.reset_with_overflow();
        }

        // Calculate FPS
        if fps_timer.sprung() {
            context.fps = frames_rendered;

            frames_rendered = 0;
            fps_timer.reset_with_overflow();
        }

        // Wait until the closest timer minus what it took to get here
        let sleep = {
            let remainders = vec![
                input_timer.remainder(),
                logic_timer.remainder(),
                render_timer.remainder(),
                fps_timer.remainder(),
            ];

            let min = *remainders.iter().min().unwrap() - (time::precise_time_ns() - old_time) as i64;
            if min > 0 { min as u32 } else { 0 }
        };
        std::thread::sleep(std::time::Duration::new(0, sleep));
    }
}
