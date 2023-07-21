use std::{cell::RefCell, ops::Deref};

use crate::{base::node::LueNode, obj_impl, obj_type, using::*};

pub trait LueLayout {
    fn host_ref(&self) -> &LueNode;
    fn host_mut(&mut self) -> &mut LueNode;

    fn taffy_ref(&self) -> &Taffy;
    fn taffy_mut(&self) -> &mut Taffy;
    fn taffy_node(&self) -> &taffy::node::Node;

    fn _on_attached_node(&mut self, node: &Rc<UnsafeCell<LueNode>>);
    fn _on_detached_node(&mut self, node: &Rc<UnsafeCell<LueNode>>);

    fn _on_attached_self(&mut self, node: &Rc<UnsafeCell<LueNode>>);
    fn _on_detached_self(&mut self, node: &Rc<UnsafeCell<LueNode>>);
}

pub struct LueStandardLayout {
    obj_self: obj_type!(Self with RcWeak),

    host: OnceCell<*mut LueNode>,

    taffy: Rc<Taffy>,
    taffy_raw: *mut Taffy,
    taffy_node: taffy::node::Node,
}
obj_impl!(LueStandardLayout with Rc);

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

    fn taffy_ref(&self) -> &Taffy {
        unsafe { self.taffy_raw.as_ref().unwrap_unchecked() }
    }
    fn taffy_mut(&self) -> &mut Taffy {
        unsafe { self.taffy_raw.as_mut().unwrap_unchecked() }
    }

    fn taffy_node(&self) -> &taffy::node::Node {
        &self.taffy_node
    }

    fn _on_attached_node(&mut self, node: &Rc<UnsafeCell<LueNode>>) {
        let taf = self.taffy_mut();

        // add child node to self
        taf.add_child(self.taffy_node.clone(), unsafe {
            node.get()
                .as_ref()
                .unwrap()
                .layout_ref()
                .taffy_node()
                .clone()
        });
    }
    fn _on_detached_node(&mut self, node: &Rc<UnsafeCell<LueNode>>) {
        let taf = self.taffy_mut();

        // remove child node from self
        taf.remove_child(self.taffy_node.clone(), unsafe {
            node.get()
                .as_ref()
                .unwrap()
                .layout_ref()
                .taffy_node()
                .clone()
        });
    }

    fn _on_attached_self(&mut self, node: &Rc<UnsafeCell<LueNode>>) {}
    fn _on_detached_self(&mut self, node: &Rc<UnsafeCell<LueNode>>) {}
}
