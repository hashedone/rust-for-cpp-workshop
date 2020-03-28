fn fib_while(mut n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    if n == 0 {
        return 0;
    }

    while n > 1 {
        n -= 1;

        let c = a + b;
        a = b;
        b = c;
    }

    b
}

fn fib_loop(mut n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    if n == 0 {
        return 0;
    }

    loop {
        if n == 1 {
            return b;
        }

        n -= 1;

        let c = a + b;
        a = b;
        b = c;
    }
}

fn fib_for(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    if n == 0 {
        return 0;
    }

    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

fn fib_rec(n: u32) -> u32 {
    fn inner(a: u32, b: u32, n: u32) -> u32 {
        if n == 0 {
            a
        } else if n == 1 {
            b
        } else {
            inner(b, a + b, n - 1)
        }
    }

    inner(0, 1, n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib_while() {
        assert_eq!(0, fib_while(0));
        assert_eq!(1, fib_while(1));
        assert_eq!(1, fib_while(2));
        assert_eq!(2, fib_while(3));
        assert_eq!(3, fib_while(4));
        assert_eq!(5, fib_while(5));
        assert_eq!(55, fib_while(10));
    }

    #[test]
    fn test_fib_loop() {
        assert_eq!(0, fib_loop(0));
        assert_eq!(1, fib_loop(1));
        assert_eq!(1, fib_loop(2));
        assert_eq!(2, fib_loop(3));
        assert_eq!(3, fib_loop(4));
        assert_eq!(5, fib_loop(5));
        assert_eq!(55, fib_loop(10));
    }

    #[test]
    fn test_fib_for() {
        assert_eq!(0, fib_for(0));
        assert_eq!(1, fib_for(1));
        assert_eq!(1, fib_for(2));
        assert_eq!(2, fib_for(3));
        assert_eq!(3, fib_for(4));
        assert_eq!(5, fib_for(5));
        assert_eq!(55, fib_for(10));
    }

    #[test]
    fn test_fib_rec() {
        assert_eq!(0, fib_rec(0));
        assert_eq!(1, fib_rec(1));
        assert_eq!(1, fib_rec(2));
        assert_eq!(2, fib_rec(3));
        assert_eq!(3, fib_rec(4));
        assert_eq!(5, fib_rec(5));
        assert_eq!(55, fib_rec(10));
    }
}
