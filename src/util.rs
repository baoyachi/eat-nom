use std::ops::Range;

pub fn new_range<T>(start: T, end: T) -> Range<T> {
    Range { start, end }
}