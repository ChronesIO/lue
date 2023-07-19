// Temporarly enabled
#![allow(unused)]

pub(crate) mod base;
pub(crate) mod using;

pub mod prelude;

fn test() {
    use prelude::*;
    use using::*;

    let node = base::node::LueNode {
        obj_self: OnceCell::new(),

        upper: None,
        upper_raw: null_mut(),

        lower: vec![],
        lower_raw: vec![],

        layout: OnceCell::new(),
        layout_raw: OnceCell::new(),
        layout_t2a: OnceCell::new(),

        system: todo!(),
    };
}
