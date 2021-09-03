use smallvec::SmallVec;
use crate::{
	M,
	Index
};

mod buffer;
mod reference;
mod reference_mut;

pub use buffer::Buffer;
pub use reference::Ref;
pub use reference_mut::Mut;

/// Node branch.
pub struct Branch {
	/// Item index.
	item_index: Index,

	/// Child node index.
	child_id: usize
}

/// Internal node metadata.
pub struct Metadata {
	/// Parent node index.
	parent: Option<usize>,

	/// First child identifier.
	first_child_id: usize,

	/// Branches.
	branches: SmallVec<[Branch; M]>
}