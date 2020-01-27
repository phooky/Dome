extern crate cairo;
extern crate cgmath;

use cairo::{ SvgSurface, Context };
use cgmath::prelude::*;
use cgmath::{ Point3, Vector3, Matrix3, Matrix4, Rad };

// Icosahedron geometry, all relative to edge length
const ICO_0R : f64 = 0.9510565; // circum-sphere
const ICO_1R : f64 = 0.8091699; // mid-sphere
const ICO_2R : f64 = 0.7557613; // in-sphere

const SQRT_3 : f64 = 1.7320508075688772;
const TRI_H : f64 = SQRT_3/2.0; // height of triangle with unit edge
use std::f64::consts::PI;

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
    annotation : Option<u8>,
}

struct Edge { 
    a : usize, 
    b : usize,
}

struct Panel {
    p : Vec<Point>,
    e : Vec<Edge>
}

fn to_alt_azim(v : &Vector3<f64>) -> (f64,f64) {
    let altitude = v.z.asin();
    let azimuth = v.y.atan2(v.x);
    (altitude, azimuth)
}

fn adjust_azimuth( (altitude, azimuth) : (f64, f64), rad : f64) -> (f64, f64) {
    let mut azimuth = azimuth + rad;
    if azimuth > PI { azimuth -= 2.0*PI; }
    (altitude, azimuth)
}

