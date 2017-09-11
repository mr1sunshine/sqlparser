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
