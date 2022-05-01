pub fn print<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}

pub trait Print {
    fn print(&self)
    where
        Self: std::fmt::Display,
    {
        print(self);
    }
}
