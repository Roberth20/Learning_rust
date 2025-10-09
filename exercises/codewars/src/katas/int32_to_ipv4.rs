fn binary_to_int(b: &str) -> i32 {
    let mut arr = vec![];
    for (i, c) in b.chars().rev().enumerate() {
            if c == '1' {
                arr.push(2_i32.pow(i as u32));
            } else {
                arr.push(0);
            }
    }
    arr.iter().sum()
}


fn int32_to_ip(int: u32) -> String {
    let mut s = format!("{int:b}");
    if s.len() < 32 {
        s.insert_str(0, "0".repeat(32 - s.len()).as_str());
    }
    let res: Vec<i32> = (0..4).map(|idx| 
        binary_to_int(&s[(idx*8)..((idx+1)*8)])
    ).collect();
    format!("{}.{}.{}.{}", res[0], res[1], res[2], res[3])

}

// Easy way without crates
fn int32_to_ip_v2(int: u32) -> String {
    let bytes: [u8; 4] = int.to_be_bytes();
    format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
        assert_eq!(int32_to_ip(32), "0.0.0.32");
    }

    #[test]
    fn basicv2() {
        assert_eq!(int32_to_ip_v2(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip_v2(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip_v2(0), "0.0.0.0");
        assert_eq!(int32_to_ip_v2(32), "0.0.0.32");
    }
}
