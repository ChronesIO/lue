use crate::using::*;

pub struct LueNode {
    obj_self: OnceCell<ArcWeak<Self>>,

    upper: Option<Arc<Self>>,
    upper_raw: *mut Self,

    lower: Vec<Arc<Self>>,
    lower_raw: Vec<*mut Self>,

    system: Arc<dyn Any>, // Todo: Change type to system
}

impl LueNode {
    fn a() {
        let t = TypeId::of::<String>();
    }
}