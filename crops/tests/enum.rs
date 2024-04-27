#[derive(crops::CBuilder, Debug, Clone, Default, PartialEq, Eq, Copy)]
enum Color {
    #[default]
    Red,
    Blue,
    Green,
}

pub fn main() {
    let c = color_default();

    color_as_blue(c);

    assert_eq!(unsafe { *c }, Color::Blue);
}
