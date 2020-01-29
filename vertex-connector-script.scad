aal = [ [-0.2, 0.1], [-0.2,0.3], [-0.2,0.6], [-0.2,1.5], [-0.2,2.6], [-0.3,3.6] ];

fill_depth = 1;
module end_params() { }

RAD=180.0/PI;
SECSZ=80;
member_w = 2.38125; // 3/32"
leg_w = 1.6*member_w;
leg_off = 4.5;
leg_len = 10;


module vertex_hull(aas) {
    intersection_for(aa = aas) {
        rotate(a=[0,-aa[0]*RAD,aa[1]*RAD])
        translate([0,0,-SECSZ/2])
        cube(size=[SECSZ,SECSZ,SECSZ],center=true);
    }
}

function depth_for_aa(aa) = leg_len * sin(-aa[0] * RAD) + leg_w * cos(-aa[0]*RAD);

module connector(aas) {
    d = max([for (x = aas) depth_for_aa(x) ]);
    translate([0,0,d])
    intersection() {
        translate([0,0,-0.00001]) vertex_hull(aas);
        for (aa = aas) {
            intersection() {
            rotate( a = [0, -aa[0]*RAD, aa[1]*RAD] )
                children();
            rotate( a = [0, 0, aa[1]*RAD] )
            translate([SECSZ/2,0,0])
            cube(size=[SECSZ,SECSZ,SECSZ],center=true);
        } }
        if (fill_depth) {
            translate([0,0,-d/2]) cube(size=[SECSZ,SECSZ,d],center=true);
        }
    }
}

module balsa_leg() {
    difference() {
        if (fill_depth == 1) {
            translate([0,-leg_w/2,-40]) cube(size=[leg_len,leg_w,40]); 
        } else {
            translate([0,-leg_w/2,-leg_w]) cube(size=[leg_len,leg_w,leg_w]); 
        }
        translate([leg_off,-member_w/2,-member_w]) cube(size=[leg_len,member_w,member_w+2]);
    }
}

module emboss_text(txt) {
    difference() { 
        children();
        rotate([0,180,0])
        translate([0,0,-0.2])
        linear_extrude(0.3) {
            text(txt, size=leg_w, font="Cabin", halign="center", valign="center");
            translate([leg_w,0,0]) square([leg_w,0.2],center=true);
            translate([-leg_w,0,0]) square([leg_w,0.2],center=true);
        }
    }
}
//connector( aal ) balsa_leg();
emboss_text("1") connector( aal ) balsa_leg();
