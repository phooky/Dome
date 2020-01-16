// We'll use celestial notation. All angles are in degrees.
test_aas = [ [0.1, 0.1], [0.1,0.2], [0.1,0.3], [-0.2,0.6] ];
RAD=360.0/PI;

module make_legs( alt_azims ) {
    for ( aa = alt_azims ) {
        alt = aa[0];
        azim = aa[1];
        rotate( a = [0, alt*RAD, azim*RAD]) children();
    }
}

make_legs( test_aas ) cube(size=[20,2,2]);
