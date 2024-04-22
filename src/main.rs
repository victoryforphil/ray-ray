mod math;
pub use math::*;
mod rendering;
pub use rendering::*;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
