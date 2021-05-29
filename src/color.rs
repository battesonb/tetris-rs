use macroquad::prelude::Color;

pub trait ColorExtension {
    fn add(&self, value: f32) -> Color;

    fn multiply(&self, value: f32) -> Color;
}

impl ColorExtension for Color {
    fn multiply(&self, value: f32) -> Color {
        Color::new(self.r * value, self.g * value, self.b * value, self.a)
    }

    fn add(&self, value: f32) -> Color {
        Color::new(self.r + value, self.g + value, self.b + value, self.a)
    }
}
