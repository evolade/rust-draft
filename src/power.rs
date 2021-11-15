// takes power
// power(3, 4); => 81
pub fn power(_growth_rate: u16, _times: u16) -> u32{
    let mut _res: u32 = _growth_rate as u32;

    for _i in 0.._times - 1 {
        _res *= _growth_rate as u32;
    }
    return _res;
}