use crate::{base::node::LueNode, obj_impl, obj_type, using::*};
use crate::base::obj::ObjImpl;

pub struct LueSystem {
    obj_self: obj_type!(Self with RcWeak),
}
obj_impl!(LueSystem with Rc);

impl LueSystem {
    fn attach_node(
        node: &Rc<LueNode>,
        upper: &Rc<LueNode>,
        index: Option<usize>,
    ) -> Result<(), ()> {
        // allow mutable
        let upper_mut = unsafe { transmute::<_, &mut LueNode>(upper.as_ref()) };
        let node_mut = unsafe { transmute::<_, &mut LueNode>(node.as_ref()) };

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
    fn detach_node(node: &Rc<LueNode>) -> Result<(), ()> {
        // allow mutable
        let node_raw = unsafe { transmute::<_, &mut LueNode>(node.as_ref()) };

        // get upper node first
        let upper = node.upper.as_ref();

        // check if lower contains upper
        if upper.is_none() {
            return Err(());
        }
        let mut upper = upper.unwrap();
        let mut upper_raw = unsafe { node.upper_raw.as_mut().unwrap() };

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
