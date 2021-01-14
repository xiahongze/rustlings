// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

/**
 * You can have values, immutable references to values, and mutable references to values
Values are neither immutable or mutable
Instead, variables are immutable or mutable
When you move a value into a new variable (via a let or a function call), you can change the mutability of the variable
The mutability and names of variables in function signatures do not affect the signature of the function
The mutability of a reference is built into the type itself, so you can't "upgrade" an immutable reference to a mutable one
* */
fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    // if i print vec0 before vec1 it complaints, because println! borrows vars as immutable
    // cannot borrow `vec0` as immutable because it is also borrowed as mutable
    // but if vec0 is borrowed as mutable and then it can be borrowed as immutable
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec // to_owned, to_vec, clone is to duplicate data
}
