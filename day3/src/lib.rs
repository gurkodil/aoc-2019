use std::fmt;
use std::option::Option;

enum Heading {
    LEFT = 0,
    RIGHT = 1,
    UP = 2,
    DOWN = 3,
}


struct Cmd {
    heading: Heading,
    steps: f64
}

struct Position {
    x: f64,
    y: f64,
}

struct Line {
    start: Position,
    end: Position
}

impl Position {

    fn move_left(&mut self, steps: f64) {
        self.x -= steps;
    }

    fn move_right(&mut self, steps: f64) {
        self.x += steps;
    }

    fn move_up(&mut self, steps: f64) {
        self.y += steps;
    }

    fn move_down(&mut self, steps: f64) {
        self.y -= steps;
    }

    fn move_by_cmd(&mut self, cmd: &Cmd) {
        match cmd.heading {
            Heading::LEFT => self.move_left(cmd.steps),
            Heading::RIGHT => self.move_right(cmd.steps),
            Heading::UP => self.move_up(cmd.steps),
            Heading::DOWN => self.move_down(cmd.steps),
        };
    }

    fn clone(&mut self) -> Position {
        Position {
            x: self.x,
            y: self.y
        }
    }

}

fn traverse(cmds: Vec<Cmd>) -> Vec<Line> {
    let mut current_position = Position {
        x: 0.0,
        y: 0.0
    };
    let mut lines: Vec<Line> = Vec::new();

    for cmd in cmds {
       let l1 = current_position.clone();
       current_position.move_by_cmd(&cmd);
       let l2 = current_position.clone();
       let line = Line { start: l1, end: l2 }; 
       lines.push(line);
    }

    lines
}

fn split_command(cmd: String) -> Cmd {
    let mut heading = String::from(cmd);
    let steps: f64 = heading
        .split_off(1)
        .parse()
        .expect("Bad format on steps");
    let heading = Heading::parse(heading);
    Cmd {
        heading: heading,
        steps: steps
    }
}

// Got help frome here:
// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
fn line_intersects(l1: &Line, l2: &Line) -> Option<Position> {
    let x1 = l1.start.x;
    let y1 = l1.start.y;
    let x2 = l1.end.x;
    let y2 = l1.end.y;

    let x3 = l2.start.x;
    let y3 = l2.start.y;
    let x4 = l2.end.x;
    let y4 = l2.end.y;

    let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if den == 0.0 {
        return None
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

    if t > 0.0 && t < 1.0 && u > 0.0  && u < 1.0 {
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        return Some(Position { x: x, y: y })
    }

    None
}

fn format_input_line_to_commands(input: String) -> Vec<Cmd> {
    input.split(",")
        .map(|l| split_command(l.to_string()))
        .collect()
}


pub fn part_one(cmd1: String, cmd2: String) -> f64 {

    let cmd_line1 = format_input_line_to_commands(cmd1);
    let cmd_line2 = format_input_line_to_commands(cmd2);

    let mut wire_path1 = traverse(cmd_line1);
    let mut wire_path2 = traverse(cmd_line2);
    let mut record = std::f64::MAX;
    for wire1 in &mut wire_path1 {
        for wire2 in &mut wire_path2 {
            if let Some(pos) = line_intersects(&wire1, &wire2) {
                let length = pos.x.abs() + pos.y.abs(); // Manhattan distance
                if length < record {
                    record = length;
                }
            }
        }
    }
    record
}

impl Line {
    fn len(&mut self) -> f64 {
        let x1 = self.start.x;
        let x2 = self.end.x;
    
        let y1 = self.start.y;
        let y2 = self.end.y;
    
        ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt()
    }
}

fn difference(a: &Position, b: &Position) -> f64 {
    let end_pos = Position {
        x: a.x,
        y: a.y
     };

     let start_pos = Position {
         x: b.x,
         y: b.y,
    };
    let mut line = Line { start: start_pos, end: end_pos };
    return line.len()
}

pub fn part_two(cmd1: String, cmd2: String) -> f64 {

    let cmd_line1 = format_input_line_to_commands(cmd1);
    let cmd_line2 = format_input_line_to_commands(cmd2);

    let mut wire_path1 = traverse(cmd_line1);
    let mut wire_path2 = traverse(cmd_line2);

    let mut record = std::f64::MAX;
    let mut steps1 = 0.0;

    for wire1 in &mut wire_path1 {
        steps1 += wire1.len(); 
        let mut steps2 = 0.0;
        for wire2 in &mut wire_path2 {
            steps2 += wire2.len();
            if let Some(pos) = line_intersects(&wire1, &wire2) {
                let diff_wire1 = difference(&wire1.end, &pos);
                let diff_wire2 = difference(&wire2.end, &pos);
                
                let length = steps1 - diff_wire1 + steps2 - diff_wire2;
                if length < record {
                    record = length;
                }
            } 
        }
    }
    record
}


impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Heading::LEFT => return write!(f, "LEFT"),
            Heading::RIGHT => return write!(f, "RIGHT"),
            Heading::UP => return write!(f, "UP"),
            Heading::DOWN => return write!(f, "DOWN"),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Heading {
    fn parse(c: String) -> Heading {
        match c.as_ref() {
            "U" => return Heading::UP,
            "D" => return Heading::DOWN,
            "L" => return Heading::LEFT,
            "R" => return Heading::RIGHT,
            other => panic!("Unexpected heading: {}", other)
        }
    }
}

#[test]
fn test_part_one() {
    let l1 = "R8,U5,L5,D3".to_string();
    let l2 = "U7,R6,D4,L4".to_string();
    
    assert!(part_one(l1, l2) == 6.0);
}

#[test]
fn test_part_two() {
    let l1 = "R8,U5,L5,D3".to_string();
    let l2 = "U7,R6,D4,L4".to_string();

    assert!(part_two(l1, l2) == 30.0);
}

