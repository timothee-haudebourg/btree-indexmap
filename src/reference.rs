use super::{index, node, Inner, Item, Node};
use generic_btree::Storage;
use slab::Slab;
use std::{borrow::Borrow, cmp::Ordering};

pub struct Ref<'a, K, V> {
	nodes: &'a Slab<Node>,
	inner: &'a Inner<K, V>,
	root: Option<usize>,
}

impl<'a, K, V> Ref<'a, K, V> {
	pub(crate) fn new(nodes: &'a Slab<Node>, inner: &'a Inner<K, V>, root: Option<usize>) -> Self {
		Self { nodes, inner, root }
	}
}

impl<'a, K, V> Storage for Ref<'a, K, V> {
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
		self.root
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

impl<'a, K, V, Q: ?Sized> generic_btree::KeyPartialOrd<Q> for crate::Ref<'a, K, V>
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

impl<'a, K, V> generic_btree::KeyPartialOrd<Item<K, V>> for crate::Ref<'a, K, V>
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

impl<'a, 'b, K1, V1, K2, V2> generic_btree::ItemPartialOrd<crate::Ref<'b, K2, V2>>
	for crate::Ref<'a, K1, V1>
where
	K1: PartialOrd<K2>,
	V1: PartialOrd<V2>,
{
	fn item_partial_cmp<'r, 's>(
		index_ref: &index::Ref<'r, K1, V1>,
		other: &index::Ref<'s, K2, V2>,
	) -> Option<Ordering>
	where
		Self: 'r,
		'b: 's,
	{
		index_ref.partial_cmp(other)
	}
}

impl<'a, K, V> generic_btree::ItemOrd for crate::Ref<'a, K, V>
where
	K: Ord,
	V: Ord,
{
	fn item_cmp<'r, 's>(index_ref: &index::Ref<'r, K, V>, other: &index::Ref<'s, K, V>) -> Ordering
	where
		Self: 'r + 's,
	{
		index_ref.cmp(other)
	}
}
