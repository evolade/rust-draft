// converts binary to decimal
// bin_to_dec("1011001"); => 89
pub fn bin_to_dec(_bin: &str) -> u32{
    let _r_bin: String = String::from(_bin.chars().rev().collect::<String>()); // reverse <_bin>
    let  _r_bin_arr: Vec<char> = _r_bin.chars().collect(); // <_r_bin> to vector
    
    let mut _res: u32 = 0;
    let mut _calc: u16;
    
    for _i in 0.._r_bin.len() {
        _calc = u16::pow(2, _i as u32);

        if _r_bin_arr[_i] == '1' {
            _res += _calc as u32;
        }
    }
    return _res;
}