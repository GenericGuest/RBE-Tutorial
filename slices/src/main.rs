use std::mem;
// Arrays - length known at compile time, slices - length not known at compile time.
// slice is two words - a pointer to the data and the length, usually word length is the same as usize (ie probably 64 bits)

fn analyze_slice(slice: &[i32])
{
    println!("First element: {}", slice[0]);
    println!("Slice length: {}", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5]; // i32 = 4 bytes per int, 4 (bytes / int) * 5 (int) = 20 bytes
    let ys: [i32; 500] = [0; 500]; //initialize with 500 0's
    println!("First element of ys: {}", ys[0]);
    println!("Ys length: {}", ys.len());
    println!("Array occupies {} bytes", mem::size_of_val(&xs)); // 20 bytes!
    analyze_slice(&xs); // so we pass pointers like in c++

    analyze_slice(&ys[1..4]); // We can slice... existing arrays

    for i in 0..xs.len() + 1 {
        match xs.get(i) { // Scala like matching :O!
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),   
        }
    }
}
