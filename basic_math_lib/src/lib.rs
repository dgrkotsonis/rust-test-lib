pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

pub fn divide(left: u64, right: u64) -> u64 {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        let result2 = multiply(2, 6);
        assert_eq!(result2, 12);
    }
}
