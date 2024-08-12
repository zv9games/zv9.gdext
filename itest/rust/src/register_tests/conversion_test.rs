/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;

#[derive(GodotClass)]
#[class(init)]
struct ConversionTest {}

#[godot_api]
impl ConversionTest {
    #[func]
    fn accept_float(value: f32) {
        println!("accept_float: {value}");
    }

    #[func]
    fn accept_packed(value: PackedInt32Array) {
        println!("accept_packed: {value}");
    }
}
