/// # if表达式
///
/// Basic usage:
///
/// ```
/// fn if_expr() {
///     let n = 13;
///     let big_n = if n < 10 && n > -10 {
///         10 * n
///     } else {
///         n / 2
///     };
///     assert_eq!(big_n, 6);
/// }
/// if_expr();
/// ```
pub fn if_expr() {
    let n = 13;
    let big_n = if n < 10 && n > -10 {
        10 * n
    } else {
        n / 2
    };
    assert_eq!(big_n, 6);
}

/// # while表达式
///
/// Basic usage:
///
/// ```
/// fn while_fizzbuzz() {
///     let mut n = 1;
///     while n < 101 {
///         if n % 15 == 0 {
///             println!("fizzbuzz");
///         } else if n % 3 == 0 {
///             println!("fizz");
///         } else if n % 5 == 0 {
///             println!("buzz");
///         } else {
///             println!("{}", n);
///         }
///         n += 1;
///     }
/// }
/// while_fizzbuzz();
/// ```
pub fn while_fizzbuzz() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

/// # while表达式
///
/// Basic usage:
///
/// ```
/// fn loop_fizzbuzz() {
///     let mut n = 1;
///     loop {
///         if n > 101 { break; }
///         if n % 15 == 0 {
///             println!("fizzbuzz");
///         } else if n % 3 == 0 {
///             println!("fizz");
///         } else if n % 5 == 0 {
///             println!("buzz");
///         } else {
///             println!("{}", n);
///         }
///         n += 1;
///     }
/// }
/// loop_fizzbuzz();
/// ```
pub fn loop_fizzbuzz() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

/// # for表达式
///
/// Basic usage:
///
/// ```
/// fn for_fizzbuzz() {
///     for n in 1..101 {
///         if n % 15 == 0 {
///             println!("fizzbuzz");
///         } else if n % 3 == 0 {
///             println!("fizz");
///         } else if n % 5 == 0 {
///             println!("buzz");
///         } else {
///             println!("{}", n);
///         }
///     }
/// }
/// for_fizzbuzz();
/// ```
pub fn for_fizzbuzz() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

/// # while true
///
/// Basic usage:
///
/// ```
/// fn while_true(x: i32) -> i32 {
///     while true {  // error[E0308]: mismatched types，expected type `i32` found type `()`
///         return x+1;
///     }
/// }
/// let y = while_true(5);
/// assert_eq!(y, 6);
/// ```
pub fn while_true(x: i32) -> i32 {
    while true {
        return x+1;
    }
}

/// # if true
///
/// Basic usage:
///
/// ```
/// fn if_true(x: i32) -> i32 {
///     if true {  // error[E0308]: mismatched types，expected type `i32` found type `()`
///         return x+1;
///     }
/// }
/// let y = if_true(5);
/// assert_eq!(y, 6);
/// ```
pub fn if_true(x: i32) -> i32 {
    if true {
        return x+1;
    }
}
