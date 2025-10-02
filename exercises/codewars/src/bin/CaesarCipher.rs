struct CaesarCipher {
    step: u32
}

impl CaesarCipher {
    fn new(integer: u32) -> Self {
        Self { step: integer }
    }

    fn encode(&self, input: &str) -> String {
        let mut new = String::new();
        for c in input.chars() {
            if c.is_alphabetic() {
                let encoded_digit = c.to_digit(36).unwrap() + self.step;
                dbg!(encoded_digit);
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

    fn decode(&self, input: &str) -> String {
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

fn main() {
    let c = CaesarCipher::new(18);
    println!("{}", c.decode("HNAPNXRF"));
    println!("{}", c.encode("HANCAKES"));

}