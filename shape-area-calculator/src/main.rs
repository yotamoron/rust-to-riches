/*
 * 7. Shape Area Calculator 
 *   Define three structs: Circle, Rectangle, and Triangle, each with the necessary dimensional fields. 
 *   Define a trait called Area with a method calculate_area(&self) -> f64. 
 *   Implement this trait for all three structs, create an instance of each, and print their areas.
 */

 struct Circle {
    radius: f64
 }

 struct Rectangle {
    width: f64,
    height: f64
 }

 struct Triangle {
    base: f64,
    height: f64
 }

 trait Area {
    fn calculate_area(&self) -> f64;
     
 }

 impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
     
 }

 impl Area for Triangle {
    fn calculate_area(&self) -> f64 {
        self.base * self.height / 2.0
    }
 }

 impl Area for Rectangle {
    fn calculate_area(&self) -> f64 {
        self.height * self.width
    }
 }

fn main() {
    let rect = Rectangle { height: 10.0, width: 5.0 };
    let tri = Triangle { base: 10.0, height: 5.0 };
    let cir = Circle { radius: 10.0 };
    println!("{} {} {}", rect.calculate_area(), tri.calculate_area(), cir.calculate_area());
}
