use std::ops::{Deref, DerefMut};

pub struct DerefMutExample<T> {
    value: T
}

impl<T> Deref for DerefMutExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("deref invoked");
        &self.value
    }
}

impl<T> DerefMut for DerefMutExample<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("deref_mut invoked");
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn middle_one() {
        let mut x = DerefMutExample { value: 'a' };
        *x = 'b';
        assert_eq!('b', *x);
    }
}


