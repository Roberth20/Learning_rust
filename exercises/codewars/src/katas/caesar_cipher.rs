pub struct CaesarCipher {
    pub step: u32
}

impl CaesarCipher {
    pub fn new(integer: u32) -> Self {
        Self { step: integer }
    }

    pub fn encode(&self, input: &str) -> String {
        let mut new = String::new();
        for c in input.chars() {
            if c.is_alphabetic() {
                let encoded_digit = c.to_digit(36).unwrap() + self.step;
                if  encoded_digit <= 35 {
                    new.push(char::from_digit(encoded_digit, 36).unwrap());
                } else {
                    new.push(char::from_digit(encoded_digit - 26, 36).unwrap());
                }
            } else {
                new.push(c);
            }
        }
        new.to_uppercase()

    }

    pub fn decode(&self, input: &str) -> String {
        let mut new = String::new();
        for c in input.chars() {
            if c.is_alphabetic() {
                let encoded_digit = if c.to_digit(36).unwrap() >= self.step{
                    c.to_digit(36).unwrap() - self.step
                } else {
                    26 + c.to_digit(36).unwrap() - self.step
                };
                if  encoded_digit > 9 {
                    new.push(char::from_digit(encoded_digit, 36).unwrap());
                } else {
                    new.push(char::from_digit(encoded_digit+26, 36).unwrap());
                }
            } else {
                new.push(c);
            }
        }
        new.to_uppercase()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let c = CaesarCipher::new(18);
        assert_eq!("PANCAKES", c.decode("hsfuscwk"));
        assert_eq!("HSFUSCWK", c.encode("PANCAKES"));
    }
}