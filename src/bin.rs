use ndarray::prelude::*;
// use ndarray;
use inpolygon::pts_in_polygon;
// use ndarray_rand::RandomExt;
// use ndarray_rand::rand_distr::Uniform;


// fn test_func(a1: &Array2<f64>, a2: &ArrayBase<ndarray::ViewRepr<&f64>, Dim<[usize; 2]>>) {
//     println!("foo , bar!");
// }

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

    // let points = Array::random(
    //     (1000, 2), 
    //     Uniform::new(-2., 2.)
    // );

    // this:
    // let mut is_inside = Array::from_elem(points.len_of(Axis(0)), false);
    // for (i,point) in points.axis_iter(Axis(0)).enumerate() {
    //     is_inside[i] = pt_in_polygon(&point, &polygon.view(), true);
    // }
    // or that
    let is_inside = pts_in_polygon(&points.view(), &polygon.view(), true);
    // finally
    println!("{}", is_inside);

    // let mut is_inside: bool;
    // for point in points.axis_iter(Axis(0)) {
    //     is_inside = pt_in_polygon(&point, &polygon.view(), true);
    //     println!("[{:.02}, {:.02}] is inside: {}", point[0], point[1], is_inside);
    // }



}