// We'll use celestial notation. All angles are in degrees.
test_aas = [ [0.1, 0.1], [0.1,0.2], [0.1,0.3], [-0.2,0.6] ];
RAD=360.0/PI;

function avg_angles(a1,a2) {
    // find the two angles that are on the mean line and
    // pick the closer
    mean1 = a1+a2/2;
    mean2 = mean1+PI;
    if ( mean2 > PI ) { mean2 = mean2 - (2*PI); }
    return 0.0; // TODO
}
        
    
// Assumption: that the alt_azims are sorted by azimuth
module make_legs( alt_azims ) {
    l = len(alt_azims);
    for ( idx = [0:l-1] ) {
        aa = alt_azims[idx];
        prior_azim = alt_azims[(idx-1) % l][1];
        next_azim = alt_azims[(idx+1) % l][1];
        alt = aa[0];
        azim = aa[1];
        cut_l = azim+prior_azim
        echo(azim);
        rotate( a = [0, alt*RAD, azim*RAD]) children();
    }
}

make_legs( test_aas ) cube(size=[20,2,2]);
