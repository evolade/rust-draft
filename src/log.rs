// logarithm
// log(2, 32); => 5
pub fn log(_base: u16, _num: u16) -> u16 {
    for _i in 0.._num {
        if u16::pow(_base, _i as u32) == _num {
            return _i;
        }
    }
    return 0;
}