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
use input;
use input::{Button, Input};
use math::{Vector2, Rectangle};
use view::{View, ViewAction, ViewData};

pub struct MainMenuView {
    backdrop: Backdrop,

    title: Title,

    play_button: MenuButton,
    options_button: MenuButton,
    scores_button: MenuButton,
    quit_button: MenuButton,

    version: Sprite,
    author: Sprite,

    highlighted: Option<usize>,
    in_child_view: bool,
}

impl MainMenuView {
    pub fn new(context: &mut Context) -> MainMenuView {
        let info_font = context.font_renderer.load_font("assets/fonts/jim_teacher.ttf", context.rel.height(0.035) as u16);

        let button_size = Vector2::new(
            context.rel.width(0.5),
            context.rel.height(0.1),
        );

        let button_start = Vector2::new(
            context.rel.center_width(button_size.x),
            context.rel.height(0.47),
        );
        let button_pad = context.rel.height(0.022);

        MainMenuView {
            backdrop: Backdrop::new(),

            title: Title::new(context),

            play_button: MenuButton::new(context, "Play Game", Rectangle::new(
                Vector2::new(
                    button_start.x,
                    0 as f32 * (button_size.y + button_pad) + button_start.y,
                ),
                button_size,
            ).unwrap()),

            options_button: MenuButton::new(context, "Options", Rectangle::new(
                Vector2::new(
                    button_start.x,
                    1 as f32 * (button_size.y + button_pad) + button_start.y,
                ),
                button_size,
            ).unwrap()),

            scores_button: MenuButton::new(context, "Leaderboards", Rectangle::new(
                Vector2::new(
                    button_start.x,
                    2 as f32 * (button_size.y + button_pad) + button_start.y,
                ),
                button_size,
            ).unwrap()),

            quit_button: MenuButton::new(context, "Exit Game", Rectangle::new(
                Vector2::new(
                    button_start.x,
                    3 as f32 * (button_size.y + button_pad) + button_start.y,
                ),
                button_size,
            ).unwrap()),

            version: match context.font_renderer.render_sprite(context, &info_font, "v0.1", Color::RGB(255, 255, 255)) {
                Some(mut sprite) => {
                    sprite.transform.position.x = sprite.get_output_region().0.get_size().x / 2.0 + context.rel.height(0.008);
                    sprite.transform.position.y = context.screen_size.y - sprite.get_output_region().0.get_size().y / 2.0 + context.rel.height(0.005);
                    sprite
                },
                None => panic!("Could not render version text!"),
            },

            author: match context.font_renderer.render_sprite(context, &info_font, "©2016 Chris Foster", Color::RGB(255, 255, 255)) {
                Some(mut sprite) => {
                    sprite.transform.position.x = context.screen_size.x - sprite.get_output_region().0.get_size().x / 2.0 - context.rel.height(0.008);
                    sprite.transform.position.y = context.screen_size.y - sprite.get_output_region().0.get_size().y / 2.0 + context.rel.height(0.005);
                    sprite
                },
                None => panic!("Could not render author text!"),
            },

            highlighted: None,
            in_child_view: false,
        }
    }
}

impl View for MainMenuView {
    fn get_view_data(&self) -> ViewData {
        let mut graphic_objects = vec![
            &self.backdrop as &GraphicObject,
        ];

        if !self.in_child_view {
            graphic_objects.extend(vec![
                &self.title as &GraphicObject,

                &self.play_button,
                &self.options_button,
                &self.scores_button,
                &self.quit_button,

                &self.version,
                &self.author,
            ].iter());
        }

        ViewData {
            graphic_objects: graphic_objects,
        }
    }

