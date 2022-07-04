fn main(){
    let tup_1: (i32, f64, u8) = (500, 6.4, 1);
    let tup_2 = (500, 6.4, 1);
    let (x, y, z) = tup_2;
    println!("The value of y is: {}", y);
    let five_hundred = tup_2.0;
    let six_point_four = tup_1.1;
    println!("{}",five_hundred);
    println!("{}",six_point_four);
}
