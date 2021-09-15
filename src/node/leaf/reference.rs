use super::Metadata;
use crate::{index, Inner, M};
use generic_btree::node::Offset;

pub struct Ref<'a, K, V> {
	meta: &'a Metadata,
	data: &'a Inner<K, V>,
}

impl<'a, K, V> Ref<'a, K, V> {
	pub(crate) fn new(meta: &'a Metadata, data: &'a Inner<K, V>) -> Self {
		Self { meta, data }
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::ItemAccess<crate::Ref<'a, K, V>> for Ref<'r, K, V> {
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

impl<'r, 'a: 'r, K, V> generic_btree::node::ItemAccess<crate::Mut<'a, K, V>> for Ref<'r, K, V> {
	fn item_count(&self) -> usize {
		self.meta.items.len()
	}

	fn borrow_item(&self, offset: Offset) -> Option<index::Ref<'r, K, V>> {
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

impl<'r, 'a: 'r, K, V> generic_btree::node::LeafRef<crate::Ref<'a, K, V>> for Ref<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.meta.parent
	}

	fn max_capacity(&self) -> usize {
		M + 1
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::LeafRef<crate::Mut<'a, K, V>> for Ref<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.meta.parent
	}

	fn max_capacity(&self) -> usize {
		M + 1
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::LeafConst<'r, crate::Ref<'a, K, V>> for Ref<'r, K, V> {
	fn item(&self, offset: Offset) -> Option<index::Ref<'r, K, V>> {
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

impl<'r, 'a: 'r, K, V> generic_btree::node::LeafConst<'r, crate::Mut<'a, K, V>> for Ref<'r, K, V> {
	fn item(&self, offset: Offset) -> Option<index::Ref<'r, K, V>> {
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
