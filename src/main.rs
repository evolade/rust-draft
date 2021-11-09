use std::io::Write; // for input();

fn main() {
    println!("EVOLADE IS THE BEST");
    
    // example
    //println!("{:?}", fib_seq(20)); 
}

// a game you prolly already know, if a number can divisible by 3 you say fizz
// if a number can divisible by 5 you say buzz
// if its both you say fizzbuzz
// fizzbuzz(30); => "<result>"
fn fizzbuzz(_reps: u8) -> Vec<&'static str> {
    let mut _res_vec: Vec<&str> = vec![];
    for _i in 1.._reps + 1 {
        if _i % 3 == 0 && _i % 5 == 0 {
            _res_vec.push("FIZZBUZZ");
        }
        else if _i % 3 == 0 {
            _res_vec.push("FIZZ");
        }
        else if _i % 5 == 0 {
            _res_vec.push("BUZZ");
        }
        else {
            _res_vec.push(Box::leak(format!("{}",_i).into_boxed_str())); // convert _i (u8) to String and then to &str
        }
    }

    return _res_vec;
}

// anagram("rust", "ruts"); => true
fn anagram(_word: &str, _word2: &str) -> bool {
    if _word == _word2 {
        return true;
    }

    if _word.len() != _word2.len() {
        return false
    }

    let mut _word_vec: Vec<char> = vec![];
    let mut _word2_vec: Vec<char> = vec![];
    let mut _is_equal: bool = false;
    let mut _point: u16 = 0;
    
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

    if _point == _word_vec_len as u16 * 2 {
        return true;
    }
    return false;
}

// rust doesnt have a solid input system so I created one
// let _name = input("name: ");
fn input(_headline: &str) -> &str{
    print!("{}", _headline);
    std::io::stdout().flush().unwrap(); // idk what this thing does dont remove it
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).expect("err"); // take input
    _input.pop(); // pop the last index ("\n")
    let _str_input: &str = Box::leak(_input.into_boxed_str()); // convert String to &str
    return _str_input;
}

// takes root 
// root(64, 3); => 4 
fn root(_root: u16, _type: u16) -> u16{
    let mut _res: u16 = 1; 

    for _i in 0.._root {
        if u16::pow(_res , _type as u32) == _root {
            return _res;
        }
        _res += 1;
    }
    return 0;
}

// logarithm
// log(2, 32); => 5
fn log(_base: u16, _num: u16) -> u16 {
    for _i in 0.._num {
        if u16::pow(_base, _i as u32) == _num {
            return _i;
        }
    }
    return 0;
}

// factorial
// fact(6); => 720
fn fact(_num: u16) -> u32{
    let mut _res:u32 = 1;

    for _i in 0.._num {
        _res = ((_i + 1) * _res as u16) as u32;
    } 
    return _res;
}

// converts binary to decimal
// bin_to_dec("1011001"); => 89
fn bin_to_dec(_bin: &str) -> u16{
    let _r_bin = String::from(_bin.chars().rev().collect::<String>()); // reverse <_bin>
    
    let  _r_bin_arr: Vec<char> = _r_bin.chars().collect(); // <_r_bin> to vector
    let mut _res: u16 = 0;
    let mut _calc: u16;
    
    for _i in 0.._r_bin.len() {
        _calc = u16::pow(2, _i as u32);

        if _r_bin_arr[_i] == '1' {
            _res += _calc;
        }
    }
    return _res;
}

// converts decimal to binary
// dec_to_bin(89); => "1011001"
fn dec_to_bin(mut _dec: u16) -> String {
    let mut _res: String = String::from(""); 
    
    if _dec % 2 == 0 {
        _res.push_str("0");
    }

    else {
        _res.push_str("1");
    }

    for _i in 0.._dec {
        _dec /= 2;
        if _dec == 0 {
            break;
        }

        if _dec % 2 == 0 {
            _res.push_str("0");
        }

        else {
            _res.push_str("1");
        }
    }
    return _res.chars().rev().collect::<String>(); // reverse the string
}

// finds the index of a string in the choosen array (vector)
// find_index(_my_array, "apple"); => <some index>
fn find_index(_arr: Vec<&str>, _target: &str) -> u16{

    for _i in 0.._arr.len() {

        if _arr[_i] == _target {
            return _i as u16;

        }
    }
    return 404;
}

// prints Fibonacci Sequence
// it cant return multiple integers so it just prints it
// fib_seq(15); => "<fibonacci sequence>"
fn fib_seq(_times: u16) -> Vec<u16> {
    let mut _fib_seq: Vec<u16> = vec![0, 1]; // I have to declare first and second before the loop
    let mut _first: usize = 1;
    let mut _second: usize = 0;
    let mut _res: u16;

    for _i in 0.._times {
        _res = _fib_seq[_first] + _fib_seq[_second]; // doing the actual fibonacci thing
        //println!("{} - {}", _i + 3, _res);
        _fib_seq.push(_res);
        _first += 1;
        _second += 1;
    }
    return _fib_seq;
}

// takes power
// power(3, 4); => 81
fn power(_growth_rate: u16, _times: u16) -> u16{
    let mut _res = _growth_rate;

    for _i in 0.._times - 1 {
        _res *= _growth_rate;
    }
    return _res;
}