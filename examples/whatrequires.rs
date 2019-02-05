use solv::{Pool, Repository};
use std::path::Path;

fn main() {
    let mut pool = Pool::new();
    pool.set_arch("x86_64").expect("set repo arch");

    let mut updates = Repository::new(&mut pool, "updates").expect("create repository");

    updates
        .set_solv_file(Path::new("/var/cache/dnf/updates.solv"), None)
        .expect("set .solv file");
}