impl Panel {
    fn build(freq : usize) -> Panel {
        // frequency 1 is a triangle; freq 2 is edges broken in one place, etc.
        // let n = (freq+1)*(freq+2) / 2;
        let mut panel = Panel{ p : Vec::new(), e : Vec::new() };
        let x0 : f64 = 0.0;
        let y0 : f64 = -(TRI_H*2.0/3.0);
        let rowht = TRI_H / freq as f64;
        let edgelen = 1.0 / freq as f64;

        // Create points on panel
        let mut next_anno = 0;
        for i in 0..freq+1 { // row
            let leftmost = x0 - (i as f64)*edgelen/2.0;
            for j in 0..i+1 { // point in row
                let anno = if j <= i/2 { 
                    if (2*i-j) <= freq {
                        let a = next_anno;
                        next_anno += 1;
                        Some(a) 
                    } else { None }
                } else { None };
                use PointType::*;
                panel.p.push( Point{ t : if j == 0 || j == i {
                    if i == 0 || i == freq { Corner } else { Edge }
                } else {
                    if i == freq { Edge } else { Ordinary }
                },
                p : Point3 { 
                    x : leftmost + edgelen * (j as f64),
                    y : y0 + rowht * (i as f64),
                    z : ICO_2R },
                annotation : anno } );
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

    fn star(&self, idx : usize) -> Vec<(f64,f64)> {
        use PointType::*;
        let mut cv = Vec::new();
        let p = &self.p[idx].p;
        let t = &self.p[idx].t;
        // p is a unit vector; we want a matrix that rotates is onto
        // the Z axis.
        let rot_mat = Matrix3::look_at(p.to_vec(),Vector3::unit_z());

        // Find all edges
        for e in &self.e {
            if e.a != idx && e.b != idx { continue; }
            let other = 
                if e.a == idx { &self.p[e.b].p } else { &self.p[e.a].p };
            let v = other - p;
            let aa = to_alt_azim(&(rot_mat*v));
            cv.push(aa);
            // Add rotated replicas for corners and edges
            match t {
                Ordinary => {},
                Edge => { 
                    //println!("Adding mirror around {}", cv[0].1);
                    // x' = -(x-a)+a = 2a-x
                    let mut new_azim = 2.0*cv[0].1-aa.1;
                    if new_azim > PI { new_azim -= 2.0*PI; }
                    if new_azim < -PI { new_azim += 2.0*PI; }
                    cv.push((aa.0,new_azim)); },
                Corner => { for i in 1..5 {
                    //println!("Adding corner {}",i as f64*PI*(2.0/5.0));
                    cv.push(adjust_azimuth(aa,i as f64*PI*(2.0/5.0))); } },
            }
        }

        use std::cmp::Ordering::Equal;
        cv.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(a.0.partial_cmp(&b.0).unwrap_or(Equal)));
        let mut deduped = Vec::new();
        let mut last_azim = -100.0;
        for (alt, azim) in cv {
            if (azim-last_azim).abs() > 0.0000001 { deduped.push( (alt,azim) ); }
            last_azim = azim;
        }
        deduped
    }

    fn render(&self, context : &Context, m : &Matrix4<f64>) {
        context.save();
        context.select_font_face("serif",cairo::FontSlant::Italic,cairo::FontWeight::Normal);
        context.set_font_size(12.0);
        context.set_line_cap(cairo::LineCap::Round);
        context.set_line_width(1.0);
        for e in &self.e {
            let a = m.transform_point(self.p[e.a].p);
            let b = m.transform_point(self.p[e.b].p);
            context.move_to(a.x, a.y);
            context.line_to(b.x, b.y);
        }
        context.stroke();
        context.set_line_width(0.8);
        for p in &self.p {
            let pp = m.transform_point(p.p);
            use PointType::*;
            match p.annotation {
                None => {},
                Some(_) => {
                    context.set_source_rgb(1.0,1.0,1.0);
                },
            }

            context.arc(pp.x, pp.y, 1.2, 0.0, 2.0*PI);
            context.fill();
            match p.t {
                Ordinary => { },
                Corner => {
                    context.new_sub_path();
                    context.arc(pp.x, pp.y, 2.4, 0.0, 2.0*PI);
                    context.new_sub_path();
                    context.arc(pp.x, pp.y, 4.2, 0.0, 2.0*PI);
                    context.stroke();
                },
                Edge => {
                    context.rectangle(pp.x-2.5,pp.y-2.5,5.0,5.0);
                    context.stroke();
                },
            }
            context.set_source_rgb(0.0,0.0,0.0);
            context.move_to(pp.x-12.0,pp.y-5.0);
            match p.annotation {
                Some(a) => {
                    let mut tmp = [0; 4];
                    let txt = ((0x61 + a) as char).encode_utf8(&mut tmp);
                    context.show_text( txt );
                },
                None => {},
            }
        }
        context.stroke();
        context.restore();
    }

}

fn main() {
    let surface =  SvgSurface::new(SVG_WIDTH,SVG_HEIGHT,Some("test.svg")).expect("Couldn't create svgsurface");
    let context = Context::new(&surface);
    let mut panel = Panel::build(2);
    context.set_source_rgb(0.6,0.6,0.6);
    context.paint();
    context.set_source_rgb(0.0,0.0,0.0);
    let rotm = Matrix4::from_angle_x(Rad(PI/4.0)) * Matrix4::from_angle_z(Rad(PI/8.4));
    let scrm = 
        Matrix4::from_translation( Vector3 { x : SVG_WIDTH/2.0, y : SVG_HEIGHT/2.0, z : 0.0 } )
        * Matrix4::from_scale(150.0);
    let t = scrm * Matrix4::from_translation( Vector3 { x:0.0, y:0.8, z:0.0 }) * rotm;
    panel.render(&context, &t);
    for p in &mut panel.p { p.p = Point3::from_vec(p.p.to_vec().normalize_to(ICO_0R)); }
    let t = scrm * Matrix4::from_translation( Vector3 { x:0.0, y:0.3, z:0.0 }) * rotm;
    panel.render(&context, &t);
    let mut stars = Vec::new();
    for i in 0..2 {
        let data = panel.star(i).iter().map(|x| format!("[{}, {}]",x.0,x.1))
            .collect::<Vec<_>>().join(", ");
        stars.push( format!("[{}]", data) );
    }
    println!(r#"
star=0;
use <vertex-connector-script.scad>
stars=[{}];
connector(stars[star]) balsa_leg();
"#, stars.join(", "));

}
