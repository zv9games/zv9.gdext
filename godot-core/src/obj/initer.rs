/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::obj::{Base, Gd, GodotClass};

/// Initialization wrapper to construct objects inside `Gd<T>`.
///
/// `Initer<B>` provides a [`Base<B>`] for initialization and APIs for more
/// complex initialization workflows, such as obtaining a [`Gd<B>`] pointer to
/// the base object during initialization.
///
/// # Usage
///
/// ```no_run
/// # use godot::prelude::*;
/// # struct World;
/// # impl World { fn register_entity(gd: &Gd<impl GodotClass>) {} }
/// #[derive(GodotClass)]
/// # #[class(init)]
/// struct Player {
///     base: Base<RefCounted>,
///     health: i32,
/// }
///
/// let obj = Gd::from_init2_fn(|initer| {
///     // Obtain a Gd<RefCounted> pointer to the base object.
///     let gd = initer.to_gd();  
///     World::register_entity(&gd);
///     
///     Player {
///         base: initer.into_base(),
///         health: 100
///     }
/// });
/// ```
///
/// Use [`Gd::from_init2_fn()`] when you need to access the object during initialization.
pub struct Initer<B: GodotClass> {
    /// The base object that will become the permanent [`Base<B>`] field.
    base: Base<B>,
}

impl<B: GodotClass> Initer<B> {
    pub(crate) fn new(base: Base<B>) -> Self {
        Self { base }
    }

    /// Returns a [`Gd<B>`] referencing the base object being constructed.
    ///
    /// Can be called multiple times during object initialization, similar to
    /// [`Base::to_init_gd`].
    ///
    /// # Usage
    /// ```no_run
    /// # use godot::prelude::*;
    /// # struct GameManager;
    /// # impl GameManager { fn register(gd: &Gd<impl GodotClass>) {} }
    /// #[derive(GodotClass)]
    /// #[class(init, base=RefCounted)]
    /// struct MyPlayer {
    ///     base: Base<RefCounted>
    /// }
    ///
    /// let obj = Gd::from_init2_fn(|initer| {
    ///     let gd = initer.to_gd();
    ///     GameManager::register(&gd);
    ///     
    ///     MyPlayer {
    ///         base: initer.into_base(),
    ///     }
    /// });
    /// ```
    pub fn to_gd(&self) -> Gd<B> {
        // Delegate to the base's to_init_gd() method.
        self.base.to_init_gd()
    }

    /// Converts this `Initer<B>` into a [`Base<B>`] for use as a field.
    ///
    /// # Usage
    /// ```no_run
    /// # use godot::prelude::*;
    /// #[derive(GodotClass)]
    /// # #[class(init)]
    /// struct MyClass {
    ///     base: Base<RefCounted>
    /// }
    ///
    /// let obj = Gd::<MyClass>::from_init2_fn(|initer| {
    ///     MyClass {
    ///         base: initer.into_base(),
    ///     }
    /// });
    /// ```
    pub fn into_base(self) -> Base<B> {
        self.base
    }
}

impl<B: GodotClass> Debug for Initer<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Initer").field("base", &self.base).finish()
    }
}

impl<B: GodotClass> Display for Initer<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Initer<{}>({})", B::class_name(), self.base)
    }
}
