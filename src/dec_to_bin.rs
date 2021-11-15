// converts decimal to binary
// dec_to_bin(89); => "1011001"
pub fn dec_to_bin(mut _dec: u32) -> String {
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