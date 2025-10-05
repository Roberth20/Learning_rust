pub fn score(dice: [u8; 5]) -> u32 {
    let mut score: u32 = 0;
    let mut mdice = dice;
    mdice.sort();
    let mut repeated = 0;
    let mut curretn = 0;
    for item in mdice {
        if item != curretn {
            curretn = item;
            repeated = 1;
        }
        dbg!(curretn, repeated);
        if (item == 1) && (repeated == 3) {
            score += 800;
            repeated = 1;
            continue; 
        }
        if item == 1 {
            score += 100;
            repeated += 1;
            continue;
        }

        if (item == 5) && (repeated == 3) {
            score += 400;
            repeated = 1;
            continue; 
        }
        if item == 5 {
            score += 50;
            repeated += 1;
            continue;
        }
        if repeated == 3 {
            score += item as u32 * 100;
        }
        repeated += 1;
    }
    
    score
}

#[cfg(test)]
mod tests {
    use super::score;
    
    fn do_test(dice: &[u8; 5], expected: u32) {
        let actual = score(*dice);
        assert!(actual == expected, "Expected score with dice {dice:?} to be {expected}, but was {actual}\n");   
    }

    #[test]
    fn test_worthless() {
        do_test(&[2, 3, 4, 6, 2], 0);
    }

    #[test]
    fn test_triplet() {
        do_test(&[4, 4, 4, 3, 3], 400);
    }

    #[test]
    fn test_mixed_set() {
        do_test(&[2, 4, 4, 5, 4], 450);
    }
}