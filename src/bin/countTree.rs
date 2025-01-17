fn get_input()->&'static str {
    return "..##.......
	#...#...#..
	.#....#..#.
	..#.#...#.#
	.#...##..#.
	..#.##.....
	.#.#.#....#
	.#........#
	#.##...#...
	#...##....#
	.#..#...#.
    "
}
fn main () {
    println!("{}", get_input()
    .lines() // to set your iterator
    .enumerate()
    .flat_map(|(idx, line)| {
        return line 
        .chars()
        .nth(idx * 3 % line.len());
    })
    .filter(|&x| x == '#')
    .count());

}