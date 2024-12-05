pub mod day1;
pub mod day2;
pub mod day3;
pub mod day5;
// pub mod day6;

pub trait Solution {
    fn parse(input: &str) -> Self where Self: Sized;
    fn part1(&self) -> u64;
    fn part2(&self) -> u64;
}
