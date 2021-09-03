use std::cmp::Ordering;
use crate::{
	Inner,
	item
};
use super::Index;

pub struct Ref<'a, K, V> {
	index: Index,
	inner: &'a Inner<K, V>
}

impl<'a, K, V> Ref<'a, K, V> {
	pub fn index(&self) -> Index {
		self.index
	}

	pub fn item(&self) -> &item::Ordered<K, V> {
		self.inner.items.get(self.index).unwrap()
	}
}

impl<'a, 'b, K1, V1, K2, V2> PartialEq<Ref<'b, K2, V2>> for Ref<'a, K1, V1> where K1: PartialEq<K2>, V1: PartialEq<V2> {
	fn eq(&self, other: &Ref<'b, K2, V2>) -> bool {
		self.item().eq(other.item())
	}
}

impl<'a, K, V> Eq for Ref<'a, K, V> where K: Eq, V: Eq {}

impl<'a, 'b, K1, V1, K2, V2> PartialOrd<Ref<'b, K2, V2>> for Ref<'a, K1, V1> where K1: PartialOrd<K2>, V1: PartialOrd<V2> {
	fn partial_cmp(&self, other: &Ref<'b, K2, V2>) -> Option<Ordering> {
		self.item().partial_cmp(other.item())
	}
}

impl<'a, 'b, K, V> Ord for Ref<'a, K, V> where K: Ord, V: Ord {
	fn cmp(&self, other: &Self) -> Ordering {
		self.item().cmp(other.item())
	}
}