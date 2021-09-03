use smallvec::SmallVec;
use generic_btree::node::Offset;
use crate::{
	M,
	Item,
	index,
	Index
};

pub struct Branch<K, V> {
	item: Item<K, V>,
	child_id: usize
}

pub struct Buffer<K, V> {
	parent: Option<usize>,
	first_child_id: usize,
	branches: SmallVec<[Branch<K, V>; M]>
}

impl<K, V> Default for Buffer<K, V> {
	fn default() -> Self {
		Self {
			parent: None,
			first_child_id: 0,
			branches: SmallVec::new()
		}
	}
}

impl<'a, K, V> generic_btree::node::buffer::Internal<crate::Mut<'a, K, V>> for Buffer<K, V> {
	fn parent(&self) -> Option<usize> {
		self.parent
	}

	fn set_parent(&mut self, parent: Option<usize>) {
		self.parent = parent
	}

	fn item_count(&self) -> usize {
		self.branches.len()
	}

	fn item<'r>(&'r self, offset: Offset) -> Option<index::Ref<'r, K, V>> where 'a: 'r {
		panic!("TODO")
	}

	fn child_id(&self, index: usize) -> Option<usize> {
		panic!("TODO")
	}

	fn max_capacity(&self) -> usize {
		M
	}

	fn set_first_child(&mut self, id: usize) {
		self.first_child_id = id
	}

	fn push_right(&mut self, item: Index, child: usize) {
		panic!("TODO")
	}

	fn forget(self) {
		std::mem::forget(self.branches)
	}
}