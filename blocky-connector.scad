// We'll use celestial notation. All angles are in degrees.
azimuth = 72;
altitude = -31.7;

// Assuming the member has a square cross section, one
// dimension in mm.
member_w = 2.38125; // 3/32"
peg_d = 0.3;
leg_off = member_w*2;
leg_l = 10;
center_hole_d = 1.5; // 1.5mm for eyehook?
truncation = 0.6; // truncation from apex

module member_slot_neg() {
    difference() {
        intersection() {
            translate([-member_w/2,0,0])
            cube(size=[member_w,100,member_w*2]);
            translate([0,0,member_w])
            rotate(-30, v=[1,0,0])
            translate([-member_w,0,-50])
            cube(size=[member_w*2,100,100]);
        }
    }
}

module leg(length,off,back) {
    difference() {
        translate([-member_w,-back,-member_w/2])
        cube(size=[member_w*2,length+off+back,member_w*1.5]);
        translate([0,off,0])
        member_slot_neg();
    }
}

module pyramid() {
    intersection() {
        intersection_for (az = [0 : azimuth : 359] ) {
            rotate(az)
            rotate(altitude,v=[1,0,0])
            translate([-50,-100,-100])
            cube(size=[100,200,member_w+100]);
        }
        difference() {
            translate([-100,-100,-100])
            cube(size=[200,200,member_w+100-truncation]);
            cylinder(d=center_hole_d,center=true,h=200, $fn=20);
        }
    }
}

module connector(azimuth,altitude) {
    for (az = [0 : azimuth : 359] ) {
        intersection() {
            pyramid();
            rotate(az)
            rotate(altitude,v=[1,0,0])
            leg(leg_l,leg_off,20);
        }
    }
}


connector(azimuth,altitude);
