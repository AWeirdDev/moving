use moving::{ move_vec_to_array };

fn main() {
    let v = vec![0, 1, 2, 3, 4]; // 5 items
    let arr = move_vec_to_array::<i16, 5>(v).unwrap();

    assert_eq!(arr, [0, 1, 2, 3, 4])
}
