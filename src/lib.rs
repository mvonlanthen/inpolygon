//! A create with functions to compute if a list of points aere inside of a polygon.
//! This is my first crate to learn rust and crates

use ndarray::prelude::*;

// Some useful links:
// * ndarray quick tutorial
//  https://github.com/rust-ndarray/ndarray/blob/master/README-quick-start.md


/// Compute if a list of points are inside and/or on the edges of a polygon. 
/// It works only with 2D points and polygon. The polygon must be close, i.e. the 
/// last point must be the same than the first point. To maximize execution speed, 
/// no argument checks are performed (e.g. are the points really 2D?).
/// 
/// # Examples
/// 
/// ```
/// use ndarray::prelude::*;
/// let polygon = array![[-1.,-1.], [1.,-1.], [1.,1.], [-1.,1.],[-1.,-1.]];
/// let points = array![[0., 0.], [2., 0.], [1., 0.], [0., -1.]];
/// let is_inside = pts_in_polygon(&points.view(), &polygon.view(), true);
/// assert!(is_inside == array![true, false, true, true]);
/// ```
pub fn pts_in_polygon(
    points: &ArrayView2<f64>,
    polygon: &ArrayView2<f64>, 
    include_edges: bool
) -> Array1<bool> {
    let mut is_inside = Array::from_elem(points.len_of(Axis(0)), false);
    for (i,point) in points.axis_iter(Axis(0)).enumerate() {
        is_inside[i] = pt_in_polygon(&point.view(), &polygon.view(), include_edges);
    }
    return is_inside;
}

/// Compute if a point is inside and/or on the edges of a polygon. Works only with 
/// 2D point and polygon
pub fn pt_in_polygon( 
    point: &ArrayView1<f64>,
    polygon: &ArrayView2<f64>, 
    include_edges: bool
) -> bool {
    let x = &point[0];
    let y = &point[1];
    let nb_poly_pts = polygon.len_of(Axis(0));
    let mut counter  = 0;
    let mut  x_intersect = 0.0;

    // a little trick to handle a point on horizontal edges
    let count_on_horz = if include_edges==true {2} else {1};

    // loop through each edges defined by (p1, p2). but first, initialize some 
    // variables
    let mut p1x = &polygon[[0,0]];
    let mut p1y = &polygon[[0,1]];
    let mut p2x: &f64;
    let mut p2y: &f64;
    for i in 1..nb_poly_pts {
        p2x = &polygon[[i%nb_poly_pts,0]];
        p2y = &polygon[[i%nb_poly_pts,1]];
        if p1y==p2y {
            // test if the point is on horizontal edge
            if (*y==*p1y) & ((*x>p1x.min(*p2x)) & (*x<p1x.min(*p2x))){
                counter += count_on_horz;
            }
        } else { // p1y!= p2y
            // check if the right ray from the point intersect with the current edge
            if (*y>=p1y.min(*p2y)) & (*y<=p1y.max(*p2y)) {
                x_intersect = (y-p1y) * (p2x-p1x)/(p2y-p1y) + p1x;
            }
            // check if the point is right on the edge
            if (*x==x_intersect) & (include_edges==true) {
                counter += 1;
            // check if the point is on the left of the current edge
            } else if *x<x_intersect {
                counter += 1;
            }
        }

        // go the next edge
        p1x = p2x;
        p1y = p2y;
    }

    // if counter is odd, then the point is inside the polygon, otherwise the 
    // point is outside
    if (counter%2)!=0 {
        return true;
    } else {
        return false;
    }

}


// tests
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_pts_in_polygon() {
        // a cube of side 2, aligned with the coordinate system and centred on [0,0].
        let polygon = array![
            [-1.,-1.], [1.,-1.], [1.,1.], [-1.,1.],[-1.,-1.]
        ];
        let points = array![
            [0., 0.], [2., 0.], [1., 0.], [0., -1.] 
        ];

        let is_inside = pts_in_polygon(&points.view(), &polygon.view(), true);
        assert!(is_inside == array![true, false, true, true]);
    } 

}


