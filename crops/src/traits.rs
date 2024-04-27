pub trait AsMutPtr<T> {
    fn as_mut_ptr(&mut self) -> *mut T;
}

impl<T> AsMutPtr<T> for T {
    fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut T
    }
}

pub trait AsConstPtr<T> {
    fn as_const_ptr(&self) -> *const T;
}

impl<T> AsConstPtr<T> for T {
    fn as_const_ptr(&self) -> *const T {
        self as *const T
    }
}
