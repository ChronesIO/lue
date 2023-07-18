use crate::using::*;

pub trait LueSystem {
    fn attach_node(node: dyn Any, upper: dyn Any);
    fn detach_node(node: dyn Any);
    fn can_attach_node(node: dyn Any, upper: dyn Any);

    fn add_context(context: dyn Any);
    fn remove_context(context: dyn Any);
}
