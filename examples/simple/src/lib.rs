#[derive(crops::CBuilder, Default, Clone, Debug)]
pub struct Brush {
    weight: u8,
    color: Color,
    name: String
}

#[derive(crops::CBuilder, Default, Clone, Debug)]
pub enum Color {
    #[default]
    Red,
    Blue,
    Green
}
