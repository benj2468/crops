#[derive(crops::CBuilder, Default, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Brush {
    weight: u8,
    color: Color,
    /// An identifier for the brush
    name: String
}

#[derive(crops::CBuilder, Default, Clone, Debug)]
pub enum Color {
    #[default]
    Red,
    Blue,
    Green
}
