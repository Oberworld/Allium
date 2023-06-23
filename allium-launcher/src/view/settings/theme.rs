use std::collections::VecDeque;
use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use common::command::Command;
use common::constants::{BUTTON_DIAMETER, SELECTION_HEIGHT};
use common::geom::{Alignment, Point, Rect};
use common::platform::{DefaultPlatform, Key, KeyEvent, Platform};
use common::stylesheet::{Stylesheet, StylesheetFont};
use common::view::{ButtonHint, ColorPicker, Number, Row, Select, SettingsList, Toggle, View};
use tokio::sync::mpsc::Sender;

pub struct Theme {
    rect: Rect,
    stylesheet: Stylesheet,
    fonts: Vec<PathBuf>,
    list: SettingsList,
    button_hints: Row<ButtonHint<String>>,
}

impl Theme {
    pub fn new(rect: Rect) -> Self {
        let stylesheet = Stylesheet::load().unwrap();

        let fonts = StylesheetFont::available_fonts().unwrap_or_default();
        let font_names: Vec<String> = fonts
            .iter()
            .map(|p| {
                p.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .replace('_', " ")
                    .replace('-', " ")
            })
            .collect();

        let list = SettingsList::new(
            Rect::new(rect.x, rect.y + 8, rect.w - 12, rect.h - 8 - 46),
            vec![
                "Dark Mode".to_string(),
                "UI Font".to_string(),
                "UI Font Size".to_string(),
                "Guide Font".to_string(),
                "Guide Font Size".to_string(),
                "Highlight Color".to_string(),
                "Foreground Color".to_string(),
                "Background Color".to_string(),
                "Disabled Color".to_string(),
                "Button A Color".to_string(),
                "Button B Color".to_string(),
                "Button X Color".to_string(),
                "Button Y Color".to_string(),
            ],
            vec![
                Box::new(Toggle::new(
                    Point::zero(),
                    stylesheet.background_color.is_dark(),
                    Alignment::Right,
                )),
                Box::new(Select::new(
                    Point::zero(),
                    fonts
                        .iter()
                        .position(|p| *p == stylesheet.ui_font.path)
                        .unwrap_or_default(),
                    font_names.clone(),
                    Alignment::Right,
                )),
                Box::new(Number::new(
                    Point::zero(),
                    stylesheet.ui_font.size as i32,
                    10,
                    40,
                    Alignment::Right,
                )),
                Box::new(Select::new(
                    Point::zero(),
                    fonts
                        .iter()
                        .position(|p| *p == stylesheet.guide_font.path)
                        .unwrap_or_default(),
                    font_names,
                    Alignment::Right,
                )),
                Box::new(Number::new(
                    Point::zero(),
                    stylesheet.guide_font.size as i32,
                    10,
                    40,
                    Alignment::Right,
                )),
                Box::new(ColorPicker::new(
                    Point::zero(),
                    stylesheet.highlight_color,
                    Alignment::Right,
                )),
                Box::new(ColorPicker::new(
                    Point::zero(),
                    stylesheet.foreground_color,
                    Alignment::Right,
                )),
                Box::new(ColorPicker::new(
                    Point::zero(),
                    stylesheet.background_color,
                    Alignment::Right,
                )),
                Box::new(ColorPicker::new(
                    Point::zero(),
                    stylesheet.disabled_color,
                    Alignment::Right,
                )),
                Box::new(ColorPicker::new(
                    Point::zero(),
                    stylesheet.button_a_color,
                    Alignment::Right,
                )),
                Box::new(ColorPicker::new(
                    Point::zero(),
                    stylesheet.button_b_color,
                    Alignment::Right,
                )),
                Box::new(ColorPicker::new(
                    Point::zero(),
                    stylesheet.button_x_color,
                    Alignment::Right,
                )),
                Box::new(ColorPicker::new(
                    Point::zero(),
                    stylesheet.button_y_color,
                    Alignment::Right,
                )),
            ],
            SELECTION_HEIGHT,
        );

        let button_hints = Row::new(
            Point::new(
                rect.x + rect.w as i32 - 12,
                rect.y + rect.h as i32 - BUTTON_DIAMETER as i32 - 8,
            ),
            vec![
                ButtonHint::new(Point::zero(), Key::A, "Edit".to_owned(), Alignment::Right),
                ButtonHint::new(Point::zero(), Key::B, "Back".to_owned(), Alignment::Right),
            ],
            Alignment::Right,
            12,
        );

        Self {
            rect,
            stylesheet,
            fonts,
            list,
            button_hints,
        }
    }
}

