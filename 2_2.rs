use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )",
            self.0,
            self.1,
            self.2,
            self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let (xx, xy, yx, yy) = (matrix.0, matrix.1, matrix.2, matrix.3);
    Matrix(xx, yx, xy, yy)
}

fn main() {
    let pair = (1, true);
    println!("{:?}", reverse(pair));

    let single_tuple = (2.5,);
    println!("{:?}", single_tuple);

    let value1 = single_tuple.0;
    //let value2 = single_tuple.1;
    println!("{:?}", value1);

    let m1 = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", m1);
    println!("transpose:\n{}", transpose(m1));
}
