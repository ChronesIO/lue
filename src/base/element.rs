use crate::{base::node::LueNode, using::*};

pub trait LueElement {
    fn host_ref(&self) -> &LueNode;
    fn host_mut(&mut self) -> &mut LueNode;

    fn _on_attached_self(&mut self);
    fn _on_detached_self(&mut self);

    fn _on_attached_node(&mut self, node: &Rc<LueNode>);
    fn _on_detached_node(&mut self, node: &Rc<LueNode>);
}
