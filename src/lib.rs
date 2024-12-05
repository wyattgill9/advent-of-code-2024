pub mod day1;
pub mod day2;
pub mod day3;

// Common traits/types if needed in the future
pub trait Solution {
    fn parse(input: &str) -> Self where Self: Sized;
    fn part1(&self) -> u64;
    fn part2(&self) -> u64;
}
