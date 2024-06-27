// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


#[test]
// fn main() {
//     let vec0 = vec![22, 44, 66];

//     let vec1 = fill_vec(vec0);

//     assert_eq!(vec1, vec![22, 44, 66, 88]);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let vec = vec;

//     vec.push(88);

//     vec
// }

fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // let mut vec = vec;
    // vec.push(88);
    // vec.to_vec()

    let mut vec = vec.clone();
    vec.push(88);
    vec
}
