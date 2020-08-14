fn main() {
    for n in 0..10 {
        println!("the value is: {}", fib(n));
    }
}

fn fib(n: i32) -> i32 {
    return match n {
        0 => 1,
        1 => 1,
        _ => fib(n-2) + fib(n-1),
    }
}

