use std::iter::Iterator;

pub fn count_number_of_decreased_values(report: &[i32], window_size: usize) -> i32 {
    let mut n = 0;
    for i in 0..report.len() - window_size {
        if report[i + 1..i + window_size + 1].iter().sum::<i32>() > report[i..i + window_size].iter().sum::<i32>() {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let sea_floor_report = [ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 ];
        assert_eq!(count_number_of_decreased_values(&sea_floor_report, 1), 7);
    }

    #[test]
    fn part2() {
        let sea_floor_report = [ 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 ];
        assert_eq!(count_number_of_decreased_values(&sea_floor_report, 3), 5);
    }
}