use super::{
	Branch, // Buffer
	Metadata,
};
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
		self.meta.branches.len()
	}

	fn borrow_item(&self, offset: Offset) -> Option<index::Ref<'_, K, V>> {
		offset
			.value()
			.map(|i| {
				self.meta
					.branches
					.get(i)
					.map(Branch::item_index)
					.map(move |index| index::Ref::new(index, self.data))
			})
			.flatten()
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::InternalRef<crate::Mut<'a, K, V>> for Mut<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.meta.parent
	}

	fn child_id(&self, index: usize) -> Option<usize> {
		self.meta.branches.get(index).map(|b| b.child_id)
	}

	fn max_capacity(&self) -> usize {
		M
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::InternalMut<'r, crate::Mut<'a, K, V>>
	for Mut<'r, K, V>
{
	fn set_parent(&mut self, parent: Option<usize>) {
		self.meta.parent = parent
	}

	fn set_first_child_id(&mut self, id: usize) {
		self.meta.first_child_id = id
	}

	fn into_item_mut(self, offset: Offset) -> Option<index::Mut<'r, K, V>> {
		offset
			.value()
			.map(|i| {
				self.meta
					.branches
					.get(i)
					.map(Branch::item_index)
					.map(move |index| index::Mut::new(index, self.data))
			})
			.flatten()
	}

	fn insert(&mut self, offset: Offset, item: Index, right_child_id: usize) {
		self.meta
			.branches
			.insert(offset.value().unwrap(), Branch::new(item, right_child_id))
	}

	fn remove(&mut self, offset: Offset) -> (Index, usize) {
		self.meta
			.branches
			.remove(offset.value().unwrap())
			.into_pair()
	}

	fn replace(&mut self, offset: Offset, mut index: Index) -> Index {
		std::mem::swap(
			&mut self.meta.branches[offset.value().unwrap()].item_index,
			&mut index,
		);
		index
	}

	fn append(&mut self, separator: Index, mut other: Metadata) -> Offset {
		let offset = self.meta.branches.len().into();
		self.meta
			.branches
			.push(Branch::new(separator, other.first_child_id));
		self.meta.branches.append(&mut other.branches);
		offset
	}
}
