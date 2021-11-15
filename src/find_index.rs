// finds the index of a string in the choosen array (vector)
// find_index(_my_array, "apple"); => <some index>
pub fn find_index(_arr: Vec<&str>, _target: &str) -> u16{

    for _i in 0.._arr.len() {

        if _arr[_i] == _target {
            return _i as u16;

        }
    }
    return 404;
}