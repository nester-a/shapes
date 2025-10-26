fn main() {
    let mut shape_geom = Geometry::new(Shape::Circle(2.0));
    println!("calc {:?}", shape_geom);
    println!("is_valid: {}; area: {}", shape_geom.is_valid(), shape_geom.area());

    shape_geom = Geometry::new(Shape::Rectangle(1.1, 2.2));
    println!("calc {:?}", shape_geom);
    println!("is_valid: {}; area: {}", shape_geom.is_valid(), shape_geom.area());

    println!("error demonstration");
    shape_geom = Geometry::new(Shape::Circle(-1.1));
    println!("calc {:?}", shape_geom);
    println!("is_valid: {}; area: {}", shape_geom.is_valid(), shape_geom.area());
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

#[derive(Debug)]
struct Geometry {
    shape: Shape,
}

impl Geometry {
    fn new(shape: Shape) -> Geometry {
        Geometry { shape }
    }

    fn area(&self) -> f64 {
        if self.is_valid() == false {
            return f64::NAN;
        }

        match self.shape {
            Shape::Circle(r) => 3.14 * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                let p = (a + b + c) / 2.0;
                p * (p - a) * (p - b) * (p - c).sqrt()
            }
        }
    }

    fn is_valid(&self) -> bool {
        match self.shape {
            Shape::Circle(rad) => rad > 0.0,
            Shape::Rectangle(w,h ) => w > 0.0 && h > 0.0,
            Shape::Triangle(a, b , c ) => a > 0.0 && b > 0.0 && c > 0.0
        }
    }
}
