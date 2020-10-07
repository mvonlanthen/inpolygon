use ndarray::prelude::*;
// use ndarray;
use inpolygon::pt_in_polygon;
// use ndarray_rand::RandomExt;
// use ndarray_rand::rand_distr::Uniform;


fn test_func(a1: &Array2<f64>, a2: &ArrayBase<ndarray::ViewRepr<&f64>, Dim<[usize; 2]>>) {
    println!("foo , bar!");
}

pub fn main() {
    let polygon  = array![
        [-1., -1.],
        [ 1., -1.],
        [ 1.,  1.],
        [-1.,  1.],
        [-1., -1.]
    ];

    let points = array![
        [0., 0.],
        [2., 0.],
        [1., 0.],
        [0., -1.]
    ];

    let points_view = points.slice(s![..3, ..]);

    // let points = Array::random(
    //     (1000000, 2), 
    //     Uniform::new(-2., 2.)
    // );

    test_func(&points, &points.slice(s![.., ..]));


    let mut x: f64;
    // let mut x: f64 = 0.;
    let mut y: f64;
    let mut is_inside: bool;
    for point in points.axis_iter(Axis(0)) {
        x = point[0];
        y = point[1];
        is_inside = pt_in_polygon(&x, &y, &polygon, true);
        println!("[{:.02}, {:.02}] is inside: {}", x, y, is_inside);
    }



}