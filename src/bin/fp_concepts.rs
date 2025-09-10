//! Functional Programming Concepts in Rust
//!
//! Demonstrates core functional programming concepts in Rust including:
//! - Pure functions
//! - Lambdas and closures
//! - Iterators (map, filter, sum)
//! - Recursion
//! - Function composition, currying, and partial application
//! - Enums and pattern matching
//! - Option handling with safe creation and pattern matching
//!
//! September 2025

/// # Pure Function
/// A pure function always returns the same result for the same input
/// and does not produce any side effects.
pub fn pure_add(a: i32, b: i32) -> i32 {
    a + b
}

/// # Lambda Example
/// Returns an anonymous function (lambda) that squares its input.
pub fn lambda_example() -> impl Fn(i32) -> i32 {
    |x| x * x
}

/// # Closure Example
/// Returns a closure that captures the external `factor` variable.
pub fn closure_example(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

/// # Map Generic
/// Applies a transformation to each element in a slice.
pub fn map_generic<T, U, F>(nums: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    nums.iter().map(|x| f(x)).collect()
}


/// # Filter + Sum
/// Filters positive numbers from a slice and returns their sum.
pub fn sum_positive(nums: &[i32]) -> i32 {
    nums.iter().filter(|&&x| x > 0).sum()
}

/// # List Comprehension Style
/// Returns squares of positive numbers from a slice.
pub fn squares_of_positive(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&x| x > 0).map(|&x| x * x).collect()
}

/// # Recursion
/// Computes the factorial of a number recursively.
pub fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

/// # Function Composition
/// Composes two functions: (f ∘ g)(x) = f(g(x))
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

/// # Currying Example
/// Returns a function that adds `a` to its argument `b`.
pub fn curry_add(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

/// # Partial Application
/// Fixes the first argument of a two-argument function.
pub fn partial_first<A, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
where
    F: Fn(A, B) -> C + Copy,
    A: Copy,
{
    move |b| f(a, b)
}

/// # Enum + Pattern Matching Example
/// Represents a simple arithmetic expression.
#[derive(Debug)]
pub enum Expr {
    Const(i32),
    Add(Box<Expr>, Box<Expr>),
}

/// Evaluates an arithmetic expression recursively.
pub fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Const(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
    }
}


/// # Option Handling Example
/// Represents a human being with a name.
#[derive(Debug)]
pub struct Human {
    name: String,
}

/// Safely creates a `Human`. Returns `None` if the name is empty.
pub fn get_human(name: &str) -> Option<Human> {
    if name.is_empty() {
        None
    } else {
        Some(Human {
            name: name.to_string(),
        })
    }
}

/// Demonstrates safe usage of Option with pattern matching.
pub fn greet(name: Option<&str>) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("Hello, stranger!"),
    }
}

/// # Main demo
fn main() {
    println!("pure_add(2, 3) = {}", pure_add(2, 3));

    let sq = lambda_example();
    println!("lambda 5^2 = {}", sq(5));

    let double = closure_example(2);
    println!("closure double 7 = {}", double(7));
    
    let squared = map_generic(&[1, 2, 3], |x| x * x);
    println!("[1,2,3] squared-> {:?}", squared);

    println!("sum_positive([-2, 3, 5]) = {}", sum_positive(&[-2, 3, 5]));
    println!("squares_of_positive([-1, 2, 3]) = {:?}", squares_of_positive(&[-1, 2, 3]));

    println!("factorial(5) = {}", factorial(5));

    let f = compose(|x| x + 7, |x| x * 5);
    println!("compose (x*2)+1 for 3 = {}", f(3));

    println!("curry_add(5)(7) = {}", curry_add(5)(7));

    let add10 = partial_first(|a, b| a + b, 10);
    println!("partial add10(3) = {}", add10(3));

    // Enum + Pattern Matching
    let expr = Expr::Add(Box::new(Expr::Const(2)), Box::new(Expr::Const(4)));
    println!("eval(Add(Const 2, Const 4)) = {}", eval(&expr));

    // Option Handling
    match get_human("Alice") {
        Some(human) => println!("Created human: {:?}", human.name),
        None => println!("No human created"),
    }

    match get_human("") {
        Some(human) => println!("Created human: {:?}", human.name),
        None => println!("No human created"),
    }

    // Pattern matching with Option
    greet(Some("Alice"));
    greet(None);
}
