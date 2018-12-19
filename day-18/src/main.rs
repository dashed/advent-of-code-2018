// https://adventofcode.com/2018/day/18

// imports

use std::collections::HashMap;

// code

type Coordinate = (i32, i32);

trait Transitions {
    fn up(&self) -> Coordinate;
    fn down(&self) -> Coordinate;
    fn left(&self) -> Coordinate;
    fn right(&self) -> Coordinate;
}

impl Transitions for Coordinate {
    fn up(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y - 1);
    }

    fn down(&self) -> Coordinate {
        let (x, y) = self;
        return (*x, y + 1);
    }

    fn left(&self) -> Coordinate {
        let (x, y) = self;
        return (x - 1, *y);
    }

    fn right(&self) -> Coordinate {
        let (x, y) = self;
        return (x + 1, *y);
    }
}

#[derive(Debug, Clone)]
enum Acre {
    Ground,
    Tree,
    Lumberyard,
}

impl Acre {
    fn next(&self, adjacent_acres: Vec<Acre>) -> Self {
        return Acre::Ground;
    }
}

struct Area {
    area: HashMap<Coordinate, Acre>,
    max_y: i32,
    max_x: i32,
}

impl Area {
    fn new() -> Self {
        Area {
            area: HashMap::new(),
            max_y: 0,
            max_x: 0,
        }
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut map_string: Vec<String> = vec![];

        for y in 0..=self.max_y {
            let mut row_string = String::from("");

            for x in 0..=self.max_x {
                let position = (x, y);

                match self.area.get(&position) {
                    None => {
                        unreachable!();
                    }
                    Some(acre) => match acre {
                        Acre::Ground => {
                            row_string.push_str(".");
                        }
                        Acre::Tree => {
                            row_string.push_str("|");
                        }
                        Acre::Lumberyard => {
                            row_string.push_str("#");
                        }
                    },
                }
            }

            map_string.push(row_string);
        }

        return map_string.join("\n");
    }

    fn insert(&mut self, position: Coordinate, acre: char) {
        let (x, y) = position;

        if x > self.max_x {
            self.max_x = x;
        }

        if y > self.max_y {
            self.max_y = y;
        }

        match acre {
            '.' => {
                self.area.insert(position, Acre::Ground);
            }
            '|' => {
                self.area.insert(position, Acre::Tree);
            }
            '#' => {
                self.area.insert(position, Acre::Lumberyard);
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn get_adjacent(&self, position: &Coordinate) -> Vec<Acre> {
        let adjacent: Vec<Coordinate> = vec![
            // clockwise
            position.up(),
            position.up().right(),
            position.right(),
            position.down().right(),
            position.down(),
            position.down().left(),
            position.left(),
            position.up().left(),
        ];

        let result: Vec<Acre> = adjacent
            .into_iter()
            .map(|coord| self.area.get(&coord))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap().clone())
            .collect();

        return result;
    }

    fn tick(&mut self) {}
}

fn generate_area(input_string: &str) -> Area {
    let mut area = Area::new();

    for (y, line) in input_string.trim().lines().enumerate() {
        let line = line.trim();

        for (x, acre) in line.chars().enumerate() {
            let position: Coordinate = (x as i32, y as i32);
            area.insert(position, acre);
        }
    }

    return area;
}

fn main() {
    let input_string = include_str!("input.txt");

    let area = generate_area(input_string);
    println!("{}", area.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_string() {
        let expected_string = include_str!("input.txt");

        let area = generate_area(expected_string);

        assert_eq!(area.to_string(), expected_string);
    }

}
