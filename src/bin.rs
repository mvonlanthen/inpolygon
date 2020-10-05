use ndarray::prelude::*;
use inpolygon::sum;

pub fn main() {
    let a = 3.0;
    let b = 4.0;
    
    let c = sum(a, b);
    println!("result: {}", c);

    let a = array![
        [1.,2.,3.], 
        [4.,5.,6.],
    ]; 
    assert_eq!(a.ndim(), 2);         // get the number of dimensions of array a
    assert_eq!(a.len(), 6);          // get the number of elements in array a
    assert_eq!(a.shape(), [2, 3]);   // get the shape of array a
    assert_eq!(a.is_empty(), false); // check if the array has zero elements

    println!("{:?}", a);

    let a = Array::<f64, _>::zeros((3, 2));

    let polygon  = array![
        [-1., -1.],
        [ 1., -1.],
        [ 1.,  1.],
        [ 1., -1.],
        [-1., -1.]
    ];

    let points = array![
        [0., 0.],
        [2., 0.]
    ];




}