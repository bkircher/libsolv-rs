use solv::{Pool, Repository};

fn main() {
    let path = std::env::args_os()
        .skip(1)
        .next()
        .expect("usage: whatrequires <solv-file>");

    let mut pool = Pool::new();
    pool.set_arch("x86_64").expect("set repo arch");

    let mut updates = Repository::new(&mut pool, "updates").expect("create repository");

    updates
        .set_solv_file(path.as_ref(), None)
        .expect("set .solv file");
}
