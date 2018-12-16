use std::collections::HashMap;
use std::collections::HashSet;

type Coordinate = (usize, usize);

enum Track {
    // |
    Vertical,
    // -
    Horizontal,
    // +
    Intersection,

    // curves
    // invariant: Curves connect exactly two perpendicular pieces of track

    // top to left /
    TopToLeft,
    // bottom to left /
    BottomToLeft,
    // top to right \
    TopToRight,
    // bottom to right \
    BottomToRight,
}

fn is_horizontal(cell: char) -> bool {
    match cell {
        '-' | '+' => true,
        _ => false,
    }
}

fn is_vertical(cell: char) -> bool {
    match cell {
        '|' | '+' => true,
        _ => false,
    }
}

impl Track {
    fn has_horizontal(&self) -> bool {
        match self {
            Track::Horizontal => true,
            Track::Intersection => true,
            _ => false,
        }
    }

    fn has_vertical(&self) -> bool {
        match self {
            Track::Vertical => true,
            Track::Intersection => true,
            _ => false,
        }
    }
}

type Map = HashMap<Coordinate, Track>;

#[derive(Debug, PartialEq, Eq, Hash)]
enum TurningOption {
    Left,
    Straight,
    Right,
}

impl TurningOption {
    fn next(&self) -> TurningOption {
        match self {
            TurningOption::Left => TurningOption::Straight,
            TurningOption::Straight => TurningOption::Right,
            TurningOption::Right => TurningOption::Left,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Cart {
    current_position: Coordinate,
    // when a cart arrives at an intersection, this rule determines the cart's
    // next destination
    turning_option: TurningOption,
}

type Carts = HashSet<Cart>;

fn main() {
    let input_string = include_str!("input.txt");

    println!("{:?}", input_string);

    let carts: Carts = HashSet::new();

    let map: Map = {
        let mut map: Map = HashMap::new();

        let mut cell_map: HashMap<Coordinate, char> = HashMap::new();

        for (x, line) in input_string.lines().enumerate() {
            for (y, cell) in line.chars().enumerate() {
                let position: Coordinate = (x, y);
                cell_map.insert(position, cell);
            }
        }

        for (position, cell) in cell_map.iter() {
            let (x, y) = position.clone();
            let position = position.clone();

            match cell {
                '|' => {
                    map.insert(position, Track::Vertical);
                }
                '-' => {
                    map.insert(position, Track::Horizontal);
                }
                '+' => {
                    map.insert(position, Track::Intersection);
                }
                '/' => {
                    // match configuration:
                    //   /-
                    //   |
                    let valid_right_side = match cell_map.get(&(x + 1, y)) {
                        None => false,
                        Some(cell) => is_horizontal(*cell),
                    };

                    let valid_bottom_side = match cell_map.get(&(x, y + 1)) {
                        None => false,
                        Some(cell) => is_vertical(*cell),
                    };

                    if valid_right_side && valid_bottom_side {
                        map.insert(position, Track::TopToLeft);
                        continue;
                    }

                }
                '\\' => {
                    println!("found \\");
                }
                _ => {}
            }
        }

        map
    };
}
