const INPUT: &'static str = "R2, L3, R2, R4, L2, L1, R2, R4, R1, L4, L5, R5, R5, R2, R2, R1, L2, L3, L2, L1, R3, L5, R187, R1, R4, L1, R5, L3, L4, R50, L4, R2, R70, L3, L2, R4, R3, R194, L3, L4, L4, L3, L4, R4, R5, L1, L5, L4, R1, L2, R4, L5, L3, R4, L5, L5, R5, R3, R5, L2, L4, R4, L1, R3, R1, L1, L2, R2, R2, L3, R3, R2, R5, R2, R5, L3, R2, L5, R1, R2, R2, L4, L5, L1, L4, R4, R3, R1, R2, L1, L2, R4, R5, L2, R3, L4, L5, L5, L4, R4, L2, R1, R1, L2, L3, L2, R2, L4, R3, R2, L1, L3, L2, L4, L4, R2, L3, L3, R2, L4, L3, R4, R3, L2, L1, L4, R4, R2, L4, L4, L5, L1, R2, L5, L2, L3, R2, L2";
// const INPUT: &'static str = "R2, L3";
// const INPUT: &'static str = "R2, R2, R2";
// const INPUT: &'static str = "R5, L5, R5, R3";

#[derive(Debug)]
enum Direction {
    L, R
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    distance: i32,
}

#[derive(Debug)]
enum CompassPoint { N, E, S, W }

#[derive(Debug)]
struct Position  {
    facing: CompassPoint,
    x: i32,
    y: i32,
}

pub fn main() {
    let mut steps: Vec<Step> = vec![];
    for step_str in INPUT.split(", ") {
        let step = parse_step(&step_str).expect("Failed to parse step_str");
        steps.push(step);
    }
    let mut position = Position { facing: CompassPoint::N, x: 0, y: 0 };
    for step in steps {
        apply_to_position(&mut position, &step);
    }
    println!("{:?}", position);
    println!("{}", position.x.abs() + position.y.abs());
}

fn apply_to_position(position: &mut Position, step: &Step) {
    let new_direction: CompassPoint = match (&position.facing, &step.direction) {
        (&CompassPoint::N, &Direction::L) => CompassPoint::W,
        (&CompassPoint::N, &Direction::R) => CompassPoint::E,
        (&CompassPoint::E, &Direction::L) => CompassPoint::N,
        (&CompassPoint::E, &Direction::R) => CompassPoint::S,
        (&CompassPoint::S, &Direction::L) => CompassPoint::E,
        (&CompassPoint::S, &Direction::R) => CompassPoint::W,
        (&CompassPoint::W, &Direction::L) => CompassPoint::S,
        (&CompassPoint::W, &Direction::R) => CompassPoint::N,
    };
    let direction_modifier = match new_direction {
        CompassPoint::W => (-1, 0),
        CompassPoint::S => (0, -1),
        CompassPoint::E => (1, 0),
        CompassPoint::N => (0, 1),
    };
    // println!("{:?}", position);
    position.x += direction_modifier.0 * step.distance;
    position.y += direction_modifier.1 * step.distance;
    position.facing = new_direction;
    // println!("{:?}", position);
}

fn parse_step(step_str: &str) -> Option<Step> {
    let (direction_str, distance_str) = step_str.split_at(1);
    let direction;
    match direction_str {
        "L" => direction = Direction::L,
        "R" => direction = Direction::R,
        _ => return None,
    };
    let distance;
    match distance_str.parse::<i32>().ok() {
        Some(v) => distance = v,
        None => return None
    }
    Some(Step { direction: direction, distance: distance })
}
