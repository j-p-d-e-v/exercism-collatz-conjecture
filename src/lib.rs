/* 
Instructions
The Collatz Conjecture or 3x+1 problem can be summarized as follows:

Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is odd, multiply n by 3 and add 1 to get 3n + 1. Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.

Given a number n, return the number of steps required to reach 1.
*/

pub fn collatz(n: u64) -> Option<u64> {
    match n {
        1 => Some(0),
        x if x > 0 => {
            let mut n: u64 = n;
            let mut steps: u64 = 0;
            while n > 1{
                if n % 2 == 0 {
                    n = n / 2 ;
                }
                else{
                    n = (n * 3) + 1;
                }
                steps += 1;
            }
            Some(steps)
        },
        _ => None,
    }
}

#[test]
fn zero_steps_for_one() {
    let output = collatz(1);
    let expected = Some(0);
    assert_eq!(output, expected);
}
#[test]
fn divide_if_even() {
    let output = collatz(16);
    let expected = Some(4);
    assert_eq!(output, expected);
}
#[test]
fn even_and_odd_steps() {
    let output = collatz(12);
    let expected = Some(9);
    assert_eq!(output, expected);
}
#[test]
fn large_number_of_even_and_odd_steps() {
    let output = collatz(1000000);
    let expected = Some(152);
    assert_eq!(output, expected);
}
#[test]
fn zero_is_an_error() {
    let output = collatz(0);
    let expected = None;
    assert_eq!(output, expected);
}