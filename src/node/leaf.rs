use smallvec::SmallVec;
use crate::{
	M,
	Index
};

mod buffer;
mod reference;
mod reference_mut;

pub use buffer::Buffer;
pub use reference::Ref;
pub use reference_mut::Mut;

pub struct Metadata {
	parent: Option<usize>,
	items: SmallVec<[Index; M+1]>
}