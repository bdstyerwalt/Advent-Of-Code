pub fn shoelace_formula(permiter: &Vec<(i32, i32)>) -> i32 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut det: i32 = 0;
    for i in 0..&permiter.len()-1 {
        let (x1, y1) = permiter[i];
        let (x2, y2) = permiter[i+1];

        det += (x1*y2)-(x2*y1);
    }
    let area = det/2;
    return area;
}

pub fn picks_therom(num_points: i32, area: i32) -> i32 {
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = i + b/2 - 1
    // Where:
    // A is area of the polygon
    // i is the interior number of points
    // b is the number of intger points on the polygon boundary (all points not just corners)

    // find i: i = A - b/2 + 1
    let b = (num_points) as i32;
    println!("number of points on boundary: {b}");
    let interior_points = area - (b/2) + 1;

    return interior_points;
}