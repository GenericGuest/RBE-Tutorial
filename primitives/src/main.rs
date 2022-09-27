use std::fmt::{self, Display};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair; // We can deconstruct a tuple
    (bool_param, int_param) // No ; so we return the value?
}

fn transpose(matrix: Matrix) -> Matrix
{
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

struct Matrix(f32, f32, f32, f32);

impl Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({}, {})", self.0, self.1);
        writeln!(f, "({}, {})", self.2, self.3)
    }
}

fn main() {
    let my_bool: bool = true;
    let my_int: i8 = 126;
    let my_uint: u16 = 20000;
    let my_float: f64 = 1234543.23;
    let my_char: char = 'a';
    
    let my_tuple : (bool, bool) = (true, false);
    let mut my_mut: u8 = 12;
    println!("{}", my_mut);
    my_mut = 16;

    println!("{}, {}, {}, {}, {}, {}, {}", my_bool, my_int, my_uint, my_float, my_char, my_tuple.0, my_mut);

    println!("1 - 2 = {}", 1i32-2);
    println!("True or false = {}", true || false);
    println!("0x0f | 0xf0 = 0x{:x}", 0b00001111u32 | 0b11110000u32);
    println!("One mil: {}", 1_000_000u32);
    let my_matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", my_matrix);
    println!("{}", transpose(my_matrix));
}
