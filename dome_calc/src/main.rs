extern crate cairo;

use cairo::{ SvgSurface, Format, Context };


// Icosahedron geometry, all relative to edge length
const ICO_0R : f64 = 0.9510565; // circum-sphere
const ICO_1R : f64 = 0.8091699; // mid-sphere
const ICO_2R : f64 = 0.7557613; // in-sphere

const SQRT_3 : f64 = 1.7320508075688772;
const TRI_H : f64 = SQRT_3/2.0; // height of triangle with unit edge

const SVG_WIDTH : f64 = 200.0;
const SVG_HEIGHT : f64 = 200.0;

struct Point { x : f64, y : f64, z : f64 }

struct Edge { a : usize, b : usize }

struct Panel {
    n : usize,
    p : Vec<Point>,
    e : Vec<Edge>
}

impl Point {
    fn project_to_radius( &mut self, r : f64 ) {
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
        let edges : f64 = (freq + 1) as f64;
        let x0 : f64 = 0.0;
        let y0 : f64 = -(TRI_H*2.0/3.0);
        let rowht = TRI_H / edges;
        let edgelen = 1.0 / edges;
        for i in 0..freq+2 { // row
            let leftmost = x0 - (i as f64)*edgelen/2.0;
            for j in 0..i+1 { // point in row
                panel.p.push( Point{ 
                    x : leftmost + edgelen * (j as f64),
                    y : y0 + rowht * (i as f64),
                    z : ICO_2R } );
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
    let surface =  SvgSurface::new(SVG_WIDTH,SVG_HEIGHT,Some("test.svg")).expect("Couldn't create svgsurface");
    let context = Context::new(&surface);
    let mut panel = Panel::build(4);
    context.set_source_rgb(0.5,0.5,0.5);
    context.paint();
    context.set_source_rgb(0.0,0.0,0.0);
    for e in &panel.e {
        let (a,b) = (&panel.p[e.a], &panel.p[e.b]);
        context.move_to(SVG_WIDTH/2.0 + a.x*100.0, SVG_HEIGHT/2.0 + a.y*100.0);
        context.line_to(SVG_WIDTH/2.0 + b.x*100.0, SVG_HEIGHT/2.0 + b.y*100.0);
    }
    for p in &mut panel.p { p.project_to_radius(ICO_0R); }
    for e in panel.e {
        let (a,b) = (&panel.p[e.a], &panel.p[e.b]);
        context.move_to(SVG_WIDTH/2.0 + a.x*100.0, SVG_HEIGHT/2.0 + a.y*100.0);
        context.line_to(SVG_WIDTH/2.0 + b.x*100.0, SVG_HEIGHT/2.0 + b.y*100.0);
    }
    context.stroke();
    println!("Hello, world!");
}
