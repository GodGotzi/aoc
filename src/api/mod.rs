pub trait Solution {
    fn load_input(&self) -> String {
        let path = format!("input/day{:02}", self.get_day());
        println!("Loading input from: {}", path);
        std::fs::read_to_string(path).expect("Something went wrong reading the file")
    }

    fn get_day(&self) -> u8;
    fn part1(&self, input: String) -> String;
    fn part2(&self, input: String) -> String;
}
