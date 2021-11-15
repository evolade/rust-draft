// anagram("rust", "ruts"); => true
pub fn anagram(_word: &str, _word2: &str) -> bool {
    if _word == _word2 {
        return true;
    }

    if _word.len() != _word2.len() {
        return false
    }

    let mut _word_vec: Vec<char> = vec![];
    let mut _word2_vec: Vec<char> = vec![];
    let mut _is_equal: bool = false;
    let mut _point: u8 = 0;
    
    for _i in _word.chars() {
        _word_vec.push(_i);
        
    }

    for _i in _word2.chars() {
        _word2_vec.push(_i);
        
    }

    let _word_vec_len: usize = _word_vec.len(); // instead of calling .len() func every time, it will use this

    for _i in 0.._word_vec_len {

        for _j in 0.._word_vec_len {

            if _word_vec[_i] == _word2_vec[_j] {
                _is_equal = true;
            }
        }

        if _is_equal {
            _point += 1;
        }
        _is_equal = false;
    }

    for _i in 0.._word_vec_len {

        for _j in 0.._word_vec_len {

            if _word2_vec[_i] == _word_vec[_j] {
                _is_equal = true;
            }
        }

        if _is_equal {
            _point += 1;
        }
        _is_equal = false; 
    }

    if _point == _word_vec_len as u8 * 2 {
        return true;
    }
    return false;
}