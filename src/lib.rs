#![feature(generic_associated_types)]
use std::{
	borrow::Borrow,
	cmp::Ordering
};
use smallvec::SmallVec;
use slab::Slab;
use slab_lists::SlabList;
use generic_btree::btree::{
	Storage,
	StorageMut,
	node::{
		Offset,
		// Item
	}
};

pub struct Item<K, V> {
	key: K,
	value: V
}

impl<K, V> Item<K, V> {
	pub fn new(key: K, value: V) -> Self {
		Self {
			key, value
		}
	}

	pub fn order(self, order: usize) -> OrderedItem<K, V> {
		OrderedItem {
			key: self.key,
			value: self.value,
			order
		}
	}
}

pub struct OrderedItem<K, V> {
	key: K,
	value: V,
	order: usize
}

impl<K, V> OrderedItem<K, V> {
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

const M: usize = 8;

/// Item index.
pub type Index = usize;

pub struct DetachedBranch<K, V> {
	item: Item<K, V>,
	child_id: usize
}

pub struct DetachedInternal<K, V> {
	parent: Option<usize>,
	first_child_id: usize,
	branches: SmallVec<[DetachedBranch<K, V>; M]>
}

impl<K, V> Default for DetachedInternal<K, V> {
	fn default() -> Self {
		Self {
			parent: None,
			first_child_id: 0,
			branches: SmallVec::new()
		}
	}
}

impl<'a, K, V> generic_btree::btree::node::buffer::Internal<BTreeIndexMapMut<'a, K, V>> for DetachedInternal<K, V> {
	fn parent(&self) -> Option<usize> {
		self.parent
	}

	fn set_parent(&mut self, parent: Option<usize>) {
		self.parent = parent
	}

	fn item_count(&self) -> usize {
		self.branches.len()
	}

	fn item<'r>(&'r self, offset: Offset) -> Option<IndexRef<'r, K, V>> where 'a: 'r {
		panic!("TODO")
	}

	fn child_id(&self, index: usize) -> Option<usize> {
		panic!("TODO")
	}

	fn max_capacity(&self) -> usize {
		M
	}

	fn set_first_child(&mut self, id: usize) {
		self.first_child_id = id
	}

	fn push_right(&mut self, item: Index, child: usize) {
		panic!("TODO")
	}

	fn forget(self) {
		std::mem::forget(self.branches)
	}
}

pub struct Branch {
	item_index: Index,
	child_id: usize
}

pub struct Internal {
	parent: Option<usize>,
	first_child_id: usize,
	branches: SmallVec<[Branch; M]>
}

pub struct InternalRef<'a, K, V> {
	internal: &'a Internal,
	inner: &'a Inner<K, V>
}

macro_rules! impl_both {
	([ $($params:tt)* ] ($tr1:path, $tr2:path) for $ty:ty { $($impl:tt)* }) => {
		impl < $($params)* > $tr1 for $ty { $($impl)* }
		impl < $($params)* > $tr2 for $ty { $($impl)* }
	};
}

impl_both! {
	[ 'r, 'a: 'r, K, V ]
	(
		generic_btree::btree::node::ItemAccess<BTreeIndexMapRef<'a, K, V>>,
		generic_btree::btree::node::ItemAccess<BTreeIndexMapMut<'a, K, V>>
	) 
	for InternalRef<'r, K, V> {
		fn item_count(&self) -> usize {
			self.internal.branches.len()
		}
	
		fn borrow_item(&self, offset: Offset) -> Option<IndexRef<'r, K, V>> {
			panic!("TODO")
		}
	}
}

impl_both! {
	[ 'r, 'a: 'r, K, V ]
	(
		generic_btree::btree::node::InternalRef<BTreeIndexMapRef<'a, K, V>>,
		generic_btree::btree::node::InternalRef<BTreeIndexMapMut<'a, K, V>>
	)
	for InternalRef<'r, K, V> {    // ...
		fn parent(&self) -> Option<usize> {
			self.internal.parent
		}

		fn child_id(&self, index: usize) -> Option<usize> {
			panic!("TODO")
		}

		fn max_capacity(&self) -> usize {
			M
		}
	}
}

