// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

/**
 * METHODS
 * 1. vec0.clone()
 * 2. move vec0 with &vec0 and modify fill_vec with move and then to_owned or clone
 * 3. as described below
 *
 **/
fn main() {
    // mutable vec allows pushing elements
    let mut vec0 = Vec::new();

    // borrowed a mutable ref to the mutable vec
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}
// without returning a value
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
