fn main() {
    // println!("Hello, world!");
    let n = 10;
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    
    if n < 0 {
        println!("Incorrect input");
    } else if n == 0 {
        println!("{}", n);
    } else if n == 1 {
        println!("{}", n);
    } else {
        for _ in 2..(n+1){
            c = a+b;
            a = b;
            b = c;
        }
        println!("{}th fubonnacci number is {}", n, c);
    }
    
}
