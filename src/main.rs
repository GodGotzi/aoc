pub mod api;
pub mod days;

use api::Solution;

fn main() {
    let solutions: Vec<Box<dyn Solution>> = vec![Box::new(days::day07::Day07)];

    for solution in solutions {
        run_solution(&*solution);
    }
}

fn run_solution(solution: &dyn Solution) {
    let input = solution.load_input();
    let part1 = solution.part1(input.clone());
    let part2 = solution.part2(input.clone());

    println!("Day {}:", solution.get_day());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
