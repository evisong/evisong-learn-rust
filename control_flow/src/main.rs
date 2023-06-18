fn fib_add(a: u8, b: u8) -> (u8, u8) {
    let c = a + b;
    println!("Next val is {}", c);
    return (b, c);
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        (a, b) = fib_add(a, b);
        i += 1;

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        (a, b) = fib_add(a, b);
        i += 1;
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        (a, b) = fib_add(a, b);
    }
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
