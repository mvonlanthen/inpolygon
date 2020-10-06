import numpy as np

def points_inside_polygon(points, poly, include_edges=True):
    '''
    Test which of the N points are within the polygon defined by M points. The 
    polygon must be closed, hence the last point must be equal to the first one.

    Usage
    -----
    isin = points_inside_polygon(points, poly, include_edges=True)

    Arguments
    ---------
    points: np.array. points.shape=(N,2)
        List of N points.
    poly: np.array. poly.shape=(M+1,2)
        Corners of the polygon. Closed polygon, hence the first point must be 
        equal to the first point (no check done in the functions!).
    include_edges: bool. default: True

    Returns
    -------
    isin: np.array of bool. isin.shape=(N,)
        True for the points inside (and on the edges) the polygon.
    '''
    xs = points[:,0]
    ys = points[:,1]
    n = len(poly)
    counters = np.zeros(len(xs),int)
    indices = np.arange(len(xs))

    # a little trick to handle points on horizontal edges
    count_on_horz = 1
    if include_edges:
        count_on_horz = 2

    # loop through each edges defined by (p1, p2)
    p1x, p1y = poly[0]
    for i in range(1, n):
        p2x, p2y = poly[i%n]
        if p1y == p2y:
            # test if points are on horizontal edge
            bidx = (ys==p1y) & ((min(p1x, p2x)<xs) & (xs<max(p1x, p2x)))
            counters[bidx] += count_on_horz
        else: # p1y!= p2y
            # select all points will horizonlta right ray crossing the segment and 
            # scompute the intersections
            c_bidx = (ys>=min(p1y, p2y)) & (ys<=max(p1y, p2y))
            idx = indices[c_bidx]
            xinters = (ys[c_bidx] - p1y) * (p2x - p1x) / float(p2y - p1y) + p1x

            # point is right on the edge
            bidx = xs[c_bidx]==xinters
            counters[idx[bidx]] += 1*include_edges

            # point is to the left from current edge
            bidx = xs[c_bidx]<xinters
            counters[idx[bidx]] += 1
        # go to next edge
        p1x, p1y = p2x, p2y

    # if counter odd, then inside polygon, otherwise outside.
    isin = (counters%2).astype(bool)

    # check if some points are corners of the polygon and set to True if 
    # include_edges=True
    xs_bidx = np.in1d(xs, poly[:,0])
    ys_bidx = np.in1d(ys, poly[:,1])
    bidx = xs_bidx & ys_bidx
    isin[bidx] = include_edges

    return isin


# https://stackoverflow.com/questions/36399381/whats-the-fastest-way-of-checking-if-a-point-is-inside-a-polygon-in-python/36400130
def ray_tracing(x,y,poly):
    n = len(poly)
    inside = False
    p2x = 0.0
    p2y = 0.0
    xints = 0.0
    p1x,p1y = poly[0]
    for i in range(n+1):
        p2x,p2y = poly[i % n]
        if y > min(p1y,p2y):
            if y <= max(p1y,p2y):
                if x <= max(p1x,p2x):
                    if p1y != p2y:
                        xints = (y-p1y)*(p2x-p1x)/(p2y-p1y)+p1x
                    if p1x == p2x or x <= xints:
                        inside = not inside
        p1x,p1y = p2x,p2y

    return inside