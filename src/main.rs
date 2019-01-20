fn main() {
    let mut reader = trajanus::xyz::open("example.xyz").expect("open xyz file");
    let xyz_snapshot = reader.read_snapshot::<f64>().expect("read xyz snapshot");

    for particle in xyz_snapshot.iter() {
        println!("{:?}", particle);
    }
}
