#![feature(generic_associated_types)]
use generic_btree::{Storage, StorageMut};
use slab::Slab;
use slab_lists::SlabList;
use std::{borrow::Borrow, fmt};

mod index;
mod item;
mod node;
mod reference;
mod reference_mut;

pub(crate) use index::Index;
pub(crate) use item::Item;
pub(crate) use node::Node;
pub(crate) use reference::Ref;
pub(crate) use reference_mut::Mut;

/// Calculates the quotient of `a` and `b`, rounding the result towards positive infinity.
///
/// TODO: replace this by the standard `div_ceil` once `int_roundings` is stabilized.
const fn div_ceil(a: usize, b: usize) -> usize {
	let d = a / b;
	let r = a % b;
	if r > 0 && b > 0 {
		d + 1
	} else {
		d
	}
}

/// Knuth order of the B-Trees.
///
/// Must be at least 4.
const M: usize = 8;

pub struct IndexMap<K, V> {
	/// BTree nodes.
	///
	/// Note that the nodes does not actually store the items
	/// of the collection, but only indexes referencing the items
	/// in the inner linked list.
	nodes: Slab<Node>,

	/// Inner data structure storing the actua items data
	/// and ordering.
	inner: Inner<K, V>,

	/// Root BTree node.
	root: Option<usize>,
}

impl<K, V> IndexMap<K, V> {
	/// Creates a new empty map.
	#[inline]
	pub fn new() -> Self {
		Self {
			nodes: Slab::new(),
			inner: Inner::new(),
			root: None,
		}
	}

