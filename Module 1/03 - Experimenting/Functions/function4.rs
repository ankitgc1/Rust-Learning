// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.

fn main() {
    let original_price = 50;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(mut price: i32) -> i32 {
    if is_even(price) {
        price = price - 10;
    } else {
        price = price - 3;
    }
    return price;
}

fn is_even(num: i32) -> bool {
    return num % 2 == 0
}