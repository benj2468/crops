pub mod utils;
pub use crops_derive::*;

pub mod traits;

#[cfg(test)]
mod tests;

#[doc(hidden)]
pub mod _macros {
    pub use libc;
}


#[macro_export]
macro_rules! c_free {
    ($var:ident) => {
        if !$var.is_null() {
            unsafe { drop(Box::from_raw($var)) };
        }
    };
}