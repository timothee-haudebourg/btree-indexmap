use generic_btree::node::Offset;
use crate::{
	M,
	Inner,
	index
};
use super::Metadata;

/// Internal node reference.
pub struct Ref<'a, K, V> {
	/// Reference to the node metadata.
	meta: &'a Metadata,

	/// Reference to the tree data.
	data: &'a Inner<K, V>
}

impl<'a, K, V> Ref<'a, K, V> {
	pub fn new(meta: &'a Metadata, data: &'a Inner<K, V>) -> Self {
		Self {
			meta, data
		}
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::ItemAccess<crate::Ref<'a, K, V>> for Ref<'r, K, V> {
	fn item_count(&self) -> usize {
		self.meta.branches.len()
	}

	fn borrow_item(&self, offset: Offset) -> Option<index::Ref<'r, K, V>> {
		panic!("TODO")
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::ItemAccess<crate::Mut<'a, K, V>> for Ref<'r, K, V> {
	fn item_count(&self) -> usize {
		self.meta.branches.len()
	}

	fn borrow_item(&self, offset: Offset) -> Option<index::Ref<'r, K, V>> {
		panic!("TODO")
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::InternalRef<crate::Ref<'a, K, V>> for Ref<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.meta.parent
	}

	fn child_id(&self, index: usize) -> Option<usize> {
		panic!("TODO")
	}

	fn max_capacity(&self) -> usize {
		M
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::InternalRef<crate::Mut<'a, K, V>> for Ref<'r, K, V> {
	fn parent(&self) -> Option<usize> {
		self.meta.parent
	}

	fn child_id(&self, index: usize) -> Option<usize> {
		panic!("TODO")
	}

	fn max_capacity(&self) -> usize {
		M
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::InternalConst<'r, crate::Ref<'a, K, V>> for Ref<'r, K, V> {
	fn item(&self, offset: Offset) -> Option<index::Ref<'r, K, V>> {
		panic!("TODO")
	}
}

impl<'r, 'a: 'r, K, V> generic_btree::node::InternalConst<'r, crate::Mut<'a, K, V>> for Ref<'r, K, V> {
	fn item(&self, offset: Offset) -> Option<index::Ref<'r, K, V>> {
		panic!("TODO")
	}
}