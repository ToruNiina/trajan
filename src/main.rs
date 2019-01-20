fn main() {
    let reader = trajanus::xyz::open_pos::<f64>("example.xyz")
        .expect("opening xyz file");

    for snapshot in reader {
        for particle in snapshot.particles {
            println!("{:?}", particle);
        }
    }
}
