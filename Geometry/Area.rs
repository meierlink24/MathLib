fn square_area(side: f64) {
    let area = side * side;
    println!("Area of Square is: {}", area);
}

fn rectangle_area(a: f64, b: f64)  {
    let area = a * b;
    println!("Area of Rectangle is: {}", area);
}

fn circle_area(r: f64)  {
    const Pi: f64 = 3.14;
    let area = Pi * r * r;
    println!("Area ot the circle is: {}", area);
}



