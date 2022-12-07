mod half_one;
mod half_two;

use half_one::half_one;
use half_two::half_two;

fn main() {
    println!("{}", half_one());
    println!("{}", half_two());
}
