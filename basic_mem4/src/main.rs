#![allow(dead_code, unused_variables)] 

fn main() {
     let mut n1 = 1;
     hello(&mut n1);
     println!("n1 = {}", n1);
}

fn hello(a:&mut i32){
    *a = 20;
    println!("arg: {}", a);
}