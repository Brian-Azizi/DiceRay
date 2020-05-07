#[derive(Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}
fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}
fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

fn main() {
    let a = tuple(4.3, -4.2, 3.1, 1.0);
    let b = tuple(4.3, -4.2, 3.1, 0.0);
    let p = point(4., -4., 3.);
    let v = vector(4., -4., 3.);
    println!("{:#?}", a);
    println!("{:#?}", b);
    println!("{:#?}", p);
    println!("{:#?}", v);
}
