#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A direct parent, as recorded by the Department of Ancestral Paperwork.
///
/// Derived classes implement this automatically. Base classes have no parent
/// and therefore no paperwork. These operations address only the direct parent;
/// call them again to reach a grandparent.
pub trait HasParent: Sized {
    /// The concrete type stored in this class's `super_class` field.
    type Parent;

    /// Borrow the direct parent.
    fn parent(&self) -> &Self::Parent;

    /// Mutably borrow the direct parent.
    fn parent_mut(&mut self) -> &mut Self::Parent;

    /// Consume the child and return its parent, dropping the child's own fields.
    fn into_parent(self) -> Self::Parent;
}

/// Declare a struct with a constructor and optional parent delegation.
///
/// See the crate documentation for syntax, visibility, custom derives, and the
/// legally significant difference between method shadowing and virtual dispatch.
/// Each invocation declares one class. Class-level generic parameters are not
/// supported; methods may have ordinary Rust generics and `where` clauses.
///
/// Generated names are `new`, and (for derived classes) `super_class`,
/// `super_cast`, `super_cast_mut`, and `into_super`. Do not redeclare them.
#[macro_export]
macro_rules! class {
    // The original syntax always made the class public. Keep that contract.
    (plain $(#[$attr:meta])* class $name:ident $($rest:tt)*) => {
        $crate::class! { plain $(#[$attr])* pub class $name $($rest)* }
    };
    ($(#[$attr:meta])* class $name:ident $($rest:tt)*) => {
        $crate::class! { $(#[$attr])* pub class $name $($rest)* }
    };

    // `plain` opts out of the complimentary Debug + Clone personality package.
    (plain $(#[$attr:meta])* $vis:vis class $name:ident extends $parent:ty {
        $($(#[$field_attr:meta])* $field_vis:vis $field:ident : $field_ty:ty),* $(,)?
    } $($item:item)*) => {
        $crate::class! { @derived [$(#[$attr])*] $vis $name [$parent] {
            $($(#[$field_attr])* $field_vis $field: $field_ty),*
        } $($item)* }
    };
    (plain $(#[$attr:meta])* $vis:vis class $name:ident {
        $($(#[$field_attr:meta])* $field_vis:vis $field:ident : $field_ty:ty),* $(,)?
    } $($item:item)*) => {
        $crate::class! { @base [$(#[$attr])*] $vis $name {
            $($(#[$field_attr])* $field_vis $field: $field_ty),*
        } $($item)* }
    };
    ($(#[$attr:meta])* $vis:vis class $name:ident extends $parent:ty {
        $($(#[$field_attr:meta])* $field_vis:vis $field:ident : $field_ty:ty),* $(,)?
    } $($item:item)*) => {
        $crate::class! { @derived [#[derive(Debug, Clone)] $(#[$attr])*] $vis $name [$parent] {
            $($(#[$field_attr])* $field_vis $field: $field_ty),*
        } $($item)* }
    };
    ($(#[$attr:meta])* $vis:vis class $name:ident {
        $($(#[$field_attr:meta])* $field_vis:vis $field:ident : $field_ty:ty),* $(,)?
    } $($item:item)*) => {
        $crate::class! { @base [#[derive(Debug, Clone)] $(#[$attr])*] $vis $name {
            $($(#[$field_attr])* $field_vis $field: $field_ty),*
        } $($item)* }
    };

    (@base [$($attr:tt)*] $vis:vis $name:ident {
        $($(#[$field_attr:meta])* $field_vis:vis $field:ident : $field_ty:ty),* $(,)?
    } $($item:item)*) => {
        $($attr)*
        $vis struct $name {
            $($(#[$field_attr])* $field_vis $field: $field_ty),*
        }

        impl $name {
            /// Construct a class from its fields, in declaration order.
            /// No application form, reference check, or Java installation needed.
            #[inline]
            pub fn new($($field: $field_ty),*) -> Self {
                Self { $($field),* }
            }

            $($item)*
        }
    };
    (@derived [$($attr:tt)*] $vis:vis $name:ident [$parent:ty] {
        $($(#[$field_attr:meta])* $field_vis:vis $field:ident : $field_ty:ty),* $(,)?
    } $($item:item)*) => {
        $($attr)*
        $vis struct $name {
            /// The direct parent. The family resemblance is literally stored here.
            pub super_class: $parent,
            $($(#[$field_attr])* $field_vis $field: $field_ty),*
        }

        impl $name {
            /// Construct a child from its parent, then its fields in declaration order.
            #[inline]
            pub fn new(super_instance: $parent, $($field: $field_ty),*) -> Self {
                Self { super_class: super_instance, $($field),* }
            }

            /// Borrow the direct parent, bypassing the child's method shadowing.
            #[inline]
            pub fn super_cast(&self) -> &$parent {
                &self.super_class
            }

            /// Mutably borrow the direct parent. Family therapy sold separately.
            #[inline]
            pub fn super_cast_mut(&mut self) -> &mut $parent {
                &mut self.super_class
            }

            /// Consume the child and recover its parent, dropping child-only fields.
            #[inline]
            pub fn into_super(self) -> $parent {
                self.super_class
            }

            $($item)*
        }

        impl ::core::ops::Deref for $name {
            type Target = $parent;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.super_class
            }
        }

        impl ::core::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.super_class
            }
        }

        impl $crate::HasParent for $name {
            type Parent = $parent;

            #[inline]
            fn parent(&self) -> &Self::Parent {
                &self.super_class
            }

            #[inline]
            fn parent_mut(&mut self) -> &mut Self::Parent {
                &mut self.super_class
            }

            #[inline]
            fn into_parent(self) -> Self::Parent {
                self.super_class
            }
        }

        impl ::core::convert::AsRef<$parent> for $name {
            #[inline]
            fn as_ref(&self) -> &$parent {
                &self.super_class
            }
        }

        impl ::core::convert::AsMut<$parent> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut $parent {
                &mut self.super_class
            }
        }
    };
    ($($unsupported:tt)*) => {
        ::core::compile_error!(
            "The Architecture Committee rejected this class declaration. Expected: \
             class Name { fields } methods, or class Child extends Parent { fields } methods. \
             Put `plain` first to disable default derives; class-level generics are unsupported."
        );
    };
}
