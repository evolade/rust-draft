// factorial
// fact(6); => 720
pub fn fact(_num: u16) -> u32{
    let mut _res: u32 = 1;

    for _i in 0.._num {
        _res = ((_i + 1) * _res as u16) as u32;
    } 
    return _res;
}