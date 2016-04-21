extern crate gcc;

fn main() {
    gcc::Config::new().file("src/ip.c").compile("libip.a");
}