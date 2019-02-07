fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    else if n == 1 {
        return 1;
    }
    else {
        return fibonacci(n-1) + fibonacci(n-2)
    }
}

fn main() {
    for i in 0..10 {
        println!("fibonacci({}) = {}", i, fibonacci(i));
    }
}
