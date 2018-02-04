#[macro_use]
extern crate serde_derive;
extern crate toml;

mod profile;

use profile::Profile;

fn main() {
    let profile = Profile::read_from_file("profile.toml");

    println!("{:#?}", profile);
}
