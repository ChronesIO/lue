use crate::{base::node::LueNode, obj_impl, obj_type, using::*};

pub trait LueLayout {
    fn host_ref(&self) -> &LueNode;
    fn host_mut(&mut self) -> &mut LueNode;

    fn _on_attached_node(&mut self, node: &Arc<LueNode>);
    fn _on_detached_node(&mut self, node: &Arc<LueNode>);

    fn _on_attached_self(&mut self, node: &Arc<LueNode>);
    fn _on_detached_self(&mut self, node: &Arc<LueNode>);
}

pub struct LueStandardLayout {
    obj_self: obj_type!(Self with ArcWeak),

    host: OnceCell<*mut LueNode>,

    taffy: Arc<Taffy>,
}
obj_impl!(LueStandardLayout with Arc);

impl LueLayout for LueStandardLayout {
    fn host_ref(&self) -> &LueNode {
        unsafe {
            self.host
                .get()
                .unwrap_unchecked()
                .as_ref()
                .unwrap_unchecked()
        }
    }
    fn host_mut(&mut self) -> &mut LueNode {
        unsafe {
            self.host
                .get()
                .unwrap_unchecked()
                .as_mut()
                .unwrap_unchecked()
        }
    }

    fn _on_attached_node(&mut self, node: &Arc<LueNode>) {
        todo!()
    }
    fn _on_detached_node(&mut self, node: &Arc<LueNode>) {
        todo!()
    }

    fn _on_attached_self(&mut self, node: &Arc<LueNode>) {
        todo!()
    }
    fn _on_detached_self(&mut self, node: &Arc<LueNode>) {
        todo!()
    }
}
