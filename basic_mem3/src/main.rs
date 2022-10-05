fn main() {
    let mut n1 = 1;
    let n2 = &mut n1;
    // let n3 = n2;
    // let n4 = &n2;
    *n2 = 2;
    println!("n2 =  {}", n2);
    println!("n1 =  {}", n1);
}