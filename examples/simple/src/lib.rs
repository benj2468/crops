#[derive(crops::CBuilder, Default, Clone, Debug)]
pub struct Brush {
    weight: u8,
    color: Color
}

#[derive(crops::CBuilder, Default, Clone, Debug)]
pub enum Color {
    #[default]
    Red,
    Blue,
    Green
}