impl_both! {
	[ 'r, 'a: 'r, K, V ]
	(
		generic_btree::btree::node::InternalConst<'r, BTreeIndexMapRef<'a, K, V>>,
		generic_btree::btree::node::InternalConst<'r, BTreeIndexMapMut<'a, K, V>>
	)
	for InternalRef<'r, K, V> {    // ...
		fn item(&self, offset: Offset) -> Option<IndexRef<'r, K, V>> {
			panic!("TODO")
		}
	}
}

pub struct DetachedLeaf<K, V> {
	parent: Option<usize>,
	branches: SmallVec<[Item<K, V>; M]>
}

impl<K, V> Default for DetachedLeaf<K, V> {
	fn default() -> Self {
		Self {
			parent: None,
			branches: SmallVec::new()
		}
	}
}

impl<'a, K, V> generic_btree::btree::node::buffer::Leaf<BTreeIndexMapMut<'a, K, V>> for DetachedLeaf<K, V> {
	fn parent(&self) -> Option<usize> {
		self.parent
	}

	fn set_parent(&mut self, parent: Option<usize>) {
		self.parent = parent
	}

	fn item_count(&self) -> usize {
		self.branches.len()
	}

	fn item<'r>(&self, offset: Offset) -> Option<IndexRef<'a, K, V>> where 'a: 'r {
		panic!("TODO")
	}

	fn max_capacity(&self) -> usize {
		M
	}

	fn push_right(&mut self, index: Index) {
		panic!("TODO")
	}

	fn forget(self) {
		std::mem::forget(self.branches)
	}
}

pub struct Leaf {
	parent: Option<usize>,
	items: SmallVec<[Index; M+1]>
}

pub struct LeafRef<'a, K, V> {
	leaf: &'a Leaf,
	inner: &'a Inner<K, V>
}

impl_both! {
	[ 'r, 'a: 'r, K, V ]
	(
		generic_btree::btree::node::ItemAccess<BTreeIndexMapRef<'a, K, V>>,
		generic_btree::btree::node::ItemAccess<BTreeIndexMapMut<'a, K, V>>
	) 
	for LeafRef<'r, K, V> {
		fn item_count(&self) -> usize {
			self.leaf.items.len()
		}
	
		fn borrow_item(&self, offset: Offset) -> Option<IndexRef<'r, K, V>> {
			panic!("TODO")
		}
	}
}

impl_both! {
	[ 'r, 'a: 'r, K, V ]
	(
		generic_btree::btree::node::LeafRef<BTreeIndexMapRef<'a, K, V>>,
		generic_btree::btree::node::LeafRef<BTreeIndexMapMut<'a, K, V>>
	) 
	for LeafRef<'r, K, V> {
		fn parent(&self) -> Option<usize> {
			self.leaf.parent
		}
	
		fn max_capacity(&self) -> usize {
			M+1
		}
	}
}

impl_both! {
	[ 'r, 'a: 'r, K, V ]
	(
		generic_btree::btree::node::LeafConst<'r, BTreeIndexMapRef<'a, K, V>>,
		generic_btree::btree::node::LeafConst<'r, BTreeIndexMapMut<'a, K, V>>
	) 
	for LeafRef<'r, K, V> {
		fn item(&self, offset: Offset) -> Option<IndexRef<'r, K, V>> {
			panic!("TODO")
		}
	}
}

pub struct InternalMut<'a, K, V> {
	internal: &'a mut Internal,
	inner: &'a mut Inner<K, V>
}

