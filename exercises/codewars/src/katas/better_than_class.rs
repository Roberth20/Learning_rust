use std::u16;

pub fn better_than_average(class_points: &[u16], your_points: u16) -> bool {
    let s: u16 = class_points.iter().sum();
    let l: u16 = class_points.len().try_into().unwrap();
    your_points > s / l
}

// Improved
fn better_than_average_v2(class_points: &[u16], your_points: u16) -> bool {
    your_points > class_points.iter().sum::<u16>() / class_points.len() as u16
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let points: [u16; 3] = [4, 5, 6];
        assert!(better_than_average(&points, 8))
    }
}