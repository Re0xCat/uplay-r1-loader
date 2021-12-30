#[inline]
pub fn alloc<T>(x: T) -> *mut T {
    Box::into_raw(Box::new(x))
}
