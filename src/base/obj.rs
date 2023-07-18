pub trait ObjImpl {
    type ObjRc;

    fn obj_ref(&self) -> Self::ObjRc;
    fn obj_ref_safe(&self) -> Self::ObjRc;
}

#[macro_export]
macro_rules! obj_impl {
    (
        $name:ident with $rc:ident
    ) => {
        impl $crate::base::obj::ObjImpl for $name {
            type ObjRc = $rc<Self>;

            fn obj_ref(&self) -> Self::ObjRc {
                unsafe {
                    self.obj_self
                        .get()
                        .unwrap_unchecked()
                        .upgrade()
                        .unwrap_unchecked()
                }
            }

            fn obj_ref_safe(&self) -> Self::ObjRc {
                self.obj_self.get().unwrap().upgrade().unwrap()
            }
        }
    };
}

#[macro_export]
macro_rules! obj_type {
    ($name:ident with $rc:ident) => {
        ::std::cell::OnceCell<$rc<$name>>
    };
}
