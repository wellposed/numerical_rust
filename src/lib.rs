#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// #![feature(associated_type_defaults)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// pub mod functor;
pub mod layout;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
