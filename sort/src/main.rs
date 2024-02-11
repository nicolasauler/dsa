#![allow(clippy::needless_return)]
use count::test_count_sorts;

mod comparison;
mod count;

fn main() {
    test_count_sorts();
}
