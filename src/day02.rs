#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Movement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub struct Position {
    pub horizontal: i32,
    pub aim: i32,
    pub depth: i32,
}

pub fn parse_movements(movements: &str) -> Vec<Movement> {
    movements
        .split('\n')
        .map(|line| {
            let trimmed_lowercase_line = line.trim().to_lowercase();
            let command_and_distance: Vec<&str> = trimmed_lowercase_line.split(' ').collect();
            let distance = command_and_distance[1].parse::<u32>().unwrap();
            match command_and_distance[0] {
                "forward" => Movement::Forward(distance),
                "down" => Movement::Down(distance),
                "up" => Movement::Up(distance),
                unknown_command => panic!("Unknown command: {}", unknown_command),
            }
        })
        .collect()
}

pub fn get_final_position(movements: &[Movement]) -> Position {
    let mut pos = Position { horizontal: 0, aim: 0, depth: 0 };
    for m in movements {
        match m {
            Movement::Forward(d) => {
                pos.horizontal += *d as i32;
                pos.depth += pos.aim * *d as i32;
            },
            Movement::Down(d) => pos.aim += *d as i32,
            Movement::Up(d) => pos.aim -= *d as i32,
        }
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let commands_str =
            "forward 5
             down 5
             forward 8
             up 3
             down 8
             forward 2";

        let commands = parse_movements(commands_str);
        assert_eq!(commands, Vec::from([Movement::Forward(5), Movement::Down(5), Movement::Forward(8), Movement::Up(3), Movement::Down(8), Movement::Forward(2)]));
    }

    #[test]
    fn part1() {
        let commands =
            parse_movements(
                "forward 5
                 down 5
                 forward 8
                 up 3
                 down 8
                 forward 2"
            );
        let final_position = get_final_position(&commands);
        assert_eq!(final_position.horizontal * final_position.aim, 150);
    }

    #[test]
    fn part2() {
        let commands =
            parse_movements(
                "forward 5
                 down 5
                 forward 8
                 up 3
                 down 8
                 forward 2"
            );
        let final_position = get_final_position(&commands);
        assert_eq!(final_position.horizontal * final_position.depth, 900);
    }
}