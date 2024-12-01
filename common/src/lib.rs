use std::ops::Deref;

pub trait StrExt {
    fn to_i32(&self) -> i32;
    fn to_u32(&self) -> u32;
}

impl<T> StrExt for T
where
    T: Deref<Target = str>,
{
    fn to_i32(&self) -> i32 {
        self.parse::<i32>().expect("valid i32")
    }

    fn to_u32(&self) -> u32 {
        self.parse::<u32>().expect("valid u32")
    }
}
