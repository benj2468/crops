use crops::traits::AsMutPtr;

#[derive(crops::CBuilder, Debug, Clone, Default, PartialEq, Eq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn main() {
    let c = color_default();

    color_with_red(c, 0x34);
    color_with_green(c, 0x12);
    color_with_blue(c, 0x56);

    let mut out: u8 = 0;
    color_get_blue(c, out.as_mut_ptr());

    assert_eq!(out, 0x56);

    color_get_red(c, out.as_mut_ptr());
    assert_eq!(out, 0x34);

    color_get_green(c, out.as_mut_ptr());
    assert_eq!(out, 0x12);
}
