fn get_input() ->&'static str {
	return "orward 5
down 5
forward 8
up 3
down 8
forward 2";
}

#[derive(Debug)] // need this to print it!!
struct Point {
    x: i32,
    y: i32,
}

// take a parse line 
fn parse_line( line: &str) -> Point {
    let (dir, amount) = line.split_once(" ").expect("Must contain a white space");
    let amount: i32 = str::parse::<i32>(amount).expect("Second arg must be an int");

   if dir == "forward" {
    return Point {x: amount, y: 0};
   } else if dir == "up" {
    return Point {x: 0, y: -amount};
   }
   return Point {x: 0, y: amount};

}
// split it 
// return it

fn main() {
let result = get_input()
    .lines()
    .map(parse_line)
    .fold(Point{x: 0, y: 0}, |mut acc, point | {
        acc.x += point.x;
        acc.y += point.y;
        return acc;
    });
    
    println!("{:?}", result);
}