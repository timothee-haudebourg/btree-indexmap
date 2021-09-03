use smallvec::SmallVec;
use generic_btree::node::Offset;
use crate::{
	M,
	Item,
	index,
	Index
};

pub struct Buffer<K, V> {
	parent: Option<usize>,
	branches: SmallVec<[Item<K, V>; M]>
}

impl<K, V> Default for Buffer<K, V> {
	fn default() -> Self {
		Self {
			parent: None,
			branches: SmallVec::new()
		}
	}
}

impl<'a, K, V> generic_btree::node::buffer::Leaf<crate::Mut<'a, K, V>> for Buffer<K, V> {
	fn parent(&self) -> Option<usize> {
		self.parent
	}

	fn set_parent(&mut self, parent: Option<usize>) {
		self.parent = parent
	}

	fn item_count(&self) -> usize {
		self.branches.len()
	}

	fn item<'r>(&self, offset: Offset) -> Option<index::Ref<'a, K, V>> where 'a: 'r {
		panic!("TODO")
	}

	fn max_capacity(&self) -> usize {
		M
	}

	fn push_right(&mut self, index: Index) {
		panic!("TODO")
	}

	fn forget(self) {
		std::mem::forget(self.branches)
	}
}