	/// Creates a new empty map with the given capacity.
	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		let nodes_capacity = div_ceil(capacity, M - 1);
		Self {
			nodes: Slab::with_capacity(nodes_capacity),
			inner: Inner::with_capacity(capacity),
			root: None,
		}
	}

	/// BTree reference.
	#[inline]
	fn btree(&self) -> Ref<K, V> {
		Ref::new(&self.nodes, &self.inner, self.root)
	}

	/// Mutable BTree reference.
	#[inline]
	fn btree_mut(&mut self) -> Mut<K, V> {
		Mut::new(&mut self.nodes, &mut self.inner, &mut self.root)
	}

	/// Returns the current item capacity of the B-Tree.
	///
	/// Every node contains at most M-1 items.
	#[inline]
	fn btree_capacity(&self) -> usize {
		self.nodes.capacity() * (M - 1)
	}

	#[inline]
	pub fn capacity(&self) -> usize {
		std::cmp::min(self.inner.capacity(), self.btree_capacity())
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.inner.items.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.inner.items.is_empty()
	}

	/// Get the index, key and value matching the given key.
	///
	/// Computes in **O(log n)** time (average).
	#[inline]
	pub fn get_full<'a, Q: ?Sized>(&'a self, key: &Q) -> Option<(usize, &'a K, &'a V)>
	where
		K: Borrow<Q>,
		Q: Ord,
		Self: 'a,
	{
		let btree = self.btree();
		let index: Option<Index> = btree.get(key).map(index::Ref::into_index);
		index.map(move |index| {
			let (key, value) = self.inner.items.get(index).unwrap().as_pair();
			(index, key, value)
		})
	}

	/// Get by key.
	///
	/// Computes in **O(log n)** time (average).
	#[inline]
	pub fn get<'a, Q: ?Sized>(&'a self, key: &Q) -> Option<&'a V>
	where
		K: Borrow<Q>,
		Q: Ord,
		Self: 'a,
	{
		let btree = self.btree();
		let index: Option<Index> = btree.get(key).map(index::Ref::into_index);
		index
			.map(move |index| self.inner.items.get(index).unwrap())
			.map(item::Ordered::as_value)
	}

	/// Get mutably by key.
	///
	/// ## Complexity
	///
	/// Computes in **O(log n)** time (average).
	#[inline]
	pub fn get_mut<'a, Q: ?Sized>(&'a mut self, key: &Q) -> Option<&'a mut V>
	where
		K: Borrow<Q>,
		Q: Ord,
		Self: 'a,
	{
		let mut btree = self.btree_mut();
		let index: Option<Index> = btree.get_mut(key).map(index::Mut::into_index);
		index
			.map(move |index| self.inner.items.get_mut(index).unwrap())
			.map(item::Ordered::as_value_mut)
	}

	/// Get by index.
	///
	/// Computes in **O(1)** time (average).
	#[inline]
	pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
		self.inner
			.vec
			.get(index)
			.map(|i| self.inner.items.get(*i))
			.flatten()
			.map(|item| item.as_pair())
	}

	/// Get mutably by index.
	///
	/// Computes in **O(1)** time (average).
	#[inline]
	pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
		self.inner
			.vec
			.get_mut(index)
			.cloned()
			.map(move |i| self.inner.items.get_mut(i))
			.flatten()
			.map(|item| item.as_pair_mut())
	}

	/// Inserts a key-value pair in the map and returns their index.
	///
	/// Computes in **O(log n)** time (average).
	#[inline]
	pub fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>)
	where
		K: Ord,
	{
		match self.btree_mut().insert(Item::new(key, value)) {
			Some((index, value)) => (index, Some(value)),
			None => (*self.inner.vec.last().unwrap(), None),
		}
	}

	/// Inserts a key-value pair in the map.
	///
	/// Computes in **O(log n)** time (average).
	#[inline]
	pub fn insert(&mut self, key: K, value: V) -> Option<V>
	where
		K: Ord,
	{
		self.btree_mut()
			.insert(Item::new(key, value))
			.map(|(_, value)| value)
	}

	/// Remove value.
	///
	/// This is an alias to [`Self::swap_remove`],
	/// meaning that this function *disturbs the order of the map*.
	/// Use [`Self::shift_remove`] to remove the value and preserve the order
	/// by sacrificing the average computation time.
	///
	/// Computes in **O(log n)** time (average).
	#[inline]
	pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
	where
		K: Borrow<Q>,
		Q: Ord,
	{
		self.swap_remove(key)
	}

	/// Remove value.
	///
	/// Computes in **O(log n)** time (average).
	#[inline]
	pub fn swap_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
	where
		K: Borrow<Q>,
		Q: Ord,
	{
		match self.btree_mut().remove(key) {
			Some(index) => {
				let item = self.inner.swap_remove(index);
				Some(item.value)
			}
			None => None,
		}
	}

	/// Remove value.
	///
	/// Computes in **O(n)** time (average).
	#[inline]
	pub fn shift_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
	where
		K: Borrow<Q>,
		Q: Ord,
	{
		match self.btree_mut().remove(key) {
			Some(index) => {
				let item = self.inner.shift_remove(index);
				Some(item.value)
			}
			None => None,
		}
	}

	/// Returns an iterator over the bindings of the map.
	///
	/// Bindings are iterated by order of insertion in the map.
	#[inline]
	pub fn iter(&self) -> Iter<K, V> {
		Iter {
			inner: &self.inner,
			indexes: self.inner.vec.iter(),
		}
	}
}

impl<K, V> Default for IndexMap<K, V> {
	fn default() -> Self {
		Self::new()
	}
}

impl<'a, K, V, Q> std::ops::Index<&'a Q> for IndexMap<K, V>
where
	K: Borrow<Q>,
	Q: ?Sized + Ord,
{
	type Output = V;

	fn index(&self, key: &'a Q) -> &V {
		self.get(key).unwrap()
	}
}

impl<'a, K, V, Q> std::ops::IndexMut<&'a Q> for IndexMap<K, V>
where
	K: Borrow<Q>,
	Q: ?Sized + Ord,
{
	fn index_mut(&mut self, key: &'a Q) -> &mut V {
		self.get_mut(key).unwrap()
	}
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for IndexMap<K, V> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_map().entries(self.iter()).finish()
	}
}

