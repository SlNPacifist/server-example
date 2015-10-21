use std::fmt::Debug;

pub trait ResultExpect<T> {
    fn expect_b(self, msg: &str) -> T;
}

impl<T, E> ResultExpect<T> for Result<T, E> where E: Debug {
    fn expect_b(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => panic!("{}: {:?}", msg, e),
        }
    }
}