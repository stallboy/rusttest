mod types;
mod ownerships;
mod references;
mod slices;
mod structs;
mod enums;
mod modules;
mod mods;
mod vecs;
mod strs;
mod hashmaps;
mod errors;
mod traits;
mod lifetimes;


fn main() {
    types::test();
    ownerships::test();
    references::test();
    slices::test();
    structs::test();
    enums::test();
    modules::test();
    mods::test();
    vecs::test();
    strs::test();
    hashmaps::test();
    errors::test();
    traits::test();
    lifetimes::test();
}
