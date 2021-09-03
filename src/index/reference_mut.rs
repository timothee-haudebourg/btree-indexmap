use crate::{
	Inner,
	Item
};
use super::Index;

pub struct Mut<'a, K, V> {
	index: Index,
	inner: &'a mut Inner<K, V>
}

impl<'r, 'a, K, V> generic_btree::node::item::Mut<crate::Mut<'r, K, V>> for Mut<'a, K, V> {
	fn swap(&mut self, index: &mut Index) {
		std::mem::swap(&mut self.index, index)
	}
}

impl<'a, K, V> generic_btree::node::item::Replace<crate::Mut<'a, K, V>, Item<K, V>> for Mut<'a, K, V> {
	type Output = V;

	fn replace(&mut self, item: Item<K, V>) -> V {
		self.inner.items.get_mut(self.index).unwrap().replace_value(item.value)
	}
}