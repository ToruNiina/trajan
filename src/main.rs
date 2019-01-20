fn main() {
    let reader = trajanus::xyz::read_pos::<f64>("example.xyz")
        .expect("opening xyz file");

    for snapshot in reader {
        println!("{:?}", snapshot.which());
        for particle in snapshot.particles {
            println!("{:?}", particle);
        }
    }
}
