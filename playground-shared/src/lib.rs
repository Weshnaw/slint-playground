pub fn increase_value(current: i32) -> i32 {
    current + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = increase_value(2);
        assert_eq!(result, 3);
    }
}
