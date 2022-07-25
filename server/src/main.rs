mod messages;
mod security;

use std::time::Instant;

use security::identity;
fn main() {
    let now = Instant::now();
    for _ in 0..500 {
        let _id = identity::new();
    }
    println!("{:?}", now.elapsed());
}
