use std::cmp::Ordering;
use crate::Index;

pub struct Item<K, V> {
	pub key: K,
	pub value: V
}

impl<K, V> Item<K, V> {
	pub fn new(key: K, value: V) -> Self {
		Self {
			key, value
		}
	}

	pub fn order(self, order: usize) -> Ordered<K, V> {
		Ordered {
			key: self.key,
			value: self.value,
			order
		}
	}
}

pub struct Ordered<K, V> {
	pub key: K,
	pub value: V,
	pub order: usize
}

impl<K, V> Ordered<K, V> {
	pub fn as_pair(&self) -> (&K, &V) {
		(&self.key, &self.value)
	}

	pub fn replace_value(&mut self, mut value: V) -> V {
		std::mem::swap(&mut self.value, &mut value);
		value
	}

	pub fn unordered(self) -> Item<K, V> {
		Item {
			key: self.key,
			value: self.value
		}
	}
}

impl<K1, V1, K2, V2> PartialEq<Ordered<K2, V2>> for Ordered<K1, V1> where K1: PartialEq<K2>, V1: PartialEq<V2> {
	fn eq(&self, other: &Ordered<K2, V2>) -> bool {
		self.key == other.key && self.value == other.value
	}
}

impl<K, V> Eq for Ordered<K, V> where K: Eq, V: Eq {}

impl<K1, V1, K2, V2> PartialOrd<Ordered<K2, V2>> for Ordered<K1, V1> where K1: PartialOrd<K2>, V1: PartialOrd<V2> {
	fn partial_cmp(&self, other: &Ordered<K2, V2>) -> Option<Ordering> {
		match self.key.partial_cmp(&other.key) {
			Some(Ordering::Equal) => self.value.partial_cmp(&other.value),
			o => o
		}
	}
}

impl<K, V> Ord for Ordered<K, V> where K: Ord, V: Ord {
	fn cmp(&self, other: &Self) -> Ordering {
		match self.key.cmp(&other.key) {
			Ordering::Equal => self.value.cmp(&other.value),
			o => o
		}
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::item::Mut<crate::Mut<'a, K, V>> for &'r mut Item<K, V> {
	fn swap(&mut self, item: &mut Index) {
		panic!("TODO")
	}
}