use trajanus::coordkind::FileKind;

fn main() {
    let reader = trajanus::xyz::open::<f64>(FileKind::Position, "example.xyz")
        .expect("opening xyz file");

    for snapshot in reader {
        for particle in snapshot.particles {
            println!("{:?}", particle);
        }
    }
}
