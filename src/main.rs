fn main() {
    let reader = trajan::xyz::XYZReader::open_pos("example.xyz").unwrap().f32();
    let mut writer = trajan::xyz::XYZWriter::new(std::io::stdout());

    for snapshot in reader {
        writer.write_snapshot(&snapshot).expect("write a snapshot into stdout");
    }
}
