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
            scale: Vector2::zero(),
            rotation: 0.0,
            position: Vector2::zero(),
        }
    }
}
