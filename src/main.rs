pub mod api;
pub mod days;

use api::Solution;

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![Box::new(days::day09::Day09)];

    for solution in solutions {
        run_solution(&*solution);
    }
}

fn run_solution(solution: &dyn Solution) {
    let now = std::time::Instant::now();

    println!("Day {}:", solution.get_day());
    let input = solution.load_input();
    let part1 = solution.part1(input.clone());
    let time_part1 = now.elapsed();

    println!("Part 1: {}", part1);
    println!("Time for part 1: {:?}", time_part1);

    let part2 = solution.part2(input.clone());
    let time_part2 = now.elapsed() - time_part1;

    println!("Part 2: {}", part2);
    println!("Time for part 2: {:?}", time_part2);
}
