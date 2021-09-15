use crate::{Index, M};
use smallvec::SmallVec;

// mod buffer;
mod reference;
mod reference_mut;

// pub use buffer::Buffer;
pub use reference::Ref;
pub use reference_mut::Mut;

/// Node branch.
pub struct Branch {
	/// Item index.
	item_index: Index,

	/// Child node index.
	child_id: usize,
}

impl Branch {
	fn new(item_index: Index, child_id: usize) -> Self {
		Self {
			item_index,
			child_id,
		}
	}

	fn item_index(&self) -> Index {
		self.item_index
	}

	fn into_pair(self) -> (Index, usize) {
		(self.item_index, self.child_id)
	}
}

/// Internal node metadata.
pub struct Metadata {
	/// Parent node index.
	parent: Option<usize>,

	/// First child identifier.
	first_child_id: usize,

	/// Branches.
	branches: SmallVec<[Branch; M]>,
}

impl Default for Metadata {
	fn default() -> Self {
		Self {
			parent: None,
			first_child_id: 0,
			branches: SmallVec::new(),
		}
	}
}

impl<'a, K, V> generic_btree::node::buffer::Internal<crate::Mut<'a, K, V>> for Metadata {
	fn parent(&self) -> Option<usize> {
		self.parent
	}

	fn set_parent(&mut self, parent: Option<usize>) {
		self.parent = parent
	}

	fn item_count(&self) -> usize {
		self.branches.len()
	}

	fn child_id(&self, index: usize) -> Option<usize> {
		self.branches.get(index).map(|b| b.child_id)
	}

	fn max_capacity(&self) -> usize {
		M
	}

	fn set_first_child_id(&mut self, id: usize) {
		self.first_child_id = id
	}

	fn push_right(&mut self, index: Index, child: usize) {
		self.branches.push(Branch::new(index, child))
	}

	fn forget(self) {
		std::mem::forget(self.branches)
	}
}
