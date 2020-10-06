use ndarray::prelude::*;
use inpolygon::pt_in_polygon;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

pub fn main() {
    // let a = 3.0;
    // let b = 4.0;
    
    // let c = sum(a, b);
    // println!("result: {}", c);

    // let a = array![
    //     [1.,2.,3.], 
    //     [4.,5.,6.],
    // ]; 
    // assert_eq!(a.ndim(), 2);         // get the number of dimensions of array a
    // assert_eq!(a.len(), 6);          // get the number of elements in array a
    // assert_eq!(a.shape(), [2, 3]);   // get the shape of array a
    // assert_eq!(a.is_empty(), false); // check if the array has zero elements

    // println!("{:?}", a);

    // let a = Array::<f64, _>::zeros((3, 2));

    // let  a = array![1., 2., 6., 4., 5.];
    // let mut counters = Array::<u32, _>::zeros(a.len());
    // let bidx = a.map(|x| *x >= 5.0);
    // println!("bidx: {}", bidx);
    // let bidx1 = a.map(|x| *x >= 4.0);
    // println!("bidx1: {}", bidx1);
    // let bidx2 = bidx & bidx1;
    // println!("bidx2: {}", bidx2);
    // counters[bidx2] += 1;
    // counters = bidx.map(|x| if x==true {})

    // for (i,b) in bidx.iter().enumerate() {
    //     if *b==true { counters[i] += 1};
    // }
    // println!("counters: {}", counters);

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
    //     (1000000, 2), 
    //     Uniform::new(-2., 2.)
    // );


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