struct Rect {
    width : i32,
    height : i32
}

impl Rect {
    fn area(&self) -> i32 {
        return self.height * self.width;
    }

    fn perimeter(&self) -> i32 {
        return 2*(self.height + self.width);
    }

    fn debugging() -> i32 {
        return 1;
    }
}

enum  Direction {
    North,
    South,
    East,
    West
}

enum Shape {
    Rect(f64,f64),
    Circle(f64)
}

fn calculate_area(shape : Shape) -> f64 {
    let area = match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rect(a,b ) => a*b
    };
    return area
}

fn main() {
    let r = Rect {
        width : 10,
        height : 5
    };
    println!("{} \n {} \n {}",r.area(),r.perimeter(),Rect::debugging());

    let direction = Direction::North;
    match direction {
        Direction::East => {println!("the direction is east")},
        Direction::West => {println!("the direction is west")},
        Direction::North => {println!("the direction is north")},
        Direction::South => {println!("the direction is south")}
    };


}