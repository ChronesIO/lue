use crate::{base::node::LueNode, obj_impl, obj_type, using::*};

pub struct LueSystem {
    obj_self: obj_type!(Self with ArcWeak),
}
obj_impl!(LueSystem with Arc);

impl LueSystem {
    fn attach_node(
        node: &Arc<LueNode>,
        upper: &Arc<LueNode>,
        index: Option<usize>,
    ) -> Result<(), ()> {
        // attach upper to lower first
        if upper._attach_node(node, index).is_err() {
            return Err(());
        }

        // attach lower to upper
        if node._attach_self(upper).is_err() {
            // revert changes
            upper._detach_node(node);
            return Err(());
        }

        // fire events
        upper._on_attached_node(node);
        node._on_attached_self(upper);

        Ok(())
    }
    fn detach_node(node: &Arc<LueNode>) -> Result<(), ()> {
        // get upper node first
        let upper = node.as_ref().upper;

        // check if lower contains upper
        if upper.is_none() {
            return Err(());
        }
        
        let mut upper = upper.unwrap();

        // detach at upper
        if upper._detach_node(node).is_err() {
            return Err(());
        }

        // detach at lower
        if node._detach_self().is_err() {
            // this should never happen
            panic!("Could not detach node from parent: `node._detach_self` failed");
        }

        upper._on_detached_node(node);
        node._on_detached_self(&upper);

        Ok(())
    }

    fn add_context(context: Box<dyn Any>) {}
    fn remove_context(context: Box<dyn Any>) {}
}
