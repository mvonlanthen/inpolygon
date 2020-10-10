use ndarray::prelude::*;
// use ndarray;
// use inpolygon::pt_in_polygon;
use inpolygon::pts_in_polygon;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use std::time::Instant;


// fn test_func(a1: &Array2<f64>, a2: &ArrayBase<ndarray::ViewRepr<&f64>, Dim<[usize; 2]>>) {
//     println!("foo , bar!");
// }

pub fn main() {
    let polygon = array![
        [-1.0,-1.0],
        [1.0,-1.0],
        [1.0,1.0],
        [-0.5,1.0],
        [-1.0,0.0],
        [-1.0,-1.0]
    ];

    // let points = array![
    //     [0., 0.],
    //     [2., 0.],
    //     [1., 0.],
    //     [0., -1.]
    // ];

    // generate a ton of points for benchmark!
    let nb_points = 10_000_000;
    let points = Array::random((nb_points, 2), Uniform::new(-2., 2.));

    let now = Instant::now();
    let _is_inside = pts_in_polygon(&points.view(), &polygon.view(), true);
    let exec_time = now.elapsed().as_micros() as f64;
    println!("exec time: {}s", exec_time/1000000.0);
    // // finally
    // println!("{}", is_inside);

    
    // let polygon = array![
    //     [-1.0,-1.0],
    //     [1.0,-1.0],
    //     [1.0,1.0],
    //     [-0.5,1.0],
    //     [-1.0,0.0],
    //     [-1.0,-1.0]
    // ];
    // let point = array![-1.,1.];
    // println!("point {}", point);

    // let is_inside = pt_in_polygon(&point.view(), &polygon.view(), true);
    // println!("{}", is_inside);

}