impl<'a, K, V> IntoIterator for &'a IndexMap<K, V> {
	type Item = (&'a K, &'a V);
	type IntoIter = Iter<'a, K, V>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<K, V> IntoIterator for IndexMap<K, V> {
	type Item = (K, V);
	type IntoIter = IntoIter<K, V>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
			indexes: self.inner.vec.into_iter(),
			items: self.inner.items,
		}
	}
}

pub(crate) struct Inner<K, V> {
	/// Items.
	items: SlabList<item::Ordered<K, V>>,

	/// Ordering array.
	vec: Vec<Index>,
}

impl<K, V> Inner<K, V> {
	#[inline]
	pub fn new() -> Self {
		Self {
			items: SlabList::new(),
			vec: Vec::new(),
		}
	}

	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			items: SlabList::with_capacity(capacity),
			vec: Vec::with_capacity(capacity),
		}
	}

	#[inline]
	pub fn capacity(&self) -> usize {
		self.items.capacity()
	}

	/// Insert the given item.
	///
	/// Computes in **O(1)** time (average).
	#[inline]
	pub fn insert(&mut self, item: Item<K, V>) -> Index {
		let order = self.vec.len();
		let index = self.items.push_back(item.order(order));
		self.vec.push(index);
		index
	}

	/// Swap remove the item at the given index.
	///
	/// Computes in **O(1)** time (average).
	#[inline]
	pub fn swap_remove(&mut self, index: Index) -> Item<K, V> {
		let item = self.items.remove(index);
		self.vec.swap_remove(item.order);

		if !self.vec.is_empty() {
			let swapped_index = self.vec[item.order];
			self.items.get_mut(swapped_index).unwrap().order = item.order;
		}

		item.unordered()
	}

	/// Shift remove the item at the given index.
	///
	/// Computes in **O(n)** time (average).
	#[inline]
	pub fn shift_remove(&mut self, index: Index) -> Item<K, V> {
		let item = self.items.remove(index);
		self.vec.remove(item.order);

		let mut i = item.order;
		for &shifted_index in &self.vec[item.order..] {
			self.items.get_mut(shifted_index).unwrap().order = i;
			i += 1;
		}

		item.unordered()
	}
}

pub struct Iter<'a, K, V> {
	inner: &'a Inner<K, V>,
	indexes: std::slice::Iter<'a, Index>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
	type Item = (&'a K, &'a V);

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.indexes.size_hint()
	}

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|index| self.inner.items.get(*index).unwrap().as_pair())
	}

	#[inline]
	fn last(mut self) -> Option<Self::Item> {
		self.next_back()
	}
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.indexes
			.next_back()
			.map(|index| self.inner.items.get(*index).unwrap().as_pair())
	}
}

impl<'a, K, V> std::iter::FusedIterator for Iter<'a, K, V> {}
impl<'a, K, V> std::iter::ExactSizeIterator for Iter<'a, K, V> {}

pub struct Keys<'a, K, V> {
	inner: &'a Inner<K, V>,
	indexes: std::slice::Iter<'a, Index>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
	type Item = &'a K;

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.indexes.size_hint()
	}

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|index| self.inner.items.get(*index).unwrap().as_key())
	}

	#[inline]
	fn last(mut self) -> Option<Self::Item> {
		self.next_back()
	}
}

impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.indexes
			.next_back()
			.map(|index| self.inner.items.get(*index).unwrap().as_key())
	}
}

impl<'a, K, V> std::iter::FusedIterator for Keys<'a, K, V> {}
impl<'a, K, V> std::iter::ExactSizeIterator for Keys<'a, K, V> {}

pub struct Values<'a, K, V> {
	inner: &'a Inner<K, V>,
	indexes: std::slice::Iter<'a, Index>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
	type Item = &'a V;

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.indexes.size_hint()
	}

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|index| self.inner.items.get(*index).unwrap().as_value())
	}

	#[inline]
	fn last(mut self) -> Option<Self::Item> {
		self.next_back()
	}
}

impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.indexes
			.next_back()
			.map(|index| self.inner.items.get(*index).unwrap().as_value())
	}
}

impl<'a, K, V> std::iter::FusedIterator for Values<'a, K, V> {}
impl<'a, K, V> std::iter::ExactSizeIterator for Values<'a, K, V> {}

pub struct IntoIter<K, V> {
	indexes: std::vec::IntoIter<Index>,
	items: SlabList<item::Ordered<K, V>>,
}

impl<K, V> Iterator for IntoIter<K, V> {
	type Item = (K, V);

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.indexes.size_hint()
	}

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|index| self.items.remove(index).into_pair())
	}

	#[inline]
	fn last(mut self) -> Option<Self::Item> {
		self.next_back()
	}
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.indexes
			.next_back()
			.map(|index| self.items.remove(index).into_pair())
	}
}

impl<'a, K, V> std::iter::FusedIterator for IntoIter<K, V> {}
impl<'a, K, V> std::iter::ExactSizeIterator for IntoIter<K, V> {}

pub struct IntoKeys<K, V> {
	indexes: std::vec::IntoIter<Index>,
	items: SlabList<item::Ordered<K, V>>,
}

impl<K, V> Iterator for IntoKeys<K, V> {
	type Item = K;

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.indexes.size_hint()
	}

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|index| self.items.remove(index).into_key())
	}

	#[inline]
	fn last(mut self) -> Option<Self::Item> {
		self.next_back()
	}
}

impl<K, V> DoubleEndedIterator for IntoKeys<K, V> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.indexes
			.next_back()
			.map(|index| self.items.remove(index).into_key())
	}
}

impl<'a, K, V> std::iter::FusedIterator for IntoKeys<K, V> {}
impl<'a, K, V> std::iter::ExactSizeIterator for IntoKeys<K, V> {}

pub struct IntoValues<K, V> {
	indexes: std::vec::IntoIter<Index>,
	items: SlabList<item::Ordered<K, V>>,
}

impl<K, V> Iterator for IntoValues<K, V> {
	type Item = V;

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.indexes.size_hint()
	}

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.indexes
			.next()
			.map(|index| self.items.remove(index).into_value())
	}

	#[inline]
	fn last(mut self) -> Option<Self::Item> {
		self.next_back()
	}
}

impl<K, V> DoubleEndedIterator for IntoValues<K, V> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.indexes
			.next_back()
			.map(|index| self.items.remove(index).into_value())
	}
}

