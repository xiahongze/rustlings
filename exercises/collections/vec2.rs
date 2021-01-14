// vec2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute the command `rustlings hint collections2` if you need
// hints.

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *i *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_loop_1(v: &mut Vec<i32>) {
    for i in v.iter_mut() {
        *i *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
    /**
     *
     *  iter() iterates over the items by reference
     *  into_iter() iterates over the items, moving them into the new scope
     *  iter_mut() iterates over the items, giving a mutable reference to each item
     */
    #[test]
    fn test_vec_loop_1() {
        let mut v: Vec<i32> = (10..).filter(|y| y % 3 == 1).take(5).collect();
        let v2 = v.clone();
        vec_loop_1(&mut v);
        // vec.iter() or vec.into_iter(), but if one moves then one can not reference to it again
        // on ehas to tell collect what type it is going to collect in
        assert_eq!(v, v2.iter().map(|x| x * 2).collect::<Vec<i32>>());
        println!("{:?}", v2);
    }
}
