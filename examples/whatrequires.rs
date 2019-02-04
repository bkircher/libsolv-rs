use solv::{Pool, Repository};

fn main() {
    let mut pool = Pool::new();
    let repo = Repository::new(&mut pool, "src");
}
