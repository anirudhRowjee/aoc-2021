
fn main() {

    let numbers = aoc::read_one_per_line::<u32>("src/inputs/day1_a.in").unwrap();
    let numbers2 = aoc::read_one_per_line::<u32>("src/inputs/day1_b.in").unwrap();

    // Thanks, TJ!
    let count_1 = numbers
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();

    println!("Output 1 : {:?}", count_1);

    // part two - three-window sum slide
    let count_2 = numbers2
        .windows(3)
        .map( |window| window.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();

    println!("Output 2 : {:?}", count_2);
}
