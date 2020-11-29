use std::ops::Add;
use std::ops::Mul;

use image::{Pixel, Rgba};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            ((self.red) * 255.) as u8,
            ((self.green) * 255.) as u8,
            ((self.blue) * 255.) as u8,
            0,
        )
    }

    pub fn from_rgba(rgba: &Rgba<u8>) -> Self {
        Color {
            red: (rgba.channels()[0] as f32) / 255.,
            blue: (rgba.channels()[1] as f32) / 255.,
            green: (rgba.channels()[2] as f32) / 255.,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Self::Output {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Self::Output {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

pub const BLACK: Color = Color {
    red: 0.,
    green: 0.,
    blue: 0.,
};