    fn update(&mut self, context: &mut Context, input: Vec<Input>, _: i64) -> (Option<ViewAction>, Vec<Input>) {
        if self.in_child_view {
            if input.iter().find(|input| **input == Input::Pause).is_some() {
                self.in_child_view = false;
            } else {
                return (None, Vec::new());
            }
        }

        let mut menu_buttons: Vec<&mut MenuButton> = vec![
            &mut self.play_button,
            &mut self.options_button,
            &mut self.scores_button,
            &mut self.quit_button,
        ];

        let (highlighted, selected) = input::menu(
            &menu_buttons.iter().map(|x| *x as &Button).collect(), // Get a list of immutable buttons
            self.highlighted,
            &input,
        );

        self.highlighted = highlighted;
        for (menu_index, menu_button) in menu_buttons.iter_mut().enumerate() {
            match self.highlighted {
                Some(ref index) => {
                    if *index == menu_index {
                        menu_button.hover = true;
                    } else {
                        menu_button.hover = false;
                    }
                },
                None => { menu_button.hover = false; },
            }
        }

        let view_action = match selected {
            Some(ref index) => {
                if *index == 0 {
                    //Some(ViewAction::SetView(GameView::new(context))
                    None
                } else if *index == 1 {
                    self.in_child_view = true;
                    //Some(ViewAction::AddView(OptionsView::new(context))
                    None
                } else if *index == 2 {
                    self.in_child_view = true;
                    //Some(ViewAction::AddView(LeaderboardsView::new(context))
                    None
                } else {
                    Some(ViewAction::ExitGame)
                }
            },
            None => None,
        };


        (view_action, Vec::new())
    }
}

const BACKDROP: Color = Color::RGB(64, 155, 0);

struct Backdrop;

impl Backdrop {
    pub fn new() -> Backdrop {
        Backdrop
    }
}

impl GraphicObject for Backdrop {
    fn draw(&self, context: &mut Context) {
        context.sdl_renderer.set_draw_color(BACKDROP);

        context.sdl_renderer.fill_rect(
            SdlRectangle::new(
                0, 0,
                context.screen_size.x as u32, context.screen_size.y as u32,
            ).unwrap().unwrap()
        );
    }
}

struct Title {
    text_sprite: Sprite,
}

impl Title {
    pub fn new(context: &mut Context) -> Title {
        let title_font = context.font_renderer.load_font("assets/fonts/jim_teacher.ttf", context.rel.height(0.25) as u16);

        let text_sprite = match context.font_renderer.render_sprite(context, &title_font, "The Reaping", Color::RGB(255, 255, 255)) {
            Some(mut sprite) => {
                sprite.transform.position = Vector2::new(
                    context.rel.width(0.5),
                    context.rel.height(0.27),
                );
                sprite
            },
            None => panic!("Could not render title text!"),
        };

        Title {
            text_sprite: text_sprite,
        }
    }
}

impl GraphicObject for Title {
    fn draw(&self, context: &mut Context) {
        self.text_sprite.draw(context);
    }
}

const MENU_NORMAL: Color = Color::RGB(75, 185, 30);
const MENU_HIGHLIGHT: Color = Color::RGB(100, 210, 50);

struct MenuButton {
    text_sprite: Sprite,
    region: Rectangle,

    hover: bool,
}

impl MenuButton {
    pub fn new(context: &mut Context, text: &str, region: Rectangle) -> MenuButton {
        let menu_font = context.font_renderer.load_font("assets/fonts/fff_aquarius_bold.ttf", context.rel.height(0.06) as u16);

        let text_sprite = match context.font_renderer.render_sprite(context, &menu_font, text, Color::RGB(255, 255, 255)) {
            Some(mut sprite) => {
                sprite.transform.position = Vector2::new(
                    region.position.x + region.get_size().x / 2.0,
                    region.position.y + region.get_size().y / 2.0,
                );
                sprite
            },
            None => panic!("Could not render menu text: {}", text),
        };

        MenuButton {
            text_sprite: text_sprite,
            region: region,

            hover: false,
        }
    }
}

impl Button for MenuButton {
    fn get_region(&self) -> Rectangle {
        self.region
    }
}

impl GraphicObject for MenuButton {
    fn draw(&self, context: &mut Context) {
        if !self.hover {
            context.sdl_renderer.set_draw_color(MENU_NORMAL);
        } else {
            context.sdl_renderer.set_draw_color(MENU_HIGHLIGHT);
        }

        context.sdl_renderer.fill_rect(self.region.to_sdl_rectangle());

        self.text_sprite.draw(context);
    }
}
