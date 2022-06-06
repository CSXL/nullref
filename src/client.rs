#[allow(clippy::module_inception)]
mod client {
    pub fn hello_world() -> i32 {
        println!("Hello world");
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let result: i32 = client::hello_world();
        assert_eq!(result, 0);
    }
}
