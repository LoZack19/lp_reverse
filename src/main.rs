use lp_reverse::lp_file::LPFile;

fn main() {
    let lp = LPFile::new("./data/part000.lp");
    lp.foo().unwrap();
}
