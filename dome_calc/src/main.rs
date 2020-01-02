extern crate cairo;

use cairo::{ SvgSurface, Format, Context };

struct Point { x : f64, y : f64, z : f64 }

struct Edge { a : usize, b : usize }

struct Panel {
    n : usize,
    p : Vec<Point>,
    e : Vec<Edge>
}

impl Point {
    fn project( &mut self, r : f64 ) {
        let origin = Point{ x: 0.0, y : 0.0, z : 0.0 };
        // Check for point at origin?
        let factor = r / distance( &origin, self );
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

fn distance( a : &Point, b : &Point ) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    ((dx*dx)+(dy*dy)+(dz*dz)).sqrt()
}

impl Panel {
    fn build(freq : usize) -> Panel {
        // frequency 0 is a triangle; freq 1 is edges broken in one place, etc.
        let n = (freq+2)*(freq+3) / 2;
        let mut panel = Panel{ n : n, p : Vec::new(), e : Vec::new() };
        for i in 0..freq+2 { // row
            for j in 0..i+1 { // point in row
                let leftmost = -(i as f64) / 2.0;
                panel.p.push( Point{ 
                    x : leftmost + j as f64,
                    y : i as f64,
                    z : 1.0 } );
            }
        }
        for i in 0..freq+1 {
            let a_row_idx = i*(i+1) / 2;
            let b_row_idx = (i+1)*(i+2) / 2;
            for j in 0..i+1 {
                let a_idx = a_row_idx+j;
                let b_idx = b_row_idx+j;
                panel.e.push( Edge{ a: a_idx, b: b_idx } );
                panel.e.push( Edge{ a: a_idx, b: b_idx+1 } );
                panel.e.push( Edge{ a: b_idx, b: b_idx+1 } );
            }
        }
        panel
    }
}

fn main() {
    let surface =  SvgSurface::new(600.0,600.0,Some("test.svg")).expect("Couldn't create svgsurface");
    let context = Context::new(&surface);
    let panel = Panel::build(4);
    context.set_source_rgb(0.5,0.5,0.5);
    context.paint();
    context.set_source_rgb(0.0,0.0,0.0);
    for e in panel.e {
        let (a,b) = (&panel.p[e.a], &panel.p[e.b]);
        context.move_to(300.0 + a.x*30.0,a.y*30.0);
        context.line_to(300.0 + b.x*30.0,b.y*30.0);
    }
    context.stroke();
    println!("Hello, world!");
}
