pub mod internal;
pub mod leaf;

pub enum Node {
	Internal(internal::Metadata),
	Leaf(leaf::Metadata)
}