/*
 * indent-stack
 *
 * Copyright (C) 2019 chankyin
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Provides a wrapper for integer types with modular arithmetic behaviour.
//!
//! Since modular arithmetic is commonly used with a specific prime modulus, this crate removes the
//! cost of storing the modulus by hardcoding it in the definition and generating definitions with
//! a macro, enhancing performance.
//!
//! This also automatically moves modulus equality checking (because values of different modulo
//! cannot be added directly) from runtime to compile time, enhancing stability.

#![no_std]
#![cfg_attr(nightly, feature(underscore_const_names, doc_cfg))]

use core::fmt::Debug;
use core::ops::{Add, Mul, Rem, Sub};

mod def_macro;

/// The Modular trait is implemented by all modular-arithmetic-defining classes.
pub trait Modular<T: Copy + Debug + Default + Add + Sub + Mul + Rem>: 
Copy + Debug + From<T> + Add + Sub + Mul + Eq {
    /// The modulus of this type.
    const MOD: T;

    /// The remainder of the value divided by `MOD`.
    fn remainder(&self) -> T;
}

#[cfg_attr(nightly, cfg(rustdoc))]
def_modular!(ExampleModular101 : u16 | i16, 101 ; example_modular_101_lbl
             #[doc = "This struct is an example struct used to demonstrate what is generated by the [`def_modular`](macro.def_modular.html) macro."]
             #[doc = ""]
             #[doc = "Do not try to use this type anywhere (including tests), because this type is only declared in `cfg(rustdoc)`."]
             );

#[cfg(test)]
mod tests;
