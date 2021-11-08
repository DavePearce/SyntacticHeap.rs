// =============================================================================
// SyntacticHeap
// =============================================================================

pub struct SyntacticHeap<T> {
    nodes: Vec<T>
}

impl<'a,T> SyntacticHeap<T> {
    
    pub fn new() -> SyntacticHeap<T> {
	SyntacticHeap{nodes: Vec::new()}
    }
    /// Determine how many nodes are in this AST.
    pub fn len(&self) -> usize {
	self.nodes.len()
    }
    /// Access a given node
    pub fn get(&self, index: usize) -> &T {
	&self.nodes[index]
    }
    /// Push a new node onto the tree
    pub fn push(&'a mut self, kind: T) -> Ref {
	// Save current size of tree
	let index = self.nodes.len();
	// Push new node in place	
	self.nodes.push(kind);
	// Return its index
	Ref::new(index)
    }
}

// =============================================================================
// Ref
// =============================================================================

/// A temporary reference into a SyntacticHeap.  This identifies a
/// node within the heap.
#[derive(Copy,Clone)]
pub struct Ref {
    index: usize
}

/// Mechanism for constructing refs
impl Ref {
    pub fn new(index: usize) -> Self {
	Ref{index}
    }

    /// Get the raw index for the node this reference refers to in the
    /// enclosing SyntacticHeap.    
    pub fn raw_index(&self) -> usize {
	self.index
    }
}

// impl<'a,T> PartialEq for Ref<'a,T> {
//     fn eq(&self, other: &Ref<'a,T>) -> bool {
// 	// Coerce into pointers to enable comparison
// 	let p1 = self.parent as *const SyntacticHeap<T>;
// 	let p2 = other.parent as *const SyntacticHeap<T>;
// 	// Do the comparison
// 	(p1 == p2) && (self.index == other.index)
//     }
// }

// Allow conversion from things to references, provided a suitable
// parent pointer is available.
// pub trait ToRef {
//     fn to_ref<'a,T>(&self, ast: &'a SyntacticHeap<T>) -> Ref<'a,T>;
// }

