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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_subtract() {
        let result = subtract(10, 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_multiply() {
        let result = multiply(4, 5);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_divide() {
        let result = divide(10, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = std::panic::catch_unwind(|| divide(10, 0));
        assert!(result.is_err());
    }
}
