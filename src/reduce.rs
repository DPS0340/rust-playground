use std::{option::Option, vec::Vec};

pub fn get_min(it: &Vec<i32>) -> Option<&i32> {
    if it.len() == 0 {
        return None;
    }

    return it.into_iter().reduce(|a, b| if a > b { b } else { a });
}