impl<'r, 'a: 'r, K, V> generic_btree::btree::node::ItemAccess<BTreeIndexMapMut<'a, K, V>> for InternalMut<'r, K, V> {
	fn item_count(&self) -> usize {
		self.internal.branches.len()
	}

	fn borrow_item(&self, offset: Offset) -> Option<IndexRef<'r, K, V>> {
		panic!("TODO")
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::btree::node::InternalRef<BTreeIndexMapMut<'a, K, V>> for InternalMut<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.internal.parent
	}

	fn child_id(&self, index: usize) -> Option<usize> {
		panic!("TODO")
	}

	fn max_capacity(&self) -> usize {
		M
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::btree::node::InternalMut<'r, BTreeIndexMapMut<'a, K, V>> for InternalMut<'r, K, V> {
	fn set_parent(&mut self, parent: Option<usize>) {
		self.internal.parent = parent
	}

	fn set_first_child(&mut self, id: usize) {
		self.internal.first_child_id = id
	}

	fn into_item_mut(self, offset: Offset) -> Option<IndexMut<'a, K, V>> {
		panic!("TODO")
	}

	fn insert(&mut self, offset: Offset, item: Index, right_child_id: usize) {
		panic!("TODO")
	}

	fn remove(&mut self, offset: Offset) -> (Index, usize) {
		panic!("TODO")
	}

	fn replace(&mut self, offset: Offset, item: Index) -> Index {
		panic!("TODO")
	}

	fn append(&mut self, separator: Index, other: DetachedInternal<K, V>) -> Offset {
		panic!("TODO")
	}
}

pub struct LeafMut<'a, K, V> {
	leaf: &'a mut Leaf,
	items: &'a mut SlabList<OrderedItem<K, V>>
}

impl<'r, 'a: 'r, K, V> generic_btree::btree::node::ItemAccess<BTreeIndexMapMut<'a, K, V>> for LeafMut<'r, K, V> {
	fn item_count(&self) -> usize {
		self.leaf.items.len()
	}

	fn borrow_item(&self, offset: Offset) -> Option<IndexRef<'r, K, V>> {
		panic!("TODO")
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::btree::node::LeafRef<BTreeIndexMapMut<'a, K, V>> for LeafMut<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.leaf.parent
	}

	fn max_capacity(&self) -> usize {
		M+1
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::btree::node::LeafMut<'r, BTreeIndexMapMut<'a, K, V>> for LeafMut<'r, K, V> {
	fn set_parent(&mut self, parent: Option<usize>) {
		self.leaf.parent = parent
	}

	/// Returns a mutable reference to the item with the given offset in the node.
	fn item_mut(&mut self, offset: Offset) -> Option<IndexMut<'_, K, V>> {
		panic!("TODO")
	}

	fn into_item_mut(self, offset: Offset) -> Option<IndexMut<'a, K, V>> {
		panic!("TODO")
	}

	fn insert(&mut self, offset: Offset, item: Index) {
		// let index = self.items.push_back(item);
		panic!("TODO")
	}

	fn remove(&mut self, offset: Offset) -> Index {
		panic!("TODO")
	}

	fn append(&mut self, separator: Index, other: DetachedLeaf<K, V>) -> Offset {
		panic!("TODO")
	}
}

pub enum Node {
	Internal(Internal),
	Leaf(Leaf)
}

// pub struct Item<K, V> {
//     key: K,
//     value: V
// }

// impl<K, V> Item<K, V> {
//     pub fn as_pair(&self) -> (&K, &V) {
//         (&self.key, &self.value)
//     }
// }

impl<'r, 'a: 'r, K, V> generic_btree::btree::node::item::Mut<BTreeIndexMapMut<'a, K, V>> for &'r mut Item<K, V> {
	fn swap(&mut self, item: &mut Index) {
		panic!("TODO")
	}
}

pub struct Inner<K, V> {
	/// Items.
	items: SlabList<OrderedItem<K, V>>,

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

pub struct BTreeIndexMap<K, V> {
	nodes: Slab<Node>,
	inner: Inner<K, V>
}

impl<K, V> BTreeIndexMap<K, V> {
	fn btree(&self) -> BTreeIndexMapRef<K, V> {
		BTreeIndexMapRef {
			nodes: &self.nodes,
			inner: &self.inner
		}
	}

