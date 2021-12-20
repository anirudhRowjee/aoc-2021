
#[derive(Debug, Clone)]
enum Move {
    Forward(i32),
    Down(i32),
    Up(i32)
}

#[derive(Debug)]
struct Position {
    horizontal : i32,
    vertical : i32,
}


#[derive(Debug)]
struct PositionAim {
    horizontal : i32,
    vertical : i32,
    aim: i32,
}

impl Position {

    // constructor
    pub fn new() -> Self {
        Self{ horizontal: 0, vertical: 0}
    }

    // get the output format
    pub fn product(self) -> i32 {
        return self.horizontal * self.vertical;
    }

    // modify the position
    pub fn modify(&mut self, current_move: Move) -> i32 {
        // apply the move
        match current_move {
            Move::Forward(x) => {
                self.horizontal += x
            },
            Move::Up(x) => {
                self.vertical -= x
            },
            Move::Down(x) => {
                self.vertical += x
            }
        }
        return 0;
    }

    // apply the modifications one by one
    pub fn consume(&mut self, moves: Vec<Move>) {
        moves
            .into_iter()
            .map(|current_move| self.modify(current_move))
            .last(); // just consume the iterator TODO see if there's a better way to do this
    }
}

impl PositionAim {
    // constructor
    pub fn new() -> Self {
        Self{ horizontal: 0, vertical: 0, aim: 0}
    }

    // get the output format
    pub fn product(self) -> i32 {
        return self.horizontal * self.vertical;
    }

    // modify the position
    pub fn modify(&mut self, current_move: Move) -> i32 {
        // apply the move
        match current_move {
            Move::Forward(x) => {
                self.horizontal += x;
                self.vertical += self.aim * x;
            },
            Move::Up(x) => {
                self.aim -= x
            },
            Move::Down(x) => {
                self.aim += x
            }
        }
        return 0;
    }

    // apply the modifications one by one
    pub fn consume(&mut self, moves: Vec<Move>) {
        moves
            .into_iter()
            .map(|current_move| self.modify(current_move))
            .last(); // just consume the iterator TODO see if there's a better way to do this
    }
}

fn parse_line_into_move(input_line: &str) -> Move {

    let input_line = input_line.split(" ").collect::<Vec<&str>>();
    // println!("{:?}", input_line);

    let direction = input_line[0];
    let quantity = input_line[1].parse::<i32>().unwrap();

    match direction {
        "forward" => Move::Forward(quantity),
        "down" => Move::Down(quantity),
        "up" => Move::Up(quantity),
        _ => panic!("Input not formatted properly")
    }
}

fn parse_lines_into_moves(lines: Vec<&str>) -> Vec<Move> {

    let moves: Vec<Move> = lines
        .into_iter()
        .map(|line| parse_line_into_move(&line))
        .collect::<Vec<Move>>();

    moves
}

fn main() {

    let lines = include_str!("./inputs/day2_a.in")
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>();

    let mut position = Position::new();

    let mut position_with_aim = PositionAim::new();

    // get the moves
    let moves: Vec<Move> = parse_lines_into_moves(lines);
    let moves_aim = moves.clone();

    position.consume(moves);
    position_with_aim.consume(moves_aim);

    println!("Result 1: {:?}", position.product());
    println!("Result 2: {:?}", position_with_aim.product());
}
