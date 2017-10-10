use numeric_literal::numeric_literal_type::NumericLiteral;

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    NumericLiteral(NumericLiteral),
    StringLiteral(String),
    BlobLiteral(Box<[u8]>),
    Null,
    CurrentTime,
    CurrentDate,
    CurrentTimestamp
}