	fn btree_mut(&mut self) -> BTreeIndexMapMut<K, V> {
		BTreeIndexMapMut {
			nodes: &mut self.nodes,
			inner: &mut self.inner
		}
	}

	/// Get by key.
	/// 
	/// ## Complexity
	/// 
	/// O(log(n))
	pub fn get<'a, Q: ?Sized>(&'a self, key: &Q) -> Option<&'a V> where K: Borrow<Q>, Q: Ord, Self: 'a {
		let btree: BTreeIndexMapRef<'a, K, V> = self.btree();
		let item: Option<IndexRef<'_, K, V>> = btree.get_item(key);
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

struct BTreeIndexMapRef<'a, K, V> {
	nodes: &'a Slab<Node>,
	inner: &'a Inner<K, V>,
	root: Option<usize>
}

impl<'a, K, V> Storage for BTreeIndexMapRef<'a, K, V> {
	type ItemRef<'r> where Self: 'r = IndexRef<'r, K, V>;
	type LeafRef<'r> where Self: 'r = LeafRef<'r, K, V>;
	type InternalRef<'r> where Self: 'r = InternalRef<'r, K, V>;

	fn root(&self) -> Option<usize> {
		self.root
	}

	fn len(&self) -> usize {
		self.inner.vec.len()
	}

	fn node<'r>(&'r self, id: usize) -> Option<generic_btree::btree::node::Ref<'r, Self>> { // Self: 'r => 'a: 'r
		match self.nodes.get(id) {
			Some(Node::Leaf(leaf)) => Some(generic_btree::btree::node::Ref::leaf(LeafRef {
				leaf,
				inner: &self.inner
			})),
			Some(Node::Internal(internal)) => Some(generic_btree::btree::node::Ref::internal(InternalRef {
				internal,
				inner: &self.inner
			})),
			None => None
		}
	}
}

struct BTreeIndexMapMut<'a, K, V> {
	nodes: &'a mut Slab<Node>,
	inner: &'a mut Inner<K, V>,
	root: &'a mut Option<usize>
}

impl<'a, K, V> Storage for BTreeIndexMapMut<'a, K, V> {
	type ItemRef<'r> where Self: 'r = IndexRef<'r, K, V>;
	type LeafRef<'r> where Self: 'r = LeafRef<'r, K, V>;
	type InternalRef<'r> where Self: 'r = InternalRef<'r, K, V>;

	fn root(&self) -> Option<usize> {
		*self.root
	}

	fn len(&self) -> usize {
		self.inner.vec.len()
	}

	fn node(&self, id: usize) -> Option<generic_btree::btree::node::Ref<'_, Self>> {
		match self.nodes.get(id) {
			Some(Node::Leaf(leaf)) => Some(generic_btree::btree::node::Ref::leaf(LeafRef {
				leaf,
				inner: &self.inner
			})),
			Some(Node::Internal(internal)) => Some(generic_btree::btree::node::Ref::internal(InternalRef {
				internal,
				inner: &self.inner
			})),
			None => None
		}
	}
}

unsafe impl<'a, K, V> StorageMut for BTreeIndexMapMut<'a, K, V> {
	type Item = Index;
	type LeafNode = DetachedLeaf<K, V>;
	type InternalNode = DetachedInternal<K, V>;

	type ItemMut<'r> where Self: 'r = IndexMut<'r, K, V>;
	type LeafMut<'r> where Self: 'r = LeafMut<'r, K, V>;
	type InternalMut<'r> where Self: 'r = InternalMut<'r, K, V>;

	/// Sets the roo node by id.
	fn set_root(&mut self, root: Option<usize>) {
		*self.root = root
	}

	/// Update the length of the B-Tree.
	fn set_len(&mut self, _new_len: usize) {
		// Nothing to do here,
		// we can deduce the length ourselves using `inner.vec.len()`.
	}

