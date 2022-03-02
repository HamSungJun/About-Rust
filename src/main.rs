mod solutions;
use Vec;
fn main() {
    let result = solutions::solution_1::two_sum(Vec::from([1, 2, 3, 4, 5]), 5);
    println!("{:?}", result);
}
