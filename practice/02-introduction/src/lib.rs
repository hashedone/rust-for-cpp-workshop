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
