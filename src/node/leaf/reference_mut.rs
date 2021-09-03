use generic_btree::node::Offset;
use slab_lists::SlabList;
use crate::{
	M,
	index,
	Index,
	item
};
use super::{
	Metadata,
	Buffer
};

pub struct Mut<'a, K, V> {
	meta: &'a mut Metadata,
	data: &'a mut SlabList<item::Ordered<K, V>>
}

impl<'a, K, V> Mut<'a, K, V> {
	pub fn new(meta: &'a mut Metadata, data: &'a mut SlabList<item::Ordered<K, V>>) -> Self {
		Self {
			meta, data
		}
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::ItemAccess<crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn item_count(&self) -> usize {
		self.meta.items.len()
	}

	fn borrow_item(&self, offset: Offset) -> Option<index::Ref<'r, K, V>> {
		panic!("TODO")
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::LeafRef<crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.meta.parent
	}

	fn max_capacity(&self) -> usize {
		M+1
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::LeafMut<'r, crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn set_parent(&mut self, parent: Option<usize>) {
		self.meta.parent = parent
	}

	/// Returns a mutable reference to the item with the given offset in the node.
	fn item_mut(&mut self, offset: Offset) -> Option<index::Mut<'_, K, V>> {
		panic!("TODO")
	}

	fn into_item_mut(self, offset: Offset) -> Option<index::Mut<'a, K, V>> {
		panic!("TODO")
	}

	fn insert(&mut self, offset: Offset, item: Index) {
		// let index = self.items.push_back(item);
		panic!("TODO")
	}

	fn remove(&mut self, offset: Offset) -> Index {
		panic!("TODO")
	}

	fn append(&mut self, separator: Index, other: Buffer<K, V>) -> Offset {
		panic!("TODO")
	}
}