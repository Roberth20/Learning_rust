pub fn xo(s: &str) -> bool {
    let mut arr = [0, 0]; // x and o
    for c in s.to_lowercase().chars() {
        if c == 'x' {
            arr[0] += 1;
        } else if c == 'o' {
            arr[1] += 1;
        } 
    }
    arr[0] == arr[1]
}

fn xo_v2(s: &str) -> bool {
    s.to_lowercase().matches("x").count() == s.to_lowercase().matches("o").count()
}



#[cfg(test)]
mod tests {
    use super::xo;
    
    fn do_test(s: &str, expected: bool) {
        let actual = xo(s);
        assert!(expected == actual, "Test failed.\n\nInput:    {s:?}\nExpected: {expected}\nActual:   {actual}\n")
    }
    
    #[test]
    fn sample_tests() {
        do_test("xo", true);
        do_test("Xo", true);
        do_test("xxOo", true);
        do_test("xxxm", false);
        do_test("Oo", false);
        do_test("ooom", false);
    }
}
