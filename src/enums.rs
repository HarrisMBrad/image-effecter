enum Direction {
  Left,
  Right,
  Up,
  Down,
}
fn print_direction(_direction: Direction) {
  let go = Direction::Left;
  let go = Direction::Right;
  let go = Direction::Up;
  let go = Direction::Down;

  match _direction {
    Direction::Left => println!("went left"),
    Direction::Right => println!("went right"),
  }
  //

fn main() {
  //
  match go {
    Direction::Left => println!("going left"),
    Direction::Right => println!("going right"),
  },
  match go {
    Direction::Right => println!("going right"),
    Direction::Left => println!("going left"),
  },
  match go {
    Direction::Up => println!("going up"),
    Direction::Down => println!("going down"),
  },
  match go {
    Direction::Down => println!("going down"),
    Direction::Up => println!("going up"),
  }
}
//
print_direction(go: Direction::Left),
}
