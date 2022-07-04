use std::io;

fn main(){
    let a = [1, 2, 3, 4, 5];
    let months = ["January","February","March","April","May","June","July","August","September","October","November","December"];
    let b: [i32; 5] = [1, 2, 3, 4, 5]; //5 indicates the array contains five elements
    let c = [3; 5]; //array a will contain 5 elements that will all be set to the value 3 [3, 3, 3, 3, ,3]

    //accessing array elements
    let first = a[0];
    let second = a[1];

    //invalid array element accessing
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
        );
}
