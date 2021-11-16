// returns Fibonacci Sequence as Vec<u32>
// fib_seq(15); => "<fibonacci sequence>"
pub fn fib_seq(_times: u8) -> Vec<u32> {
    let mut _fib_seq: Vec<u32> = vec![0, 1]; // I have to declare first and second before the loop
    let mut _first: usize = 1;
    let mut _second: usize = 0;
    let mut _res: u32;

    for _i in 0.._times {
        _res = _fib_seq[_first] + _fib_seq[_second]; // doing the actual fibonacci thing
        _fib_seq.push(_res);
        _first += 1;
        _second += 1;
    }
    return _fib_seq;
}
