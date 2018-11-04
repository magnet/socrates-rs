pub mod component;
pub mod module;
pub mod service;

pub type Result<T> = std::io::Result<T>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
