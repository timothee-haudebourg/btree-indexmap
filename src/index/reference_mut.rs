use super::Index;
use crate::{Inner, Item};

pub struct Mut<'a, K, V> {
	index: Index,
	inner: &'a mut Inner<K, V>,
}

impl<'a, K, V> Mut<'a, K, V> {
	pub(crate) fn new(index: Index, inner: &'a mut Inner<K, V>) -> Self {
		Self { index, inner }
	}

	// pub fn index(&self) -> Index {
	// 	self.index
	// }

	pub fn into_index(self) -> Index {
		self.index
	}

	// pub fn item_mut(&mut self) -> &mut item::Ordered<K, V> {
	// 	self.inner.items.get_mut(self.index).unwrap()
	// }

	// pub fn into_item_mut(self) -> &'a mut item::Ordered<K, V> {
	// 	self.inner.items.get_mut(self.index).unwrap()
	// }
}

impl<'r, 'a, K, V> generic_btree::node::item::Mut<crate::Mut<'r, K, V>> for Mut<'a, K, V> {
	fn swap(&mut self, index: &mut Index) {
		std::mem::swap(&mut self.index, index)
	}
}

impl<'a, K, V> generic_btree::node::item::Replace<crate::Mut<'a, K, V>, Item<K, V>>
	for Mut<'a, K, V>
{
	type Output = (Index, V);

	fn replace(&mut self, item: Item<K, V>) -> Self::Output {
		(
			self.index,
			self.inner
				.items
				.get_mut(self.index)
				.unwrap()
				.replace_value(item.value),
		)
	}
}
