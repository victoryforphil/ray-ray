mod math;
pub use math::*;
mod rendering;
pub use rendering::*;
fn main() {
    let a = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
    let b: Matrix = a.sub_matrix();
    println!("SubMatrix {:#?}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
