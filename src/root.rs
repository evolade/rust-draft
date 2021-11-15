// takes root 
// root(64, 3); => 4 
pub fn root(_root: u16, _type: u16) -> u16{
    let mut _res: u16 = 1; 

    for _i in 0.._root {
        if u16::pow(_res , _type as u32) == _root {
            return _res;
        }
        _res += 1;
    }
    return 0;
}