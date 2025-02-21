pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtract(left: u64, right: u64) -> u64 {
    left - right
}

pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

pub fn divide(left: u64, right: u64) -> u64 {
    left / right
}

pub fn square(num: u64) -> u64 {
    num * num
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

        let result3 = divide(72, 8);
        assert_eq!(result3, 9);

        let result4 = subtract(14, 3);
        assert_eq!(result4, 11);

        let result5 = square(7);
        assert_eq!(result5, 49);
    }
}
