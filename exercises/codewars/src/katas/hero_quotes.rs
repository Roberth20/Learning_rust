struct BatmanQuotes;

impl BatmanQuotes {
    fn get_quote(quotes: &Vec<&str>, hero: &str) -> String {
        let HERO = ["Batman0", "Robin1", "Joker2"];
        for h in HERO {
            let index = h.chars().last().unwrap();
            if hero.starts_with(&h[..1]) && (hero.contains(index)) {
                return format!("{}: {}", &h[..h.len() - 1], quotes[index.to_digit(10).unwrap() as usize]);
            }
        }
       String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn test() {
        let quotes = vec!["WHERE IS SHE?!", "Holy haberdashery, Batman!", "Let's put a smile on that faaaceee!"];
        assert_eq!(BatmanQuotes::get_quote(&quotes, "Rob1n"), "Robin: Holy haberdashery, Batman!");
        assert_eq!(BatmanQuotes::get_quote(&quotes, "Batm0n"), "Batman: WHERE IS SHE?!");
        assert_eq!(BatmanQuotes::get_quote(&quotes, "Jok2r"), "Joker: Let's put a smile on that faaaceee!");
        
    }
}
