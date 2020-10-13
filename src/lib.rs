//! A create with functions to compute if a list of points are inside a polygon.
//! This is my first crate. The goal is mostly to learn rust.

use ndarray::prelude::*;
use ndarray::parallel::prelude::*;

/// Compute if a list of points are inside and/or on the edges of a polygon. 
/// It works only with 2D points and polygon. The polygon must be close, i.e. the 
/// last point must be the same than the first point. No test is done to check if 
/// the polygon is really closed!
/// 
/// # Examples
/// ```
/// use ndarray::prelude::*;
/// use inpolygon::pts_in_polygon;
/// let polygon = array![
///     [1.,-1.], [1.,1.], [-0.5,1.], [-1.,0.], [-1.,-1.], [1.,-1.]
/// ];
/// let points = array![
///    [0., 0.], [0.,2.], [2.,0.], [1.,0.], [0.,1.], [-2.,0.], [-0.75,1.], 
///    [-1.,1.], [0.,-1.], [-1.,0.], [-1.,-1.], [1.,-1.], [-1.,-0.5]
/// ];
/// let is_inside = pts_in_polygon(&points.view(), &polygon.view(), true, true);
/// let is_inside_truth = array![
///    true, false, false, true, true, false, false, false, true, true,  true,  true, true
/// ];
/// assert!(is_inside == is_inside_truth);
/// ```
/// 
/// # Panics
///
/// If the second dimension of arguments `points` and/or `polygon` is not equal 
/// to 2 (i.e. 2D points and polygon), the function panics.
pub fn pts_in_polygon(
    points: &ArrayView2<f64>,
    polygon: &ArrayView2<f64>, 
    include_edges: bool,
    parallel: bool
) -> Array1<bool> {
    if points.len_of(Axis(1))!=2 {
        panic!(
            "The dimension of the second axis of `points` must be equal to 2 (i.e 2D points), not {}.", 
            points.len_of(Axis(1))
        );
    }
    if polygon.len_of(Axis(1))!=2 {
        panic!(
            "The dimension of the second axis of `polygon` must be equal to 2 (i.e 2D points), not {}.", 
            polygon.len_of(Axis(1))
        );
    }
    if parallel==true {
        ArrayBase::from_vec(
            points.axis_iter(Axis(0))
                  .into_par_iter()
                  .map(|pt| pt_in_polygon(&pt, &polygon, include_edges))
                  .collect()
        )
    } else {
        points.map_axis(Axis(1), |pt| pt_in_polygon(&pt, &polygon, include_edges))
    }
}

/// Compute if a point is inside and/or on the edges of a polygon. Works only in 
/// 2D. No argument shape/dim check in this function. These checks are done in the 
/// public functions
fn pt_in_polygon( 
    point: &ArrayView1<f64>,
    polygon: &ArrayView2<f64>, 
    include_edges: bool
) -> bool {
    // extract some variable
    let x = &point[0];
    let y = &point[1];
    let nb_poly_pts = polygon.len_of(Axis(0));
    let mut is_inside  = false;
    let mut  x_intersect: f64;

    // loop through each edges defined by (p1, p2). but first, initialize some 
    // variables
    let mut p1x = &polygon[[0,0]];
    let mut p1y = &polygon[[0,1]];
    let mut p2x: &f64;
    let mut p2y: &f64;
    for i in 0..nb_poly_pts-1 {
        p2x = &polygon[[i+1,0]];
        p2y = &polygon[[i+1,1]];
        if p1y==p2y {
            // horizontal edge. Check if the point is on the edge and process 
            // according `include_edges`
            if (*y==*p1y) & ((*x>=p1x.min(*p2x)) & (*x<p1x.max(*p2x))){
                if include_edges==true {return true;} else {return false;}
            }
        } else {
            // check if the right ray from the point intersect with the current edge
            if (*y>=p1y.min(*p2y)) & (*y<p1y.max(*p2y)) {
                x_intersect = (y-p1y) * (p2x-p1x)/(p2y-p1y) + p1x;
                if *x==x_intersect {
                    // check if the point is exactly on the edge and process 
                    //according `include_edges`
                    if include_edges==true {return true;} else {return false;}
                } else if *x<x_intersect {
                    // check if the point is on the left of the current edge
                    is_inside = !is_inside;
                }
            } 
        }
        // go to the next edge
        p1x = p2x;
        p1y = p2y;
    }
    return is_inside;
}


// tests
// -----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pts_in_polygon() {
        let polygon = array![
            [1.,-1.], [1.,1.], [-0.5,1.], [-1.,0.], [-1.,-1.], [1.,-1.],
        ];
        let points = array![
            [0., 0.], [0.,2.], [2.,0.], [1.,0.], [0.,1.], [-2.,0.], [-0.75,1.], 
            [-1.,1.], [0.,-1.], [-1.,0.], [-1.,-1.], [1.,-1.], [-1.,-0.5]
        ];
        let is_inside = pts_in_polygon(
            &points.view(), 
            &polygon.view(), 
            true,
            true
        );
        let is_inside_truth = array![
            true, false, false,  true,  true, false, false, 
            false,  true, true,  true,  true,  true
        ];
        assert!(is_inside == is_inside_truth);
    } 

}


