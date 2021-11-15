// a game you prolly already know, if a number can divisible by 3 you say fizz
// if a number can divisible by 5 you say buzz
// if its both you say fizzbuzz
// fizzbuzz(30); => "<result>"
pub fn fizzbuzz(_reps: u8) -> Vec<&'static str> {
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