pub(crate) use std::ffi::c_void;
pub(crate) use std::mem::transmute;
pub(crate) use std::ptr::{null, null_mut};

pub(crate) use std::any::{Any, TypeId};

pub(crate) use std::rc::Rc;
pub(crate) type RcWeak<T> = std::rc::Weak<T>;

pub(crate) use std::sync::Arc;
pub(crate) type ArcWeak<T> = std::sync::Weak<T>;

pub(crate) use std::cell::OnceCell;
