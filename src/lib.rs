// Temporarily enabled
#![allow(unused)]

// Permanently enabled
#![allow(mutable_transmutes)]

pub(crate) mod base;
pub(crate) mod using;

pub mod prelude;

mod todo;

fn test() {
    use prelude::*;
    use using::*;
}