impl<'a, K, V> std::iter::FusedIterator for IntoValues<K, V> {}
impl<'a, K, V> std::iter::ExactSizeIterator for IntoValues<K, V> {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let mut map = IndexMap::new();
		assert_eq!(map.is_empty(), true);
		map.insert(1, ());
		map.insert(1, ());
		assert_eq!(map.len(), 1);
		assert!(map.get(&1).is_some());
		assert_eq!(map.is_empty(), false);
	}

	#[test]
	fn new() {
		let map = IndexMap::<String, String>::new();
		println!("{:?}", map);
		assert_eq!(map.capacity(), 0);
		assert_eq!(map.len(), 0);
		assert_eq!(map.is_empty(), true);
	}

	#[test]
	fn insert() {
		let insert = [0, 4, 2, 12, 8, 7, 11, 5];
		let not_present = [1, 3, 6, 9, 10];
		let mut map = IndexMap::with_capacity(insert.len());

		for (i, elt) in insert.into_iter().enumerate() {
			assert_eq!(map.len(), i);
			map.insert(elt, elt);
			assert_eq!(map.len(), i + 1);
			assert_eq!(map.get(&elt), Some(&elt));
			assert_eq!(map[&elt], elt);
		}
		println!("{:?}", map);

		for &elt in &not_present {
			assert!(map.get(&elt).is_none());
		}
	}

	#[test]
	fn insert_full() {
		let insert = vec![9, 2, 7, 1, 4, 6, 13];
		let present = vec![1, 6, 2];
		let mut map = IndexMap::with_capacity(insert.len());

		for (i, elt) in insert.into_iter().enumerate() {
			assert_eq!(map.len(), i);
			let (index, existing) = map.insert_full(elt, elt);
			assert_eq!(existing, None);
			assert_eq!(Some(index), map.get_full(&elt).map(|x| x.0));
			assert_eq!(map.len(), i + 1);
		}

		let len = map.len();
		for elt in present {
			let (index, existing) = map.insert_full(elt, elt);
			assert_eq!(existing, Some(elt));
			assert_eq!(Some(index), map.get_full(&elt).map(|x| x.0));
			assert_eq!(map.len(), len);
		}
	}

	// #[test]
	// fn insert_2() {
	//     let mut map = IndexMap::with_capacity(16);

	//     let mut keys = vec![];
	//     keys.extend(0..16);
	//     keys.extend(128..267);

	//     for &i in &keys {
	//         let old_map = map.clone();
	//         map.insert(i, ());
	//         for key in old_map.keys() {
	//             if map.get(key).is_none() {
	//                 println!("old_map: {:?}", old_map);
	//                 println!("map: {:?}", map);
	//                 panic!("did not find {} in map", key);
	//             }
	//         }
	//     }

	//     for &i in &keys {
	//         assert!(map.get(&i).is_some(), "did not find {}", i);
	//     }
	// }

	// #[test]
	// fn insert_order() {
	//     let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
	//     let mut map = IndexMap::new();

	//     for &elt in &insert {
	//         map.insert(elt, ());
	//     }

	//     assert_eq!(map.keys().count(), map.len());
	//     assert_eq!(map.keys().count(), insert.len());
	//     for (a, b) in insert.iter().zip(map.keys()) {
	//         assert_eq!(a, b);
	//     }
	//     for (i, k) in (0..insert.len()).zip(map.keys()) {
	//         assert_eq!(map.get_index(i).unwrap().0, k);
	//     }
	// }

	// #[test]
	// fn grow() {
	//     let insert = [0, 4, 2, 12, 8, 7, 11];
	//     let not_present = [1, 3, 6, 9, 10];
	//     let mut map = IndexMap::with_capacity(insert.len());

	//     for (i, &elt) in enumerate(&insert) {
	//         assert_eq!(map.len(), i);
	//         map.insert(elt, elt);
	//         assert_eq!(map.len(), i + 1);
	//         assert_eq!(map.get(&elt), Some(&elt));
	//         assert_eq!(map[&elt], elt);
	//     }

	//     println!("{:?}", map);
	//     for &elt in &insert {
	//         map.insert(elt * 10, elt);
	//     }
	//     for &elt in &insert {
	//         map.insert(elt * 100, elt);
	//     }
	//     for (i, &elt) in insert.iter().cycle().enumerate().take(100) {
	//         map.insert(elt * 100 + i as i32, elt);
	//     }
	//     println!("{:?}", map);
	//     for &elt in &not_present {
	//         assert!(map.get(&elt).is_none());
	//     }
	// }

	// #[test]
	// fn reserve() {
	//     let mut map = IndexMap::<usize, usize>::new();
	//     assert_eq!(map.capacity(), 0);
	//     map.reserve(100);
	//     let capacity = map.capacity();
	//     assert!(capacity >= 100);
	//     for i in 0..capacity {
	//         assert_eq!(map.len(), i);
	//         map.insert(i, i * i);
	//         assert_eq!(map.len(), i + 1);
	//         assert_eq!(map.capacity(), capacity);
	//         assert_eq!(map.get(&i), Some(&(i * i)));
	//     }
	//     map.insert(capacity, std::usize::MAX);
	//     assert_eq!(map.len(), capacity + 1);
	//     assert!(map.capacity() > capacity);
	//     assert_eq!(map.get(&capacity), Some(&std::usize::MAX));
	// }

	// #[test]
	// fn shrink_to_fit() {
	//     let mut map = IndexMap::<usize, usize>::new();
	//     assert_eq!(map.capacity(), 0);
	//     for i in 0..100 {
	//         assert_eq!(map.len(), i);
	//         map.insert(i, i * i);
	//         assert_eq!(map.len(), i + 1);
	//         assert!(map.capacity() >= i + 1);
	//         assert_eq!(map.get(&i), Some(&(i * i)));
	//         map.shrink_to_fit();
	//         assert_eq!(map.len(), i + 1);
	//         assert_eq!(map.capacity(), i + 1);
	//         assert_eq!(map.get(&i), Some(&(i * i)));
	//     }
	// }

	// #[test]
	// fn remove() {
	//     let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
	//     let mut map = IndexMap::new();

	//     for &elt in &insert {
	//         map.insert(elt, elt);
	//     }

	//     assert_eq!(map.keys().count(), map.len());
	//     assert_eq!(map.keys().count(), insert.len());
	//     for (a, b) in insert.iter().zip(map.keys()) {
	//         assert_eq!(a, b);
	//     }

	//     let remove_fail = [99, 77];
	//     let remove = [4, 12, 8, 7];

	//     for &key in &remove_fail {
	//         assert!(map.swap_remove_full(&key).is_none());
	//     }
	//     println!("{:?}", map);
	//     for &key in &remove {
	//         //println!("{:?}", map);
	//         let index = map.get_full(&key).unwrap().0;
	//         assert_eq!(map.swap_remove_full(&key), Some((index, key, key)));
	//     }
	//     println!("{:?}", map);

	//     for key in &insert {
	//         assert_eq!(map.get(key).is_some(), !remove.contains(key));
	//     }
	//     assert_eq!(map.len(), insert.len() - remove.len());
	//     assert_eq!(map.keys().count(), insert.len() - remove.len());
	// }

	// #[test]
	// fn remove_to_empty() {
	//     let mut map = indexmap! { 0 => 0, 4 => 4, 5 => 5 };
	//     map.swap_remove(&5).unwrap();
	//     map.swap_remove(&4).unwrap();
	//     map.swap_remove(&0).unwrap();
	//     assert!(map.is_empty());
	// }

	// #[test]
	// fn swap_remove_index() {
	//     let insert = [0, 4, 2, 12, 8, 7, 11, 5, 3, 17, 19, 22, 23];
	//     let mut map = IndexMap::new();

	//     for &elt in &insert {
	//         map.insert(elt, elt * 2);
	//     }

	//     let mut vector = insert.to_vec();
	//     let remove_sequence = &[3, 3, 10, 4, 5, 4, 3, 0, 1];

	//     // check that the same swap remove sequence on vec and map
	//     // have the same result.
	//     for &rm in remove_sequence {
	//         let out_vec = vector.swap_remove(rm);
	//         let (out_map, _) = map.swap_remove_index(rm).unwrap();
	//         assert_eq!(out_vec, out_map);
	//     }
	//     assert_eq!(vector.len(), map.len());
	//     for (a, b) in vector.iter().zip(map.keys()) {
	//         assert_eq!(a, b);
	//     }
	// }

	// #[test]
	// fn partial_eq_and_eq() {
	//     let mut map_a = IndexMap::new();
	//     map_a.insert(1, "1");
	//     map_a.insert(2, "2");
	//     let mut map_b = map_a.clone();
	//     assert_eq!(map_a, map_b);
	//     map_b.swap_remove(&1);
	//     assert_ne!(map_a, map_b);

	//     let map_c: IndexMap<_, String> = map_b.into_iter().map(|(k, v)| (k, v.into())).collect();
	//     assert_ne!(map_a, map_c);
	//     assert_ne!(map_c, map_a);
	// }

	// #[test]
	// fn extend() {
	//     let mut map = IndexMap::new();
	//     map.extend(vec![(&1, &2), (&3, &4)]);
	//     map.extend(vec![(5, 6)]);
	//     assert_eq!(
	//         map.into_iter().collect::<Vec<_>>(),
	//         vec![(1, 2), (3, 4), (5, 6)]
	//     );
	// }

	// #[test]
	// fn entry() {
	//     let mut map = IndexMap::new();

	//     map.insert(1, "1");
	//     map.insert(2, "2");
	//     {
	//         let e = map.entry(3);
	//         assert_eq!(e.index(), 2);
	//         let e = e.or_insert("3");
	//         assert_eq!(e, &"3");
	//     }

	//     let e = map.entry(2);
	//     assert_eq!(e.index(), 1);
	//     assert_eq!(e.key(), &2);
	//     match e {
	//         Entry::Occupied(ref e) => assert_eq!(e.get(), &"2"),
	//         Entry::Vacant(_) => panic!(),
	//     }
	//     assert_eq!(e.or_insert("4"), &"2");
	// }

	// #[test]
	// fn entry_and_modify() {
	//     let mut map = IndexMap::new();

	//     map.insert(1, "1");
	//     map.entry(1).and_modify(|x| *x = "2");
	//     assert_eq!(Some(&"2"), map.get(&1));

	//     map.entry(2).and_modify(|x| *x = "doesn't exist");
	//     assert_eq!(None, map.get(&2));
	// }

	// #[test]
	// fn entry_or_default() {
	//     let mut map = IndexMap::new();

	//     #[derive(Debug, PartialEq)]
	//     enum TestEnum {
	//         DefaultValue,
	//         NonDefaultValue,
	//     }

	//     impl Default for TestEnum {
	//         fn default() -> Self {
	//             TestEnum::DefaultValue
	//         }
	//     }

	//     map.insert(1, TestEnum::NonDefaultValue);
	//     assert_eq!(&mut TestEnum::NonDefaultValue, map.entry(1).or_default());

	//     assert_eq!(&mut TestEnum::DefaultValue, map.entry(2).or_default());
	// }

	// #[test]
	// fn occupied_entry_key() {
	//     // These keys match hash and equality, but their addresses are distinct.
	//     let (k1, k2) = (&mut 1, &mut 1);
	//     let k1_ptr = k1 as *const i32;
	//     let k2_ptr = k2 as *const i32;
	//     assert_ne!(k1_ptr, k2_ptr);

	//     let mut map = IndexMap::new();
	//     map.insert(k1, "value");
	//     match map.entry(k2) {
	//         Entry::Occupied(ref e) => {
	//             // `OccupiedEntry::key` should reference the key in the map,
	//             // not the key that was used to find the entry.
	//             let ptr = *e.key() as *const i32;
	//             assert_eq!(ptr, k1_ptr);
	//             assert_ne!(ptr, k2_ptr);
	//         }
	//         Entry::Vacant(_) => panic!(),
	//     }
	// }

	// #[test]
	// fn keys() {
	//     let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
	//     let map: IndexMap<_, _> = vec.into_iter().collect();
	//     let keys: Vec<_> = map.keys().copied().collect();
	//     assert_eq!(keys.len(), 3);
	//     assert!(keys.contains(&1));
	//     assert!(keys.contains(&2));
	//     assert!(keys.contains(&3));
	// }

	// #[test]
	// fn values() {
	//     let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
	//     let map: IndexMap<_, _> = vec.into_iter().collect();
	//     let values: Vec<_> = map.values().copied().collect();
	//     assert_eq!(values.len(), 3);
	//     assert!(values.contains(&'a'));
	//     assert!(values.contains(&'b'));
	//     assert!(values.contains(&'c'));
	// }

	// #[test]
	// fn values_mut() {
	//     let vec = vec![(1, 1), (2, 2), (3, 3)];
	//     let mut map: IndexMap<_, _> = vec.into_iter().collect();
	//     for value in map.values_mut() {
	//         *value *= 2
	//     }
	//     let values: Vec<_> = map.values().copied().collect();
	//     assert_eq!(values.len(), 3);
	//     assert!(values.contains(&2));
	//     assert!(values.contains(&4));
	//     assert!(values.contains(&6));
	// }
}
