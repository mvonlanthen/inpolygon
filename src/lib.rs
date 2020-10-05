use ndarray::prelude::*;




pub fn in_polygon(polygon: &Array2<f64>, points: &Array2<f64>) {

}


pub fn sum(a: f32, b: f32) -> f32 {
    return a+b;
}


// Some useful links:
// * ndarray quick tutorial
//  https://github.com/rust-ndarray/ndarray/blob/master/README-quick-start.md





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(1.0, 4.0), 5.0)
    }
}
