// We'll use celestial notation. All angles are in degrees.
azimuth = 72;
altitude = -31.7;

// Assuming the member has a square cross section, one
// dimension in mm.
member_w = 2.38125; // 3/32"
peg_d = 0.3;
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
        for (y = [4, 6]) for (xf = [1, -1])
            translate([xf*member_w/2,y,0])
            cylinder(h=member_w*5,d=peg_d,center=true,$fn=20);
    }
}

module connector(azimuth,altitude) {
    difference() {
        intersection() {
            translate([-50,-50,0])
            cube([100,100,100]);
            translate([0,0,5.5+member_w])
            intersection_for (az = [0 : azimuth : 359] ) {
                rotate(az)
                rotate(altitude,v=[1,0,0])
                translate([-30,-30,-60])
                cube([60,60,60]);
            }
        }
        for (az = [0 : azimuth : 359]) {
            translate([0,0,5])
            rotate(az)
            rotate(altitude,v=[1,0,0])
            translate([0,member_w,0])
            member_slot_neg();
        }
    }
}

connector(azimuth,altitude);
