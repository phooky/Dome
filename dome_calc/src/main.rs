extern crate cairo;
extern crate cgmath;

use cairo::{ SvgSurface, Format, Context };
use cgmath::prelude::*;
use cgmath::{ Point3, Vector3, Matrix4 };

// Icosahedron geometry, all relative to edge length
const ICO_0R : f64 = 0.9510565; // circum-sphere
const ICO_1R : f64 = 0.8091699; // mid-sphere
const ICO_2R : f64 = 0.7557613; // in-sphere

const SQRT_3 : f64 = 1.7320508075688772;
const TRI_H : f64 = SQRT_3/2.0; // height of triangle with unit edge

const SVG_WIDTH : f64 = 300.0;
const SVG_HEIGHT : f64 = 300.0;

enum PointType {
    Ordinary,
    Corner,
    Edge,
}

struct Point {
    p : Point3<f64>,
    t : PointType,
}

struct Edge { a : usize, b : usize }

struct Panel {
    n : usize,
    p : Vec<Point>,
    e : Vec<Edge>
}

impl Panel {
    fn build(freq : usize) -> Panel {
        // frequency 1 is a triangle; freq 2 is edges broken in one place, etc.
        let n = (freq+1)*(freq+2) / 2;
        let mut panel = Panel{ n : n, p : Vec::new(), e : Vec::new() };
        let x0 : f64 = 0.0;
        let y0 : f64 = -(TRI_H*2.0/3.0);
        let rowht = TRI_H / freq as f64;
        let edgelen = 1.0 / freq as f64;

        // Create points on panel
        for i in 0..freq+1 { // row
            let leftmost = x0 - (i as f64)*edgelen/2.0;
            for j in 0..i+1 { // point in row
                panel.p.push( Point{ t : if j == 0 || j == i {
                    if i == 0 || i == freq {
                        PointType::Corner } else {
                        PointType::Edge }
                } else {
                    PointType::Ordinary 
                },
                p : Point3 { 
                    x : leftmost + edgelen * (j as f64),
                    y : y0 + rowht * (i as f64),
                    z : ICO_2R } } );
            }
        }

        // Add edges
        for i in 0..freq {
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

    fn render(&self, context : &Context, m : &Matrix4<f64>) {
        context.save();
        context.set_line_cap(cairo::LineCap::Round);
        context.set_line_width(1.5);
        for e in &self.e {
            let a = m.transform_point(self.p[e.a].p);
            let b = m.transform_point(self.p[e.b].p);
            context.move_to(a.x, a.y);
            context.line_to(b.x, b.y);
        }
        context.stroke();
        context.restore();
    }

}

fn main() {
    let surface =  SvgSurface::new(SVG_WIDTH,SVG_HEIGHT,Some("test.svg")).expect("Couldn't create svgsurface");
    let context = Context::new(&surface);
    let mut panel = Panel::build(4);
    context.set_source_rgb(0.5,0.5,0.5);
    context.paint();
    context.set_source_rgb(0.0,0.0,0.0);
    let mut t = 
        Matrix4::from_translation( Vector3 { x : SVG_WIDTH/2.0, y : SVG_HEIGHT/2.0, z : 0.0 } )
        * Matrix4::from_scale(100.0);

    panel.render(&context, &t);
    for p in &mut panel.p { p.p = Point3::from_vec(p.p.to_vec().normalize_to(ICO_0R)); }
    panel.render(&context, &t);
}
