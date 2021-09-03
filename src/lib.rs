#![feature(generic_associated_types)]
use std::borrow::Borrow;
use slab::Slab;
use slab_lists::SlabList;
use generic_btree::{
	Storage,
	StorageMut
};

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

/// BTree order. Must be at least 
const M: usize = 8;

pub struct BTreeIndexMap<K, V> {
	nodes: Slab<Node>,
	inner: Inner<K, V>,
	root: Option<usize>
}

impl<K, V> BTreeIndexMap<K, V> {
	fn btree(&self) -> Ref<K, V> {
		Ref::new(&self.nodes, &self.inner, self.root)
	}

	fn btree_mut(&mut self) -> Mut<K, V> {
		Mut::new(&mut self.nodes, &mut self.inner, &mut self.root)
	}

	/// Get by key.
	/// 
	/// ## Complexity
	/// 
	/// O(log(n))
	pub fn get<'a, Q: ?Sized>(&'a self, key: &Q) -> Option<&'a V> where K: Borrow<Q>, Q: Ord, Self: 'a {
		let btree: Ref<'a, K, V> = self.btree();
		let item: Option<index::Ref<'_, K, V>> = btree.get_item(key);
		item.map(|index_ref| {
			let index = index_ref.index();
			self.inner.items.get(index).map(|item| &item.value)
		}).flatten()
	}

	/// Get by index.
	/// 
	/// ## Complexity
	/// 
	/// O(1)
	pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
		self.inner.vec.get(index).map(|i| self.inner.items.get(*i)).flatten().map(|item| item.as_pair())
	}

	/// Insert value.
	pub fn insert(&mut self, key: K, value: V) -> Option<V> where K: Ord {
		self.btree_mut().insert(Item::new(key, value))
	}

	/// Remove value.
	/// 
	/// This is an alias to [`Self::swap_remove`],
	/// meaning that this function *disturbs the order of the map*.
	/// Use [`Self::shift_remove`] to remove the value and preserve the order
	/// by sacrificing the average computation time.
	/// 
	/// Computes in **O(log n)** time (average).
	pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q>, Q: Ord {
		self.swap_remove(key)
	}

	/// Remove value.
	/// 
	/// Computes in **O(log n)** time (average).
	pub fn swap_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q>, Q: Ord {
		match self.btree_mut().remove(key) {
			Some(index) => {
				let item = self.inner.swap_remove(index);
				Some(item.value)
			}
			None => None
		}
	}

	/// Remove value.
	/// 
	/// Computes in **O(n)** time (average).
	pub fn shift_remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q>, Q: Ord {
		match self.btree_mut().remove(key) {
			Some(index) => {
				let item = self.inner.shift_remove(index);
				Some(item.value)
			}
			None => None
		}
	}
}

pub struct Inner<K, V> {
	/// Items.
	items: SlabList<item::Ordered<K, V>>,

	/// Ordering array.
	vec: Vec<Index>
}

impl<K, V> Inner<K, V> {
	/// Insert the given item.
	/// 
	/// /// Computes in **O(1)** time (average).
	pub fn insert(&mut self, item: Item<K, V>) -> Index {
		let order = self.vec.len();
		let index = self.items.push_back(item.order(order));
		self.vec.push(index);
		index
	}

	/// Swap remove the item at the given index.
	/// 
	/// Computes in **O(1)** time (average).
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
	/// /// Computes in **O(n)** time (average).
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