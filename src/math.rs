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

use std::ops::{Add, Sub, Mul, Neg, Div};

use sdl2::rect::Rect as SdlRectangle;

#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x: x,
            y: y,
        }
    }

    pub fn zero() -> Vector2 {
        Vector2 {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn transform(&self, t: &Transform) -> Vector2 {
        Vector2::new(
            self.x * t.scale.x * t.rotation.cos() - self.y * t.scale.y * t.rotation.sin() + t.position.x,
            self.x * t.scale.x * t.rotation.sin() + self.y * t.scale.y * t.rotation.cos() + t.position.y,
        )
    }

    pub fn abs(&self) -> Vector2 {
        Vector2::new(
            self.x.abs(),
            self.y.abs(),
        )
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub scale: Vector2,
    pub rotation: f32,
    pub position: Vector2,
}

impl Transform {
    pub fn new(scale: Vector2, rotation: f32, position: Vector2) -> Transform {
        Transform {
            scale: scale,
            rotation: rotation,
            position: position,
        }
    }

    pub fn zero() -> Transform {
        Transform {
            scale: Vector2::new(1.0, 1.0),
            rotation: 0.0,
            position: Vector2::zero(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub position: Vector2,

    size: Vector2,
}

impl Rectangle {
    pub fn new(position: Vector2, size: Vector2) -> Option<Rectangle> {
        if size.x < 0.0 || size.y < 0.0 {
            None
        } else {
            Some(Rectangle {
                position: position,
                size: size,
            })
        }
    }

    pub fn get_size(&self) -> Vector2 {
        self.size
    }

    pub fn set_size(&mut self, size: Vector2) {
        self.size = Vector2::new(
            if size.x < 0.0 { 0.0 } else { size.x },
            if size.y < 0.0 { 0.0 } else { size.y },
        );
    }

    /// Returns the scaled and positioned rectangle and bools to indicate a
    /// horizontal or vertical flip, respectively.  Does not apply rotation.
    pub fn transform(&self, t: &Transform) -> (Rectangle, (bool, bool)) {
        let new_size = self.size * t.scale.abs();
        let new_position = self.position + t.position + ((self.size - new_size) / 2.0);

        (
            Rectangle::new(new_position, new_size).unwrap(),
            (
                t.scale.x < 0.0,
                t.scale.y < 0.0,
            ),
        )
    }

    pub fn to_sdl_rectangle(&self) -> SdlRectangle {
        let (size_x, size_y) = (
            self.size.x.round() as u32,
            self.size.y.round() as u32,
        );

        let (width, height) = (
            if size_x == 0 { 1 } else { size_x },
            if size_y == 0 { 1 } else { size_y },
        );

        SdlRectangle::new(
            self.position.x.round() as i32, self.position.y.round() as i32,
            width, height,
        ).unwrap().unwrap()
    }
}
