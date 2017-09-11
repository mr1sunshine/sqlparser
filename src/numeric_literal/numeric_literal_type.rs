use std::fmt::{self, Debug, Display};
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq)]
pub enum NumericLiteral {
    Integer(i32),
    Float(f64)
}

impl fmt::Display for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NumericLiteral::Integer(i) => write!(f, "{}", i),
            NumericLiteral::Float(i) => write!(f, "{}", i),
        }
    }
}

impl Add for NumericLiteral {
    type Output = NumericLiteral;

    fn add(self, other: NumericLiteral) -> NumericLiteral {
        match self {
            NumericLiteral::Integer(i) => {
                match other {
                    NumericLiteral::Integer(j) => NumericLiteral::Integer(i + j),
                    NumericLiteral::Float(j) => NumericLiteral::Float(i as f64 + j),
                }
            },
            NumericLiteral::Float(i) => {
                match other {
                    NumericLiteral::Integer(j) => NumericLiteral::Float(i + j as f64),
                    NumericLiteral::Float(j) => NumericLiteral::Float(i + j),
                }
            }
        }
    }
}

impl Mul for NumericLiteral {
    type Output = NumericLiteral;

    fn mul(self, other: NumericLiteral) -> NumericLiteral {
        match self {
            NumericLiteral::Integer(i) => {
                match other {
                    NumericLiteral::Integer(j) => NumericLiteral::Integer(i * j),
                    NumericLiteral::Float(j) => NumericLiteral::Float(i as f64 * j),
                }
            },
            NumericLiteral::Float(i) => {
                match other {
                    NumericLiteral::Integer(j) => NumericLiteral::Float(i * j as f64),
                    NumericLiteral::Float(j) => NumericLiteral::Float(i * j),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_literal_format_integer() {
        let test = NumericLiteral::Integer(1);
        let test_str = format!("{}", test);
        assert_eq!(test_str, "1");
    }

    #[test]
    fn test_numeric_literal_format_float() {
        let test = NumericLiteral::Float(0.5);
        let test_str = format!("{}", test);
        assert_eq!(test_str, "0.5");
    }

    #[test]
    fn test_numeric_literal_add_integer_integer() {
        assert_eq!(NumericLiteral::Integer(2), NumericLiteral::Integer(1) + NumericLiteral::Integer(1));
    }

    #[test]
    fn test_numeric_literal_add_integer_float() {
        assert_eq!(NumericLiteral::Float(1.5), NumericLiteral::Integer(1) + NumericLiteral::Float(0.5));
    }

    #[test]
    fn test_numeric_literal_add_float_integer() {
        assert_eq!(NumericLiteral::Float(1.5), NumericLiteral::Float(0.5) + NumericLiteral::Integer(1));
    }

    #[test]
    fn test_numeric_literal_add_float_float() {
        assert_eq!(NumericLiteral::Float(3.5), NumericLiteral::Float(1.75) + NumericLiteral::Float(1.75));
    }

    #[test]
    fn test_numeric_literal_mul_integer_integer() {
        assert_eq!(NumericLiteral::Integer(2), NumericLiteral::Integer(1) * NumericLiteral::Integer(2));
    }

    #[test]
    fn test_numeric_literal_mul_integer_float() {
        assert_eq!(NumericLiteral::Float(0.5), NumericLiteral::Integer(1) * NumericLiteral::Float(0.5));
    }

    #[test]
    fn test_numeric_literal_mul_float_integer() {
        assert_eq!(NumericLiteral::Float(0.5), NumericLiteral::Float(0.5) * NumericLiteral::Integer(1));
    }

    #[test]
    fn test_numeric_literal_mul_float_float() {
        assert_eq!(NumericLiteral::Float(3.0625), NumericLiteral::Float(1.75) * NumericLiteral::Float(1.75));
    }


}