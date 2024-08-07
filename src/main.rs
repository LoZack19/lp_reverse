use lp_reverse::lp_file::LPFile;
use lp_reverse::lp_file::var_replace::VarReplacer;

fn main() {
    let lp = LPFile::new("./data/part000.lp");
    lp.var_replace(VarReplacer::new(0..10000, "name")).unwrap();
}
