pub trait Solve<T> {
    fn solve(&self) -> T;
    fn solve_part_two(&self) -> T;
    fn load_input() -> Vec<String>;
}
