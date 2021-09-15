use crate::{Index, M};
use smallvec::SmallVec;

// mod buffer;
mod reference;
mod reference_mut;

// pub use buffer::Buffer;
pub use reference::Ref;
pub use reference_mut::Mut;

pub struct Metadata {
	parent: Option<usize>,
	items: SmallVec<[Index; M + 1]>,
}

impl Default for Metadata {
	fn default() -> Self {
		Self {
			parent: None,
			items: SmallVec::new(),
		}
	}
}

impl<'a, K, V> generic_btree::node::buffer::Leaf<crate::Mut<'a, K, V>> for Metadata {
	fn parent(&self) -> Option<usize> {
		self.parent
	}

	fn set_parent(&mut self, parent: Option<usize>) {
		self.parent = parent
	}

	fn item_count(&self) -> usize {
		self.items.len()
	}

	fn max_capacity(&self) -> usize {
		M
	}

	fn push_right(&mut self, index: Index) {
		self.items.push(index)
	}

	fn forget(self) {
		std::mem::forget(self.items)
	}
}
