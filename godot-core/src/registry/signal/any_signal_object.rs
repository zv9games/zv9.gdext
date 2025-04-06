/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::classes;
use crate::classes::Object;
use crate::obj::{Gd, Inherits, WithUserSignals};

/// Internal representation of a signal collection.
// #[doc(hidden)]
// pub struct SignalColl<'c> {
//     object: UserSignalObj<'c>,
// }
//
// impl<'c> SignalColl<'c> {
//     pub fn from_external<C>(concrete_gd: &'c mut Gd<C>) -> Self
//     where
//         C: Inherits<classes::Object>,
//     {
//         Self {
//             object: concrete_gd.clone().upcast(),
//         }
//     }
//
//     pub fn from_gd<C>(concrete_gd: Gd<C>) -> Self
//     where
//         C: Inherits<classes::Object>,
//     {
//         Self {
//             object: concrete_gd.upcast(),
//         }
//     }
//
//     /// Hand out a new object pointer for an individual signal.
//     pub fn hand_out(&self) -> Gd<classes::Object> {
//         self.object.clone()
//     }
// }

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Type-erased representation for an object (either owned externally, or borrowed internally).
///
/// Links to a Godot object, either via reference (for `&mut self` uses) or via `Gd`.
#[doc(hidden)]
pub enum ErasedSignalObj<'c> {
    // Note: we could technically extend AnySignalObject to include all Gd<T> and unify the two, however that would require separate v-table
    // (or monomorphization in case of static polymorphism) for each externally used type (i.e. all Godot classes), while this implementation
    // requires only one for each user class.
    /// Helpful for emit: reuse `&mut self` from within the `impl` block, goes through `base_mut()` re-borrowing and thus allows re-entrant calls
    /// through Godot.
    Internal {
        self_mut: &'c mut dyn AnySignalObject,
    },

    /// From outside, based on `Gd` pointer.
    External { gd: Gd<Object> },
}

// pub struct ErasedSignalObject<'c> {
//     object: &'c mut dyn AnySignalObject,
// }

impl<'c> ErasedSignalObj<'c> {
    /// Used by engine classes, stores a reference to the `Gd` pointer.
    pub fn from_external<C>(object: &'c mut Gd<C>) -> Self
    where
        C: Inherits<Object>,
    {
        Self::External {
            gd: object.clone().upcast(),
        }
    }

    /// Used by user classes, stores reference via `dyn WithUserSignals` trait object.
    pub fn from_internal(object: &'c mut dyn AnySignalObject) -> Self {
        Self::Internal { self_mut: object }
    }

    pub(crate) fn with_object_mut<F>(&mut self, _f: F)
    where
        F: FnOnce(&mut Object),
    {
        todo!()
    }

    // pub(crate) fn to_owned_object(&self) -> Gd<Object> {
    //     self.to_owned_object()
    // }

    // pub(crate) fn with_object_mut(&mut self, f: impl FnOnce(&mut classes::Object)) {
    //     match self {
    //         ErasedSignalObject::Internal { self_ref } => todo!(),
    //         ErasedSignalObject::External { gd } => f(gd.upcast_object_mut()),
    //     }
    // }
    //
    // pub(crate) fn borrow_object_mut(&mut self) -> &mut classes::Object {
    //     match self {
    //         ErasedSignalObject::Internal { self_ref } => self_ref.borrow_object_mut(),
    //         ErasedSignalObject::External { gd } => gd.upcast_object_mut(),
    //     }
    // }
    //
    pub(crate) fn to_owned_object(&self) -> Gd<Object> {
        match self {
            Self::Internal { self_mut } => self_mut.to_owned_object(),
            Self::External { gd } => gd.clone(),
        }
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// Trait for the type erasure.
#[doc(hidden)]
pub trait AnySignalObject {
    // fn with_object_mut(&mut self, f: impl FnOnce(&mut classes::Object));
    fn borrow_object_mut(&mut self) -> &mut classes::Object;
    fn to_owned_object(&self) -> Gd<classes::Object>;
}

impl<T> AnySignalObject for T
where
    T: WithUserSignals,
{
    fn borrow_object_mut(&mut self) -> &mut Object {
        todo!()
    }

    fn to_owned_object(&self) -> Gd<Object> {
        self.to_gd().upcast()
    }
}

/*
/// Indirection from [`TypedSignal`] to the actual Godot object.
///
/// Needs to differentiate the two cases:
/// - `C` is a user object implementing `WithBaseField`, possibly having access from within the class.
/// - `C` is an engine object, so only accessible through `Gd<C>`.
pub(crate) trait SignalObj<C: GodotClass> {
    fn with_object_mut(&mut self, f: impl FnOnce(&mut classes::Object));
    fn to_owned_object(&self) -> Gd<C>;
}

impl<C: WithBaseField> SignalObj<C> for UserSignalObj<'_, C> {
    fn with_object_mut(&mut self, f: impl FnOnce(&mut classes::Object)) {
        match self {
            UserSignalObj::Internal { obj_mut } => f(obj_mut.base_mut().upcast_object_mut()),
            UserSignalObj::External { gd } => f(gd.upcast_object_mut()),
        }
    }

    fn to_owned_object(&self) -> Gd<C> {
        match self {
            UserSignalObj::Internal { obj_mut } => WithBaseField::to_gd(*obj_mut),
            UserSignalObj::External { gd } => gd.clone(),
        }
    }
}

impl<C: GodotClass> SignalObj<C> for Gd<C> {
    fn with_object_mut(&mut self, f: impl FnOnce(&mut classes::Object)) {
        f(self.upcast_object_mut());
    }

    fn to_owned_object(&self) -> Gd<C> {
        self.clone()
    }
}*/
