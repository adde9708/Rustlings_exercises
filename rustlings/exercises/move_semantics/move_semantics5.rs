// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

fn main() {
    //Assign 100 to x
    let mut x = 100;

    // Create the variable y which points to x
    let y = &mut x;

    //Increment y by 100 and remember that y is still x,
    //so basically this just increments x by 100
    *y += 100;

    /*
      These two lines does the exact same thing as what
      was done with the y variable
    */
    let z = &mut x;
    *z += 1000;

    assert_eq!(x, 1200);
}
