fn main() {
    let reader = trajan::xyz::XYZReader::<f64, _>::open_pos("example.xyz")
        .unwrap();

    for snapshot in reader {
        println!("{:?}", snapshot.which());
        for particle in snapshot.particles {
            println!("{}", particle);
        }
    }
}
