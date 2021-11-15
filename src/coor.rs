// prints the coordinates of the given x and y value
// coor(8, 16, 32); => "<coordinate table>""
pub fn coor(mut _x: u8, _y: u8, _table_size: u8) {

    if _table_size < _x || _table_size < _y || _x == 0 || _y == 0 {
        println!("!");
        return;
    }
    _x *= 2;

    let mut _y_axis: u8;

    for _i in 0.._table_size {
        _y_axis = _table_size - _i;
        
        if _y_axis == _y {
            
            if _table_size > 9 && _y_axis < 10 {
                print!(" {}", _y_axis);

                for _j in 1.._x {
                    print!("-");
                }
            }

            else {
                print!("{}", _y_axis);

                for _j in 0.._x - 1 {
                    print!("-");
                }
            }
            print!("x");

            for _j in 0.._table_size * 2 - _x {
                print!("-");
            }
            println!();
        }

        else {

            if _table_size > 9 && _y_axis < 10 {
                print!(" {}", _y_axis);
            }

            else {
                print!("{}", _y_axis);
            }

            for _j in 1.._x {
                print!(" ");
            }
            println!("|");           
        }
    }

    let mut _x_axis: Vec<char> = vec![]; 
    let mut _x_axis_collector: Vec<char>;
    
    if _table_size > 9 {

        let mut _x_axis_second_digit: usize = 1;

        print!(" 0 ");

        for _i in 1.._table_size + 1 {

            if _i > 9 {
                _x_axis_collector = Box::leak(_i.to_string().into_boxed_str()).chars().collect();

                for _j in _x_axis_collector {
                    _x_axis.push(_j);
                }
            }

            else {
                print!("{} ", _i);
            }
        }

        for _i in 0.._x_axis.len() / 2 {
            print!("{} ", _x_axis[_i  * 2]);
        }
        print!("\n                     0 ");

        for _i in 0.._x_axis.len() / 2 - 1 {
            _x_axis_second_digit += 2;
            print!("{} ", _x_axis[_x_axis_second_digit]);
        }
    }
    
    else {
        print!("0 ");
        
        for _i in 1.._table_size + 1 {
            print!("{} ", _i);
        }
    }
    println!();
}