// ax^2 + bx + c = 0

fn quadratic_equation(a: f64, b: f64, c: f64) {

    let d = b*b - 4.0 * a * c;
    let sqrt_d = d.sqrt();
    
    if d > 0.0 {
        let x1 = (-b + sqrt_d) / (2.0 * a);
        let x2 = (-b - sqrt_d) / (2.0 * a);
    
        println!("The roots are real and distinct: x1: {} and x2: {}", x1, x2);
    }else if d == 0.0 {
        let x = -b / (2.0 * a);
        println!("The roots are real and equal: x {}", x);
    
    } else if d < 0.0 {
        println!("The roots are imaginary.");
    }
    
    
    
    
    }
    
    fn main() {
    
    quadratic_equation(1.0,-6.0, -16.0);
    }