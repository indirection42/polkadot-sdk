// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Externalities extensions storage.
//!
//! Externalities support to register a wide variety custom extensions. The [`Extensions`] provides
//! some convenience functionality to store and retrieve these extensions.
//!
//! It is required that each extension implements the [`Extension`] trait.

use crate::Error;
use alloc::{
	boxed::Box,
	collections::btree_map::{BTreeMap, Entry},
};
use core::{
	any::{Any, TypeId},
	ops::DerefMut,
};

/// Marker trait for types that should be registered as [`Externalities`](crate::Externalities)
/// extension.
///
/// As extensions are stored as `Box<Any>`, this trait should give more confidence that the correct
/// type is registered and requested.
pub trait Extension: Send + 'static {
	/// Return the extension as `&mut dyn Any`.
	///
	/// This is a trick to make the trait type castable into an `Any`.
	fn as_mut_any(&mut self) -> &mut dyn Any;

	/// Get the [`TypeId`] of this `Extension`.
	fn type_id(&self) -> TypeId;
}

impl Extension for Box<dyn Extension> {
	fn as_mut_any(&mut self) -> &mut dyn Any {
		(**self).as_mut_any()
	}

	fn type_id(&self) -> TypeId {
		(**self).type_id()
	}
}

/// Macro for declaring an extension that usable with [`Extensions`].
///
/// The extension will be an unit wrapper struct that implements [`Extension`], `Deref` and
/// `DerefMut`. The wrapped type is given by the user.
///
/// # Example
/// ```
/// # use sp_externalities::decl_extension;
/// decl_extension! {
///     /// Some test extension
///     struct TestExt(String);
/// }
/// ```
#[macro_export]
macro_rules! decl_extension {
	(
		$( #[ $attr:meta ] )*
		$vis:vis struct $ext_name:ident ($inner:ty);
	) => {
		$( #[ $attr ] )*
		$vis struct $ext_name (pub $inner);

		impl $crate::Extension for $ext_name {
			fn as_mut_any(&mut self) -> &mut dyn core::any::Any {
				self
			}

			fn type_id(&self) -> core::any::TypeId {
				core::any::Any::type_id(self)
			}
		}

		impl core::ops::Deref for $ext_name {
			type Target = $inner;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl core::ops::DerefMut for $ext_name {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}

		impl From<$inner> for $ext_name {
			fn from(inner: $inner) -> Self {
				Self(inner)
			}
 		}
	};
	(
		$( #[ $attr:meta ] )*
		$vis:vis struct $ext_name:ident;
	) => {
		$( #[ $attr ] )*
		$vis struct $ext_name;

		impl $crate::Extension for $ext_name {
			fn as_mut_any(&mut self) -> &mut dyn core::any::Any {
				self
			}

			fn type_id(&self) -> core::any::TypeId {
				core::any::Any::type_id(self)
			}
		}
	}
}

/// Something that provides access to the [`Extensions`] store.
///
/// This is a super trait of the [`Externalities`](crate::Externalities).
pub trait ExtensionStore {
	/// Tries to find a registered extension by the given `type_id` and returns it as a `&mut dyn
	/// Any`.
	///
	/// It is advised to use [`ExternalitiesExt::extension`](crate::ExternalitiesExt::extension)
	/// instead of this function to get type system support and automatic type downcasting.
	fn extension_by_type_id(&mut self, type_id: TypeId) -> Option<&mut dyn Any>;

	/// Register extension `extension` with specified `type_id`.
	///
	/// It should return error if extension is already registered.
	fn register_extension_with_type_id(
		&mut self,
		type_id: TypeId,
		extension: Box<dyn Extension>,
	) -> Result<(), Error>;

	/// Deregister extension with specified 'type_id' and drop it.
	///
	/// It should return error if extension is not registered.
	fn deregister_extension_by_type_id(&mut self, type_id: TypeId) -> Result<(), Error>;
}

/// Stores extensions that should be made available through the externalities.
#[derive(Default)]
pub struct Extensions {
	extensions: BTreeMap<TypeId, Box<dyn Extension>>,
}

impl core::fmt::Debug for Extensions {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Extensions: ({})", self.extensions.len())
	}
}

impl Extensions {
	/// Create new instance of `Self`.
	pub fn new() -> Self {
		Self::default()
	}

	/// Register the given extension.
	pub fn register<E: Extension>(&mut self, ext: E) {
		let type_id = ext.type_id();
		self.extensions.insert(type_id, Box::new(ext));
	}

	/// Register extension `extension` using the given `type_id`.
	pub fn register_with_type_id(
		&mut self,
		type_id: TypeId,
		extension: Box<dyn Extension>,
	) -> Result<(), Error> {
		match self.extensions.entry(type_id) {
			Entry::Vacant(vacant) => {
				vacant.insert(extension);
				Ok(())
			},
			Entry::Occupied(_) => Err(Error::ExtensionAlreadyRegistered),
		}
	}

	/// Return a mutable reference to the requested extension.
	pub fn get_mut(&mut self, ext_type_id: TypeId) -> Option<&mut dyn Any> {
		self.extensions
			.get_mut(&ext_type_id)
			.map(DerefMut::deref_mut)
			.map(Extension::as_mut_any)
	}

	/// Deregister extension for the given `type_id`.
	///
	/// Returns `true` when the extension was registered.
	pub fn deregister(&mut self, type_id: TypeId) -> bool {
		self.extensions.remove(&type_id).is_some()
	}

	/// Returns a mutable iterator over all extensions.
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&TypeId, &mut Box<dyn Extension>)> {
		self.extensions.iter_mut()
	}

	/// Merge `other` into `self`.
	///
	/// If both contain the same extension, the extension instance of `other` will overwrite the
	/// instance found in `self`.
	pub fn merge(&mut self, other: Self) {
		self.extensions.extend(other.extensions);
	}
}

impl Extend<Extensions> for Extensions {
	fn extend<T: IntoIterator<Item = Extensions>>(&mut self, iter: T) {
		iter.into_iter()
			.for_each(|ext| self.extensions.extend(ext.extensions.into_iter()));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	decl_extension! {
		struct DummyExt(u32);
	}
	decl_extension! {
		struct DummyExt2(u32);
	}

	#[test]
	fn register_and_retrieve_extension() {
		let mut exts = Extensions::new();
		exts.register(DummyExt(1));
		exts.register(DummyExt2(2));

		let ext = exts.get_mut(TypeId::of::<DummyExt>()).expect("Extension is registered");
		let ext_ty = ext.downcast_mut::<DummyExt>().expect("Downcasting works");

		assert_eq!(ext_ty.0, 1);
	}

	#[test]
	fn register_box_extension() {
		let mut exts = Extensions::new();
		let box1: Box<dyn Extension> = Box::new(DummyExt(1));
		let box2: Box<dyn Extension> = Box::new(DummyExt2(2));
		exts.register(box1);
		exts.register(box2);

		{
			let ext = exts.get_mut(TypeId::of::<DummyExt>()).expect("Extension 1 is registered");
			let ext_ty = ext.downcast_mut::<DummyExt>().expect("Downcasting works for Extension 1");
			assert_eq!(ext_ty.0, 1);
		}
		{
			let ext2 = exts.get_mut(TypeId::of::<DummyExt2>()).expect("Extension 2 is registered");
			let ext_ty2 =
				ext2.downcast_mut::<DummyExt2>().expect("Downcasting works for Extension 2");
			assert_eq!(ext_ty2.0, 2);
		}
	}
}