	fn allocate_node(&mut self, node: generic_btree::btree::node::Buffer<Self>) -> usize {
		panic!("TODO")
	}

	fn release_node(&mut self, id: usize) -> generic_btree::btree::node::Buffer<Self> {
		panic!("TODO")
	}

	/// Returns the node with the given id, if any.
	fn node_mut(&mut self, id: usize) -> Option<generic_btree::btree::node::Mut<'_, Self>> {
		panic!("TODO")
	}
}

impl<'a, K, V> generic_btree::btree::Insert<Item<K, V>> for BTreeIndexMapMut<'a, K, V> {
	fn allocate_item(&mut self, item: Item<K, V>) -> Index {
		self.inner.insert(item)
	}
}

pub struct IndexRef<'a, K, V> {
	index: Index,
	inner: &'a Inner<K, V>
}

impl<'a, K, V> IndexRef<'a, K, V> {
	pub fn index(&self) -> Index {
		self.index
	}
}

impl<'a, K, V> generic_btree::btree::ItemOrd for BTreeIndexMapRef<'a, K, V> where K: Ord {
	fn item_cmp<'r, 's>(item: &IndexRef<'r, K, V>, other: &IndexRef<'s, K, V>) -> Ordering where Self: 'r + 's {
		panic!("TODO")
	}
}

impl<'a, K, V> generic_btree::btree::ItemOrd for BTreeIndexMapMut<'a, K, V> where K: Ord {
	fn item_cmp<'r, 's>(item: &IndexRef<'r, K, V>, other: &IndexRef<'s, K, V>) -> Ordering where Self: 'r + 's {
		panic!("TODO")
	}
}

impl<'a, K, V, Q: ?Sized> generic_btree::btree::ItemPartialOrd<Q> for BTreeIndexMapRef<'a, K, V> where K: Borrow<Q>, Q: Ord {
	fn item_partial_cmp<'r>(item: &IndexRef<'r, K, V>, key: &Q) -> Option<Ordering> where Self: 'r {
		panic!("TODO")
	}
}

impl<'a, K, V, Q: ?Sized> generic_btree::btree::ItemPartialOrd<Q> for BTreeIndexMapMut<'a, K, V> where K: Borrow<Q>, Q: Ord {
	fn item_partial_cmp<'r>(item: &IndexRef<'r, K, V>, key: &Q) -> Option<Ordering> where Self: 'r {
		panic!("TODO")
	}
}

impl<'a, K, V> generic_btree::btree::ItemPartialOrd<Item<K, V>> for BTreeIndexMapRef<'a, K, V> where K: Ord {
	fn item_partial_cmp<'r>(item: &IndexRef<'r, K, V>, other: &Item<K, V>) -> Option<Ordering> where Self: 'r {
		panic!("TODO")
	}
}

impl<'a, K, V> generic_btree::btree::ItemPartialOrd<Item<K, V>> for BTreeIndexMapMut<'a, K, V> where K: Ord {
	fn item_partial_cmp<'r>(item: &IndexRef<'r, K, V>, other: &Item<K, V>) -> Option<Ordering> where Self: 'r {
		panic!("TODO")
	}
}

pub struct IndexMut<'a, K, V> {
	index: Index,
	inner: &'a mut Inner<K, V>
}

impl<'r, 'a, K, V> generic_btree::btree::node::item::Mut<BTreeIndexMapMut<'r, K, V>> for IndexMut<'a, K, V> {
	fn swap(&mut self, index: &mut Index) {
		std::mem::swap(&mut self.index, index)
	}
}

impl<'a, K, V> generic_btree::btree::node::item::Replace<BTreeIndexMapMut<'a, K, V>, Item<K, V>> for IndexMut<'a, K, V> {
	type Output = V;

	fn replace(&mut self, item: Item<K, V>) -> V {
		self.inner.items.get_mut(self.index).unwrap().replace_value(item.value)
	}
}