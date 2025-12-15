//! This module defines the traits and types for handling extension call data.
use codec::Decode;
use scale_info::prelude::vec::Vec;

/// The type for extension identifiers.
pub type ExtensionIdTy = u64;

/// Identifies an extension by a numeric ID.
///
/// In practice, this is implemented for macro-generated call types and the ID is derived from the
/// extension interface (so changing the trait's signature changes the ID).
pub trait ExtensionId {
	/// The unique identifier of the extension.
	const EXTENSION_ID: ExtensionIdTy;
}

/// Dispatches an already-decoded extension call.
///
/// Implementations typically:
/// - call into the host implementation (e.g. your runtime), and
/// - return the SCALE-encoded return value as raw bytes.
pub trait Dispatchable {
	/// Dispatches the extension call.
	fn dispatch(self) -> Result<Vec<u8>, DispatchError>;
}

/// The error type for dispatch operations.
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum DispatchError {
	/// A marker/placeholder variant was dispatched.
	///
	/// The extension-decl macro generates an internal `__marker(PhantomData<Impl>)` enum variant
	/// to keep the implementation type in the call enum. That variant is `#[doc(hidden)]` and is
	/// not meant to ever be constructed from real call data.
	#[cfg_attr(feature = "std", error("PhantomData"))]
	PhantomData,
}

/// A trait for extension call data.
///
/// This trait combines several traits that are required for extension call data:
/// - `Dispatchable`: Allows dispatching calls to the extension functions.
/// - `ExtensionId`: Identifies the extension.
/// - `Decode`: Allows decoding the call data.
pub trait CallData: Dispatchable + ExtensionId + Decode {}
impl<T> CallData for T where T: Dispatchable + ExtensionId + Decode {}
