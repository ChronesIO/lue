use crate::using::*;

macro_rules! _t2a_count_helper {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::base::t2a::_t2a_count_helper!($($xs)*));
}
pub(crate) use _t2a_count_helper;

#[macro_export]
macro_rules! t2a_count {
    ($($trait:ident),* $(,)?) => {
        $crate::base::t2a::_t2a_count_helper!($($trait)*)
    };
}

#[macro_export]
macro_rules! t2a_type {
    ($($trait:ident),* $(,)?) => {
        [(::std::any::TypeId, [u8; ::std::mem::size_of::<*mut dyn ::std::error::Error>()]); $crate::t2a_count!($($trait)*)]
    };
}

#[macro_export]
macro_rules! t2a_type_vec {
    () => {
        ::std::vec::Vec<(::std::any::TypeId, [u8; ::std::mem::size_of::<*mut dyn ::std::error::Error>()])>
    };
}

#[macro_export]
macro_rules! t2a_zero {
    ($($trait:ident),* $(,)?) => {
        [
            $(
                (
                    ::std::any::TypeId::of::<()>(),
                    [0u8; ::std::mem::size_of::<*mut dyn $trait>()]
                )
            ),*
        ] as $crate::t2a_type!($($trait)*)
    };
}

#[macro_export]
macro_rules! t2a_zero_vec {
    ($($trait:ident),* $(,)?) => {
        vec![
            $(
                (
                    ::std::any::TypeId::of::<()>(),
                    [0u8; ::std::mem::size_of::<*mut dyn $trait>()]
                )
            ),*
        ] as $crate::t2a_type_vec!()
    };
}

#[macro_export]
macro_rules! t2a_impl {
    ($struct:ident to [$($trait:ident),* $(,)?]) => {
        [
            $(
                (
                    ::std::any::TypeId::of::<dyn $trait>(),
                    unsafe {
                        let ptr = &mut $struct as &mut dyn $trait;
                        ::std::mem::transmute_copy::<_, [u8; ::std::mem::size_of::<*mut dyn $trait>()]>(&ptr)
                    }
                )
            ),*
        ] as $crate::t2a_type!($($trait)*)
    };
}

#[macro_export]
macro_rules! t2a_impl_vec {
    ($struct:ident to [$($trait:ident),* $(,)?]) => {
        [
            $(
                (
                    ::std::any::TypeId::of::<dyn $trait>(),
                    unsafe {
                        let ptr = &mut $struct as &mut dyn $trait;
                        ::std::mem::transmute_copy::<_, [u8; ::std::mem::size_of::<*mut dyn $trait>()]>(&ptr)
                    }
                )
            ),*
        ] as $crate::t2a_type_vec!()
    };
}

#[macro_export]
macro_rules! t2a_cast_ptr {
    ($table:ident to $trait:ident) => {
        unsafe {
            let mut bytes = [0u8; ::std::mem::size_of::<*mut dyn $trait>()];
            for e in $table {
                if e.0 == ::std::any::TypeId::of::<dyn $trait>() {
                    bytes = e.1;
                    break;
                }
            }
            ::std::mem::transmute_copy::<_, *mut dyn $trait>(&bytes)
        }
    };
}

#[macro_export]
macro_rules! t2a_cast_ref {
    ($table:ident to $trait:ident) => {
        unsafe { t2a_cast_ptr!($table to $trait).as_ref() }
    };
}

#[macro_export]
macro_rules! t2a_cast_mut {
    ($table:ident to $trait:ident) => {
        unsafe { t2a_cast_ptr!($table to $trait).as_mut() }
    };
}
