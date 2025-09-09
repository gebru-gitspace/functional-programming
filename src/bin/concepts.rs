//! Functional Programming Concepts in Rust
//!
//! Simplified: highlights pure functions, lambdas,
//! recursion, composition, enums, and finally the builder pattern.

/// Pure Function
/// A pure function always returns the same result for the same input,
/// with no side effects.

//// September 2025
pub fn pure_add(a: i32, b: i32) -> i32 {
    a + b
}

/// Lambda (anonymous function)
pub fn lambda_example() -> impl Fn(i32) -> i32 {
    |x| x * x
}

/// Closure (captures environment)
pub fn closure_example(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

/// Map (apply a function to each element)
pub fn map_example(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|x| x * 2).collect()
}

/// Filter + Sum
pub fn sum_positive(nums: &[i32]) -> i32 {
    nums.iter().filter(|&&x| x > 0).sum()
}

/// List Comprehension style (filter + map + collect)
pub fn squares_of_positive(nums: &[i32]) -> Vec<i32> {
    nums.iter().filter(|&&x| x > 0).map(|&x| x * x).collect()
}

/// Recursion (factorial)
pub fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

/// Function Composition (f ∘ g)
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C + Copy,
    G: Fn(A) -> B + Copy,
{
    move |x| f(g(x))
}

/// Currying (returning functions)
pub fn curry_add(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

/// Partial Application
pub fn partial_first<A, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
where
    F: Fn(A, B) -> C + Copy,
    A: Copy,
{
    move |b| f(a, b)
}

/// Enum + Pattern Matching
#[derive(Debug)]
pub enum Expr {
    Const(i32),
    Add(Box<Expr>, Box<Expr>),
}

pub fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Const(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
    }
}

/// Builder Pattern (advanced example)
#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder { host: None, port: None }
    }
}

pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
}

impl ConfigBuilder {
    pub fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    pub fn build(self) -> Config {
        Config {
            host: self.host.unwrap_or("127.0.0.1".to_string()),
            port: self.port.unwrap_or(80),
        }
    }
}

/// A simple struct representing a human with a name.
#[derive(Debug)]
pub struct Human {
    name: String,
}

/// Creates a `Human` if the given name is not empty.
/// # Returns
/// * `Some(Human)` if the name is non-empty
/// * `None` if the name is empty
pub fn get_human(name: &str) -> Option<Human> {
    if name.is_empty() {
        None
    } else {
        Some(Human {
            name: name.to_string(),
        })
    }
}

/// Demonstrates pattern matching with Option
pub fn greet(name: Option<&str>) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("Hello, stranger!"),
    }
}

/// Small demo
fn main() {
    println!("pure_add(2,3) = {}", pure_add(2, 3));

    let sq = lambda_example();
    println!("lambda 5^2 = {}", sq(5));

    let double = closure_example(2);
    println!("closure double 7 = {}", double(7));

    println!("map_example([1,2,3]) = {:?}", map_example(&[1, 2, 3]));
    println!("sum_positive([-2, 3, 5]) = {}", sum_positive(&[-2, 3, 5]));
    println!("squares_of_positive([-1,2,3]) = {:?}", squares_of_positive(&[-1, 2, 3]));

    println!("factorial(5) = {}", factorial(5));

    let f = compose(|x| x + 1, |x| x * 2);
    println!("compose (x*2)+1 for 3 = {}", f(3));

    println!("curry_add(5)(7) = {}", curry_add(5)(7));

    let add10 = partial_first(|a, b| a + b, 10);
    println!("partial add10(3) = {}", add10(3));

    // Demonstrate Enum + Pattern Matching
    let expr = Expr::Add(Box::new(Expr::Const(2)), Box::new(Expr::Const(4)));
    println!("eval(Add(Const 2, Const 4)) = {}", eval(&expr));

    // Demonstrate Builder Pattern
    let cfg = Config::builder().host("localhost").port(8080).build();
    println!("Config: {:?}", cfg);

    // Demonstrate Option with get_human-> handling type errors safely
    match get_human("Alice") {
        Some(human) => println!("Created human: {:?}", human.name),
        None => println!("No human created"),
    }

    match get_human("") {
        Some(human) => println!("Created human: {:?}", human.name),
        None => println!("No human created"),
    }

    // Demonstrate pattern matching with Option
    greet(Some("Alice"));
    greet(None);
}