#[async_trait(?Send)]
impl View for Theme {
    fn draw(
        &mut self,
        display: &mut <DefaultPlatform as Platform>::Display,
        styles: &Stylesheet,
    ) -> Result<bool> {
        let mut drawn = false;

        if self.list.should_draw() && self.list.draw(display, styles)? {
            drawn = true;
        }

        if self.button_hints.should_draw() && self.button_hints.draw(display, styles)? {
            drawn = true;
        }

        Ok(drawn)
    }

    fn should_draw(&self) -> bool {
        self.list.should_draw() || self.button_hints.should_draw()
    }

    fn set_should_draw(&mut self) {
        self.list.set_should_draw();
        self.button_hints.set_should_draw();
    }

    async fn handle_key_event(
        &mut self,
        event: KeyEvent,
        commands: Sender<Command>,
        bubble: &mut VecDeque<Command>,
    ) -> Result<bool> {
        if self
            .list
            .handle_key_event(event, commands.clone(), bubble)
            .await?
        {
            while let Some(command) = bubble.pop_front() {
                if let Command::ValueChanged(i, val) = command {
                    match i {
                        0 => match val.as_bool().unwrap() {
                            true => {
                                if !self.stylesheet.background_color.is_dark() {
                                    self.stylesheet.foreground_color =
                                        self.stylesheet.foreground_color.invert();
                                    self.stylesheet.background_color =
                                        self.stylesheet.background_color.invert();
                                    self.list.set_child(
                                        3,
                                        Box::new(ColorPicker::new(
                                            Point::zero(),
                                            self.stylesheet.foreground_color,
                                            Alignment::Right,
                                        )),
                                    );
                                    self.list.set_child(
                                        4,
                                        Box::new(ColorPicker::new(
                                            Point::zero(),
                                            self.stylesheet.background_color,
                                            Alignment::Right,
                                        )),
                                    );
                                }
                            }
                            false => {
                                if self.stylesheet.background_color.is_dark() {
                                    self.stylesheet.foreground_color =
                                        self.stylesheet.foreground_color.invert();
                                    self.stylesheet.background_color =
                                        self.stylesheet.background_color.invert();
                                    self.list.set_child(
                                        3,
                                        Box::new(ColorPicker::new(
                                            Point::zero(),
                                            self.stylesheet.foreground_color,
                                            Alignment::Right,
                                        )),
                                    );
                                    self.list.set_child(
                                        4,
                                        Box::new(ColorPicker::new(
                                            Point::zero(),
                                            self.stylesheet.background_color,
                                            Alignment::Right,
                                        )),
                                    );
                                }
                            }
                        },
                        1 => {
                            self.stylesheet.ui_font.path =
                                self.fonts[val.as_int().unwrap() as usize].clone()
                        }
                        2 => self.stylesheet.ui_font.size = val.as_int().unwrap() as u32,
                        3 => {
                            self.stylesheet.guide_font.path =
                                self.fonts[val.as_int().unwrap() as usize].clone()
                        }
                        4 => self.stylesheet.guide_font.size = val.as_int().unwrap() as u32,
                        5 => self.stylesheet.highlight_color = val.as_color().unwrap(),
                        6 => self.stylesheet.foreground_color = val.as_color().unwrap(),
                        7 => self.stylesheet.background_color = val.as_color().unwrap(),
                        8 => self.stylesheet.disabled_color = val.as_color().unwrap(),
                        9 => self.stylesheet.button_a_color = val.as_color().unwrap(),
                        10 => self.stylesheet.button_b_color = val.as_color().unwrap(),
                        11 => self.stylesheet.button_x_color = val.as_color().unwrap(),
                        12 => self.stylesheet.button_y_color = val.as_color().unwrap(),
                        _ => unreachable!("Invalid index"),
                    }

                    commands
                        .send(Command::SaveStylesheet(Box::new(self.stylesheet.clone())))
                        .await?;
                }
            }
            return Ok(true);
        }

        match event {
            KeyEvent::Pressed(Key::B) => {
                bubble.push_back(Command::CloseView);
                Ok(true)
            }
            _ => Ok(true),
        }
    }

    fn children(&self) -> Vec<&dyn View> {
        vec![&self.list, &self.button_hints]
    }

    fn children_mut(&mut self) -> Vec<&mut dyn View> {
        vec![&mut self.list, &mut self.button_hints]
    }

    fn bounding_box(&mut self, _styles: &Stylesheet) -> Rect {
        self.rect
    }

    fn set_position(&mut self, _point: Point) {
        unimplemented!()
    }
}
