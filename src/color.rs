use iced::Color;

pub fn rgb_to_color(r: i32, g: i32, b: i32, a: f32) -> Color {
    Color {
        r: r as f32 / 256f32,
        g: g as f32 / 256f32,
        b: b as f32 / 256f32,
        a,
    }
}
