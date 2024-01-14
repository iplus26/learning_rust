#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 1000,  y: 729 };
    let mut r = &mut point;
    let mut rr = &mut r;
    let mut rrr = &mut rr;
    let rrrr = &mut rrr;

    // Modify the value of `point` through the reference `rrrr`
    rrrr.y += 1;

    println!("{:#?}", point)
}
