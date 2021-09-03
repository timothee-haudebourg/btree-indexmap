use generic_btree::node::Offset;
use crate::{
	M,
	Inner,
	index,
	Index
};
use super::{
	Metadata,
	Buffer
};

pub struct Mut<'a, K, V> {
	meta: &'a mut Metadata,
	data: &'a mut Inner<K, V>
}

impl<'a, K, V> Mut<'a, K, V> {
	pub fn new(meta: &'a mut Metadata, data: &'a mut Inner<K, V>) -> Self {
		Self {
			meta, data
		}
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::ItemAccess<crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn item_count(&self) -> usize {
		self.meta.branches.len()
	}

	fn borrow_item(&self, offset: Offset) -> Option<index::Ref<'r, K, V>> {
		panic!("TODO")
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::InternalRef<crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.meta.parent
	}

	fn child_id(&self, index: usize) -> Option<usize> {
		panic!("TODO")
	}

	fn max_capacity(&self) -> usize {
		M
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::InternalMut<'r, crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn set_parent(&mut self, parent: Option<usize>) {
		self.meta.parent = parent
	}

	fn set_first_child(&mut self, id: usize) {
		self.meta.first_child_id = id
	}

	fn into_item_mut(self, offset: Offset) -> Option<index::Mut<'a, K, V>> {
		panic!("TODO")
	}

	fn insert(&mut self, offset: Offset, item: Index, right_child_id: usize) {
		panic!("TODO")
	}

	fn remove(&mut self, offset: Offset) -> (Index, usize) {
		panic!("TODO")
	}

	fn replace(&mut self, offset: Offset, item: Index) -> Index {
		panic!("TODO")
	}

	fn append(&mut self, separator: Index, other: Buffer<K, V>) -> Offset {
		panic!("TODO")
	}
}