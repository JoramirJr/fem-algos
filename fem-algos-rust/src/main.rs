mod algos;
use std::{env, str::FromStr};

use crate::algos::binary_search::binary_search;
// use crate::algos::linear_search::linear_search;
fn main() {
    let args: Vec<String> = env::args().collect();

    let search: usize = FromStr::from_str(&args[2]).unwrap_or(0);

    let container: [usize; 20] = [
        77, 58, 95, 29, 98, 7, 40, 8, 66, 14, 11, 21, 18, 38, 20, 99, 25, 52, 87, 1,
    ];
    // println!("{:?}", linear_search(container, search));
    println!("{:?}", binary_search(container, search));
}
