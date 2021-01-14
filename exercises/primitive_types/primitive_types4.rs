// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    // & borrowing is necessary because without it, compiler does not know the
    // size at compilation time
    // Slices are similar to arrays, but their length is not known at compile time.
    // Instead, a slice is a two-word object, the first word is a pointer to the data,
    // and the second word is the length of the slice. The word size is the same as usize,
    // determined by the processor architecture eg 64 bits on an x86-64.
    // Slices can be used to borrow a section of an array, and have the type signature &[T].
    // Another data type that does not have ownership is the slice.
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
