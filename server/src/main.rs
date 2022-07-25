mod messages;
mod security;

use security::identity;
fn main() {
    let id = identity::new();
    println!("{}", id.toml());
}
