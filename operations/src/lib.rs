pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn pythagorean_theorem(x: f64, y: f64) -> f64 {
    (x.abs().powf(2.0) + y.abs().powf(2.0) as f64).sqrt() as f64
}
