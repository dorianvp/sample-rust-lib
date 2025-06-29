use corelib::MathError;

pub fn add(lhs: u64, rhs: u64) -> u64 {
    lhs + rhs
}

pub fn subtract(lhs: u64, rhs: u64) -> u64 {
    lhs - rhs
}

pub fn multiply(lhs: u64, rhs: u64) -> u64 {
    lhs * rhs
}

pub fn divide(lhs: u64, rhs: u64) -> Result<u64, MathError> {
    if rhs == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(lhs / rhs)
    }
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
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err(MathError::DivisionByZero));
    }
}
