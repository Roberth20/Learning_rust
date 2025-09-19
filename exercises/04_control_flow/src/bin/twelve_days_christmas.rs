fn main() {
    // Start defining the values of chore
    let ordinals = ["first", "second", "thrid", "fourth", "fifth", "sixth",
    "seventh", "eighth", "nineth", "tenth", "eleventh", "twelveth"];
    let verses = ["A partridge in a pear tree", 
    "Two turtle doves", "Three French hens", "four calling birds", 
    "five gold rings", "six geese a-laying", "seven swans a-swimming",
    "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping",
    "eleven pipers piping", "twelve drummers drumming"];
    
    // Iterate over the ordinal days
    for (index, ord) in ordinals.iter().enumerate() {
        println!("On the {ord} day of Christmas my true love sent to me");
        // Reverse the verses
        for v in (0..index+1).rev() {
            if v == 0 {
                println!("And {}", verses[v]);
            } else {
                println!("{},", verses[v]);
            }
        }
        println!("")
    }
}