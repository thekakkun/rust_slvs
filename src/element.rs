use std::any::Any;

use crate::{bindings::Slvs_hGroup, System};

pub trait AsHandle: private::Sealed {
    fn handle(&self) -> u32;
}

pub trait AsGroup {
    fn group(&self) -> Slvs_hGroup;
}

pub trait AsSlvsType {
    fn slvs_type(&self) -> i32;
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait FromSystem {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! define_element {
    ($slvs_type:ident,
        struct $name:ident {
            $($field_name:ident: $field_type:ty,)*
        }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            pub group: Group,
            $(pub $field_name: $field_type,)*
        }

        impl $name {
            pub fn new(group: Group, $($field_name: $field_type,)*) -> Self {
                Self{
                    group,
                    $($field_name,)*
                }
            }
        }

        impl AsGroup for $name {
            fn group(&self) -> Slvs_hGroup {
                self.group.handle()
            }
        }

        impl AsSlvsType for $name {
            fn slvs_type(&self) -> i32 {
                $slvs_type as _
            }
        }
    };
}

mod private {
    use super::AsHandle;

    pub trait Sealed {}
    impl<H: AsHandle> Sealed for H {}
}
