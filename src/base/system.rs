use crate::base::obj::ObjImpl;
use crate::{base::node::LueNode, obj_impl, obj_type, using::*};

pub struct LueSystem {
    obj_self: obj_type!(Self with RcWeak),
}
obj_impl!(LueSystem with Rc);

impl LueSystem {
    fn attach_node(
        node: &Rc<UnsafeCell<LueNode>>,
        upper: &Rc<UnsafeCell<LueNode>>,
        index: Option<usize>,
    ) -> Result<(), ()> {
        // allow mutable
        let upper_mut = unsafe { &mut *upper.get() };
        let node_mut = unsafe { &mut *upper.get() };

        // attach upper to lower first
        if upper_mut._attach_node(node, index).is_err() {
            return Err(());
        }

        // attach lower to upper
        if node_mut._attach_self(upper).is_err() {
            // revert changes
            upper_mut._detach_node(node);
            return Err(());
        }

        // fire events
        upper_mut._on_attached_node(node);
        node_mut._on_attached_self(upper);

        Ok(())
    }
    fn detach_node(node: &Rc<UnsafeCell<LueNode>>) -> Result<(), ()> {
        // allow mutable
        let node_raw = unsafe { node.get().as_mut().unwrap() };

        // get upper node first
        let upper = unsafe { node.get().as_ref().unwrap().upper.as_ref() };

        // check if lower contains upper
        if upper.is_none() {
            return Err(());
        }
        let mut upper = upper.unwrap();
        let mut upper_raw = unsafe { node.get().as_ref().unwrap().upper_raw.as_mut().unwrap() };

        // detach at upper
        if upper_raw._detach_node(node).is_err() {
            return Err(());
        }

        // detach at lower
        if node_raw._detach_self().is_err() {
            // this should never happen
            panic!("Could not detach node from parent: `node._detach_self` failed");
        }

        upper_raw._on_detached_node(node);
        node_raw._on_detached_self(&upper);

        Ok(())
    }

    fn add_context(context: Box<dyn Any>) {}
    fn remove_context(context: Box<dyn Any>) {}
}
