mod mockclient;
mod server;

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn testing_works() {
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn testing_panic() {
        panic!();
    }
}
