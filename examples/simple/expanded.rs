#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub struct Brush {
    weight: u8,
    color: Color,
}
#[no_mangle]
pub extern "C" fn brush_default() -> *mut Brush {
    Box::into_raw(Box::default())
}
#[no_mangle]
pub extern "C" fn brush_clone(s: &Brush) -> *mut Brush {
    Box::into_raw(Box::new(s.clone()))
}
#[no_mangle]
pub extern "C" fn brush_debug(s: &Brush) {
    {
        ::std::io::_print(format_args!("{0:?}\n", s));
    };
}
/// Replaces the current value with the provided value
fn _inner_brush_with_weight(source: *mut Brush, value: u8) -> ::crops::utils::CResult {
    ::crops::utils::check_null(source)
        .map_err(|e| {
            let res = ::alloc::fmt::format(format_args!("{1} ({0})", "Brush", e));
            res
        })?
        .weight = value;
    Ok(())
}
/// Replaces the current value with the provided value
#[no_mangle]
pub extern "C" fn brush_with_weight(source: *mut Brush, value: u8) -> i32 {
    match _inner_brush_with_weight(source, value) {
        Ok(_) => 0,
        Err(e) => {
            {
                ::std::io::_eprint(format_args!("{0:?}\n", e));
            };
            1
        }
    }
}
/// Replaces the current value with the provided value
fn _inner_brush_get_weight(
    source: *const Brush,
    c_value: *mut u8,
) -> ::crops::utils::CResult {
    let value = &::crops::utils::check_null_const(source)
        .map_err(|e| {
            let res = ::alloc::fmt::format(format_args!("{1} ({0})", "Brush", e));
            res
        })?
        .weight;
    *::crops::utils::check_null(c_value)? = *value;
    Ok(())
}
/// Replaces the current value with the provided value
#[no_mangle]
pub extern "C" fn brush_get_weight(source: *const Brush, c_value: *mut u8) -> i32 {
    match _inner_brush_get_weight(source, c_value) {
        Ok(_) => 0,
        Err(e) => {
            {
                ::std::io::_eprint(format_args!("{0:?}\n", e));
            };
            1
        }
    }
}
/// Replaces the current value with the provided value
fn _inner_brush_with_color(
    source: *mut Brush,
    value: &Color,
) -> ::crops::utils::CResult {
    ::crops::utils::check_null(source)
        .map_err(|e| {
            let res = ::alloc::fmt::format(format_args!("{1} ({0})", "Brush", e));
            res
        })?
        .color = value.clone();
    Ok(())
}
/// Replaces the current value with the provided value
#[no_mangle]
pub extern "C" fn brush_with_color(source: *mut Brush, value: &Color) -> i32 {
    match _inner_brush_with_color(source, value) {
        Ok(_) => 0,
        Err(e) => {
            {
                ::std::io::_eprint(format_args!("{0:?}\n", e));
            };
            1
        }
    }
}
/// Replaces the current value with the provided value
fn _inner_brush_get_color(
    source: *const Brush,
    c_value: *mut Color,
) -> ::crops::utils::CResult {
    let value = &::crops::utils::check_null_const(source)
        .map_err(|e| {
            let res = ::alloc::fmt::format(format_args!("{1} ({0})", "Brush", e));
            res
        })?
        .color;
    *::crops::utils::check_null(c_value)? = value.clone();
    Ok(())
}
/// Replaces the current value with the provided value
#[no_mangle]
pub extern "C" fn brush_get_color(source: *const Brush, c_value: *mut Color) -> i32 {
    match _inner_brush_get_color(source, c_value) {
        Ok(_) => 0,
        Err(e) => {
            {
                ::std::io::_eprint(format_args!("{0:?}\n", e));
            };
            1
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for Brush {
    #[inline]
    fn default() -> Brush {
        Brush {
            weight: ::core::default::Default::default(),
            color: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Brush {
    #[inline]
    fn clone(&self) -> Brush {
        Brush {
            weight: ::core::clone::Clone::clone(&self.weight),
            color: ::core::clone::Clone::clone(&self.color),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Brush {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Brush",
            "weight",
            &self.weight,
            "color",
            &&self.color,
        )
    }
}
pub enum Color {
    #[default]
    Red,
    Blue,
    Green,
}
#[no_mangle]
pub extern "C" fn color_default() -> *mut Color {
    Box::into_raw(Box::default())
}
#[no_mangle]
pub extern "C" fn color_clone(s: &Color) -> *mut Color {
    Box::into_raw(Box::new(s.clone()))
}
#[no_mangle]
pub extern "C" fn color_debug(s: &Color) {
    {
        ::std::io::_print(format_args!("{0:?}\n", s));
    };
}
fn _inner_color_as_red(res: *mut Color) -> ::crops::utils::CResult {
    let res = ::crops::utils::check_null(res)
        .map_err(|e| {
            let res = ::alloc::fmt::format(format_args!("{1} ({0})", "Color", e));
            res
        })?;
    *res = Color::Red;
    Ok(())
}
#[no_mangle]
pub extern "C" fn color_as_red(res: *mut Color) -> i32 {
    match _inner_color_as_red(res) {
        Ok(_) => 0,
        Err(e) => {
            {
                ::std::io::_eprint(format_args!("{0:?}\n", e));
            };
            1
        }
    }
}
#[no_mangle]
pub extern "C" fn color_from_red() -> *mut Color {
    Box::into_raw(Box::new(Color::Red))
}
fn _inner_color_as_blue(res: *mut Color) -> ::crops::utils::CResult {
    let res = ::crops::utils::check_null(res)
        .map_err(|e| {
            let res = ::alloc::fmt::format(format_args!("{1} ({0})", "Color", e));
            res
        })?;
    *res = Color::Blue;
    Ok(())
}
#[no_mangle]
pub extern "C" fn color_as_blue(res: *mut Color) -> i32 {
    match _inner_color_as_blue(res) {
        Ok(_) => 0,
        Err(e) => {
            {
                ::std::io::_eprint(format_args!("{0:?}\n", e));
            };
            1
        }
    }
}
#[no_mangle]
pub extern "C" fn color_from_blue() -> *mut Color {
    Box::into_raw(Box::new(Color::Blue))
}
fn _inner_color_as_green(res: *mut Color) -> ::crops::utils::CResult {
    let res = ::crops::utils::check_null(res)
        .map_err(|e| {
            let res = ::alloc::fmt::format(format_args!("{1} ({0})", "Color", e));
            res
        })?;
    *res = Color::Green;
    Ok(())
}
#[no_mangle]
pub extern "C" fn color_as_green(res: *mut Color) -> i32 {
    match _inner_color_as_green(res) {
        Ok(_) => 0,
        Err(e) => {
            {
                ::std::io::_eprint(format_args!("{0:?}\n", e));
            };
            1
        }
    }
}
#[no_mangle]
pub extern "C" fn color_from_green() -> *mut Color {
    Box::into_raw(Box::new(Color::Green))
}
#[automatically_derived]
impl ::core::default::Default for Color {
    #[inline]
    fn default() -> Color {
        Self::Red
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Color {
    #[inline]
    fn clone(&self) -> Color {
        match self {
            Color::Red => Color::Red,
            Color::Blue => Color::Blue,
            Color::Green => Color::Green,
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Color {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Color::Red => "Red",
                Color::Blue => "Blue",
                Color::Green => "Green",
            },
        )
    }
}
