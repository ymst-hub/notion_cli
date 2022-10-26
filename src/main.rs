mod file_local;
fn main() {
    file_local::create_local_env();
    for a in file_local::read_local_env() {
        println!("{}", a);
    }
}
