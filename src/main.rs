mod types;
mod ownerships;
mod references;
mod slices;
mod structs;
mod enums;
mod packages;
mod mods;

fn main() {
    types::test();
    ownerships::test();
    references::test();
    slices::test();
    structs::test();
    enums::test();
    packages::test();
    mods::test();
}
