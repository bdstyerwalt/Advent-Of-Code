
fn main() {
    let input = include_str!("input.txt");
    let p1 = part1(input, 64);
    let p2 = part2(input, 26501365);
    println!("Part 1: {} | Part 2: {}", p1, p2);
}
