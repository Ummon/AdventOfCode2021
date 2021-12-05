use std::iter::Iterator;

type Diagnostic = Vec<Vec<char>>;

pub fn parse(diagnostic: &str) -> Diagnostic {
    diagnostic
        .split('\n')
        .map(|line| { line.trim().chars().collect() })
        .collect()
}

pub fn gamma_and_epsilon_rates(diagnostic: &Diagnostic) -> (u16, u16) {
    let l = diagnostic[0].len();
    dbg!(l);
    let mut gamma = 0u16;
    for i in 0..l {
        if diagnostic.iter().fold(0, |n, current| { n + if current[i] == '1' { 1 } else { 0 } }) > diagnostic.len() / 2 {
            gamma |= 1;
        }

        if i < l - 1 {
            gamma <<= 1;
        }
    }

    (gamma, !gamma & 0xFFFFu16 >> (16 - l))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let diagnostic =
            parse(
                "00100
                11110
                10110
                10111
                10101
                01111
                00111
                11100
                10000
                11001
                00010
                01010"
            );
        let (gamma, epsilon) = gamma_and_epsilon_rates(&diagnostic);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }
}