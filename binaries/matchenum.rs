enum Direction {
    Up,
    Down,
    Left,
    Right
}



fn main (){

    let number = 80;
    let player_direction : Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("we are up!"),
        Direction::Down => println!("Down!"),
        _ => println!("no")
    }

    match number {
        12 => println!("12 !"),
        //Match inclusive range
        13..=19 => println!("13 - 19"),
        // Match several values
        80 | 82 | 90 | 5 => println!("nono"),
        _ => println!("special!")
    }

}