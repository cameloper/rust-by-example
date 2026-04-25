use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elemens", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    
    let ys: [i32; 500] = [0; 500];

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    analyze_slice(&xs);
    analyze_slice(&xs[1 .. 4]);
    analyze_slice(&ys[3 .. 6]);

    let emtpy_array: [u32; 0] = [];
    assert_eq!(&emtpy_array, &[]);
    assert_eq!(&emtpy_array, &[][..]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i)
        }
    }

    let mut num = 0;
    match xs.get(3) {
        Some(number) => num = *number,
        None => println!("Index {} too big", 3),
    }

    println!("Number {}", num);
}
