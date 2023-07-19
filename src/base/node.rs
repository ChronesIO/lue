use std::{error::Error, ops::Deref};

use crate::{base::layout::LueLayout, obj_impl, obj_type, t2a_type_vec, using::*};

pub struct LueNode {
    obj_self: obj_type!(Self with ArcWeak),

    pub upper: Option<Arc<Self>>,
    pub upper_raw: *mut Self,

    pub lower: Vec<Arc<Self>>,
    pub lower_raw: Vec<*mut Self>,

    pub(crate) layout: OnceCell<Arc<dyn LueLayout>>,
    pub(crate) layout_raw: OnceCell<*mut dyn LueLayout>,
    pub(crate) layout_t2a: OnceCell<t2a_type_vec!()>,

    system: Arc<dyn Any>, // Todo: Change type to system
}
obj_impl!(LueNode with Arc);

impl LueNode {
    pub fn has_upper(&self) -> bool {
        self.upper.is_some()
    }

    pub fn has_lower(&self, node: &Arc<Self>) -> bool {
        self.has_lower_ref(node.as_ref())
    }
    pub fn has_lower_ref(&self, node_ref: &Self) -> bool {
        let node_ptr = node_ref as *const _;
        match self.lower_raw.iter().find(|&&x| x as *const _ == node_ptr) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn layout_ref(&self) -> &dyn LueLayout {
        unsafe {
            self.layout_raw
                .get()
                .unwrap_unchecked()
                .as_ref()
                .unwrap_unchecked()
        }
    }
    pub fn layout_mut(&mut self) -> &mut dyn LueLayout {
        unsafe {
            self.layout_raw
                .get()
                .unwrap_unchecked()
                .as_mut()
                .unwrap_unchecked()
        }
    }

    pub fn layout_t2a(&self) -> &t2a_type_vec!() {
        unsafe { self.layout_t2a.get().unwrap_unchecked() }
    }

    pub(crate) fn _attach_node(
        &mut self,
        node: &Arc<Self>,
        index: Option<usize>,
    ) -> Result<(), ()> {
        if self.has_lower(node) {
            return Err(());
        }

        if let Some(s) = index {
            self.lower.insert(s, node.clone());
            self.lower_raw
                .insert(s, unsafe { transmute(node.as_ref() as *const _) });
        } else {
            self.lower.push(node.clone());
            self.lower_raw
                .push(unsafe { transmute(node.as_ref() as *const _) });
        }

        Ok(())
    }
    pub(crate) fn _detach_node(&mut self, node: &Arc<Self>) -> Result<(), ()> {
        let node_ref = node.as_ref() as *const _;
        if let Some(s) = self
            .lower_raw
            .iter()
            .position(|&x| x as *const _ == node_ref)
        {
            self.lower.remove(s);
            self.lower_raw.remove(s);
            return Ok(());
        }
        Err(())
    }

    pub(crate) fn _attach_self(&mut self, node: &Arc<Self>) -> Result<(), ()> {
        if self.upper.is_some() {
            return Err(());
        }

        let node_ref = node.as_ref() as *const _;
        self.upper = Some(node.clone());
        self.upper_raw = unsafe { transmute(node_ref) };

        Ok(())
    }
    pub(crate) fn _detach_self(&mut self) -> Result<(), ()> {
        if self.upper.is_none() {
            return Err(());
        }

        self.upper = None;
        self.upper_raw = null_mut();

        Ok(())
    }

    pub(crate) fn _on_attached_node(&mut self, node: &Arc<Self>) {
        // trigger event in layout
        unsafe {
            self.layout.get().unwrap_unchecked()._on_attached_node(node);
        }
    }
    pub(crate) fn _on_detached_node(&mut self, node: &Arc<Self>) {
        // trigger event in layout
        unsafe {
            self.layout.get().unwrap_unchecked()._on_detached_node(node);
        }
    }

    pub(crate) fn _on_attached_self(&mut self, node: &Arc<Self>) {
        // trigger event in layout
        unsafe {
            self.layout.get().unwrap_unchecked()._on_attached_self(node);
        }
    }
    pub(crate) fn _on_detached_self(&mut self, node: &Arc<Self>) {
        // trigger event in layout
        unsafe {
            self.layout.get().unwrap_unchecked()._on_detached_self(node);
        }
    }
}
