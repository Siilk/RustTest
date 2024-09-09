use std::any::{Any};
use std::mem;

#[allow(dead_code)]
#[allow(unused)]
#[allow(non_snake_case)]
fn run_arrays()
{
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    arrInfo(&arr);

    //[1, 2, [3, 4, 5, 6], 7, 8, 9, 10];
    let slice = &arr[2..6];

    arrInfo(slice);

    fn arrInfo(a: &[i32]) {
        println!("first: {}", a[0]);
        println!("last: {}", a[a.len() - 1]);

        println!("length: {}", a.len());

        println!("mem size: {}\n", mem::size_of_val(&a));
    }

    println!("mem size: {}\n", mem::size_of_val(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    println!("mem size: {}\n", mem::size_of_val(&[3, 4, 5, 6]));

    let xs = [1, 2, 3, 4, 5];
    for i in 0..xs.len() + 1
    {
        let option = xs.get(i);
        println!("option {:?}", option);
        match option
        {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    // Out of bound indexing on slice causes runtime error.
    println!("{}", xs[..][5]);
}

