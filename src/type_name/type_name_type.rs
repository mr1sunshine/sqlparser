use signed_number::signed_number_type::SignedNumber;

#[derive(Debug, PartialEq)]
pub enum TypeName {
    OnlyName {
        name: Vec<String>
    },
    NameWithNumber {
        name: Vec<String>,
        number: SignedNumber
    },  
    NameWithTwoNumbers {
        name: Vec<String>,
        number1: SignedNumber,
        number2: SignedNumber
    }
}