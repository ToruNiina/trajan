use trajanus::xyz::*;

fn main() {
    let xyz_snapshot = read_xyz_snapshot("example.xyz").expect("read xyz file");

    println!("found {} particles", xyz_snapshot.len());
    for particle in xyz_snapshot.iter() {
        println!("{:?}", particle);
    }
}
