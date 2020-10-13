use ndarray::prelude::*;
use inpolygon::pts_in_polygon;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use std::time::Instant;


pub fn main() {
    // let polygon = array![
    //     [-1.0,-1.0],
    //     [1.0,-1.0],
    //     [1.0,1.0],
    //     [-0.5,1.0],
    //     [-1.0,0.0],
    //     [-1.0,-1.0]
    // ];

    // generate a large random polygon for benchmark!
    let nb_poly_points = 200;
    let polygon = Array::random((nb_poly_points, 2), Uniform::new(-1., 1.));


    // generate a ton of points for benchmark!
    let nb_points = 10_000_000;
    let points = Array::random((nb_points, 2), Uniform::new(-2., 2.));

    let now = Instant::now();
    let _is_inside = pts_in_polygon(
        &points.view(), 
        &polygon.view(), 
        true,
        false
    );
    let exec_time = now.elapsed().as_micros() as f64;
    println!("exec time serial:   {}s", exec_time/1000000.0);

    let now = Instant::now();
    let _is_inside = pts_in_polygon(
        &points.view(), 
        &polygon.view(), 
        true,
        true
    );
    let exec_time = now.elapsed().as_micros() as f64;
    println!("exec time parallel: {}s", exec_time/1000000.0);

}