use ndarray::prelude::*;



/// documentation goes here
/// # some title
/// foo bar blah blah blah.
pub fn in_polygon(polygon: &Array2<f64>, points: &Array2<f64>, include_edges: bool) {
    // extract some ref for readability later on
    let xs = points.slice(s![.., 0]);
    let ys = points.slice(s![.., 1]);
    let n = polygon.len();
    let counters: Array<i32, Ix1> = Array::zeros(xs.len());

    // can we do better than this for loop. I just want an array with integers
    // from 0 to xs.len()...
    let mut indices: Array<usize, Ix1> = Array::zeros(xs.len());
    for i in (0..indices.len()) {
        indices[i] = i;
    }

    // a little trick to handle points on horizontal edges
    let mut count_on_horz = 1;
    if include_edges {count_on_horz = 2;}




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
