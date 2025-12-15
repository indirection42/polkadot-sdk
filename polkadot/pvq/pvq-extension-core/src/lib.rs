#![cfg_attr(not(feature = "std"), no_std)]
//! # PVQ extension core
//!
//! Defines an example core PVQ extension.
//!
//! ## Overview
//!
//! The core extension provides [`extension::ExtensionCore::has_extension`], which PVQ programs can
//! use to detect whether an optional host extension is enabled before attempting to call it.
//!
//! ## Usage
//!
//! ```ignore
//! use pvq_extension_core::extension::ExtensionCore;
//!
//! // The concrete `ExtensionId` type is defined by the host/runtime.
//! // if ExtensionCore::has_extension(id) {
//! //     // safe to call into the optional extension
//! // }
//! ```
//!
//! ## Feature flags
//!
//! - `std`: Enables standard library support.

use pvq_extension::extension_decl;

/// The core PVQ extension definitions.
#[extension_decl]
pub mod extension {
	//! Core extension definition for PVQ programs.
	//!
	//! This module is augmented by the [`pvq_extension::extension_decl`] macro.

	/// The core PVQ extension trait.
	#[extension_decl::extension]
	pub trait ExtensionCore {
		/// Identifier of a host extension.
		///
		/// This type is provided by the host/runtime and is used as input to
		/// [`Self::has_extension`].
		type ExtensionId;

		/// Returns `true` if the extension identified by `id` is enabled.
		///
		/// PVQ programs can use this to feature-gate calls to optional extensions.
		fn has_extension(id: Self::ExtensionId) -> bool;
	}
}
