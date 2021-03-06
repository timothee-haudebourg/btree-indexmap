use super::{index, node, Index, Inner, Item, Node};
use generic_btree::{Storage, StorageMut};
use slab::Slab;
use std::{borrow::Borrow, cmp::Ordering};

pub struct Mut<'a, K, V> {
	nodes: &'a mut Slab<Node>,
	inner: &'a mut Inner<K, V>,
	root: &'a mut Option<usize>,
}

impl<'a, K, V> Mut<'a, K, V> {
	pub(crate) fn new(
		nodes: &'a mut Slab<Node>,
		inner: &'a mut Inner<K, V>,
		root: &'a mut Option<usize>,
	) -> Self {
		Self { nodes, inner, root }
	}
}

impl<'a, K, V> Storage for Mut<'a, K, V> {
	type ItemRef<'r>
	where
		Self: 'r,
	= index::Ref<'r, K, V>;
	type LeafRef<'r>
	where
		Self: 'r,
	= node::leaf::Ref<'r, K, V>;
	type InternalRef<'r>
	where
		Self: 'r,
	= node::internal::Ref<'r, K, V>;

	fn root(&self) -> Option<usize> {
		*self.root
	}

	fn len(&self) -> usize {
		self.inner.vec.len()
	}

	fn node(&self, id: usize) -> Option<generic_btree::node::Ref<'_, Self>> {
		match self.nodes.get(id) {
			Some(Node::Leaf(leaf)) => Some(generic_btree::node::Ref::leaf(node::leaf::Ref::new(
				leaf, self.inner,
			))),
			Some(Node::Internal(internal)) => Some(generic_btree::node::Ref::internal(
				node::internal::Ref::new(internal, self.inner),
			)),
			None => None,
		}
	}
}

unsafe impl<'a, K, V> StorageMut for Mut<'a, K, V> {
	type Item = Index;
	type LeafNode = node::leaf::Metadata;
	type InternalNode = node::internal::Metadata;

	type ItemMut<'r>
	where
		Self: 'r,
	= index::Mut<'r, K, V>;
	type LeafMut<'r>
	where
		Self: 'r,
	= node::leaf::Mut<'r, K, V>;
	type InternalMut<'r>
	where
		Self: 'r,
	= node::internal::Mut<'r, K, V>;

	/// Sets the roo node by id.
	fn set_root(&mut self, root: Option<usize>) {
		*self.root = root
	}

	/// Update the length of the B-Tree.
	fn set_len(&mut self, _new_len: usize) {
		// Nothing to do here,
		// we can deduce the length ourselves using `inner.vec.len()`.
	}

	fn allocate_node(&mut self, node: generic_btree::node::Buffer<Self>) -> usize {
		let node = match node {
			generic_btree::node::Buffer::Leaf(leaf) => Node::Leaf(leaf),
			generic_btree::node::Buffer::Internal(internal) => Node::Internal(internal),
		};

		self.nodes.insert(node)
	}

	fn release_node(&mut self, id: usize) -> generic_btree::node::Buffer<Self> {
		match self.nodes.remove(id) {
			Node::Leaf(leaf) => generic_btree::node::Buffer::Leaf(leaf),
			Node::Internal(internal) => generic_btree::node::Buffer::Internal(internal),
		}
	}

	/// Returns the node with the given id, if any.
	fn node_mut(&mut self, id: usize) -> Option<generic_btree::node::Mut<'_, Self>> {
		let inner = &mut self.inner;
		self.nodes.get_mut(id).map(move |node| match node {
			Node::Leaf(leaf) => generic_btree::node::Mut::leaf(node::leaf::Mut::new(leaf, inner)),
			Node::Internal(internal) => {
				generic_btree::node::Mut::internal(node::internal::Mut::new(internal, inner))
			}
		})
	}
}

impl<'a, K, V> generic_btree::Insert<Item<K, V>> for Mut<'a, K, V> {
	fn allocate_item(&mut self, item: Item<K, V>) -> Index {
		self.inner.insert(item)
	}
}

impl<'a, K, V, Q: ?Sized> generic_btree::KeyPartialOrd<Q> for crate::Mut<'a, K, V>
where
	K: Borrow<Q>,
	Q: Ord,
{
	fn key_partial_cmp<'r>(index_ref: &index::Ref<'r, K, V>, key: &Q) -> Option<Ordering>
	where
		Self: 'r,
	{
		index_ref.item().key.borrow().partial_cmp(key)
	}
}

impl<'a, K, V> generic_btree::KeyPartialOrd<Item<K, V>> for crate::Mut<'a, K, V>
where
	K: PartialOrd,
{
	fn key_partial_cmp<'r>(index_ref: &index::Ref<'r, K, V>, other: &Item<K, V>) -> Option<Ordering>
	where
		Self: 'r,
	{
		index_ref.item().key.partial_cmp(&other.key)
	}
}
