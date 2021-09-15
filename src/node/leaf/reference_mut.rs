use super::Metadata;
use crate::{index, Index, Inner, M};
use generic_btree::node::Offset;

pub struct Mut<'a, K, V> {
	meta: &'a mut Metadata,
	data: &'a mut Inner<K, V>,
}

impl<'a, K, V> Mut<'a, K, V> {
	pub(crate) fn new(meta: &'a mut Metadata, data: &'a mut Inner<K, V>) -> Self {
		Self { meta, data }
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::ItemAccess<crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn item_count(&self) -> usize {
		self.meta.items.len()
	}

	fn borrow_item(&self, offset: Offset) -> Option<index::Ref<'_, K, V>> {
		offset
			.value()
			.map(|i| {
				self.meta
					.items
					.get(i)
					.cloned()
					.map(move |index| index::Ref::new(index, self.data))
			})
			.flatten()
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::LeafRef<crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.meta.parent
	}

	fn max_capacity(&self) -> usize {
		M + 1
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::LeafMut<'r, crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn set_parent(&mut self, parent: Option<usize>) {
		self.meta.parent = parent
	}

	/// Returns a mutable reference to the item with the given offset in the node.
	fn item_mut(&mut self, offset: Offset) -> Option<index::Mut<'_, K, V>> {
		offset
			.value()
			.map(|i| {
				self.meta
					.items
					.get(i)
					.cloned()
					.map(move |index| index::Mut::new(index, self.data))
			})
			.flatten()
	}

	fn into_item_mut(self, offset: Offset) -> Option<index::Mut<'r, K, V>> {
		offset
			.value()
			.map(|i| {
				self.meta
					.items
					.get(i)
					.cloned()
					.map(move |index| index::Mut::new(index, self.data))
			})
			.flatten()
	}

	fn insert(&mut self, offset: Offset, item: Index) {
		let i = offset.value().unwrap();
		self.meta.items.insert(i, item);
	}

	fn remove(&mut self, offset: Offset) -> Index {
		let i = offset.value().unwrap();
		self.meta.items.remove(i)
	}

	fn append(&mut self, separator: Index, mut other: Metadata) -> Offset {
		let offset = self.meta.items.len().into();
		self.meta.items.push(separator);
		self.meta.items.append(&mut other.items);
		offset
	}
}
