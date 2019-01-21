fn main() {
    let reader = trajan::xyz::XYZReader::open_pos("example.xyz").unwrap().f32();

    for snapshot in reader {
        println!("{:?}", snapshot.which());
        for particle in snapshot.particles {
            println!("{}", particle);
        }
    }
}
