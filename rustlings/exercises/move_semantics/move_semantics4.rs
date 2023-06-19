// move_semantics4.rs
// Refactor this code so that instead of passing `vec0` into the `fill_vec` function,
// the Vector gets created in the function itself and passed back to the main
// function.
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand for a hint.

fn main() {
    //Create variable to get the return value of the function
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    // create a new empty vector
    let vec = Vec::new();
    //assign vec0 to vec
    let mut vec0 = vec;
    //put values into vec0
    vec0.push(22);
    vec0.push(44);
    vec0.push(66);

    vec0
}
