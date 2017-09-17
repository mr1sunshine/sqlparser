use signed_number::signed_number_type::SignedNumber;

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    SignedNumber(SignedNumber),
    StringLiteral(String),
    BlobLiteral(Box<[u8]>),
    Null,
    CurrentTime,
    CurrentDate,
    CurrentTimestamp
}