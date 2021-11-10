use std::fmt;
use std::ops::Deref;

// =============================================================================
// Node
// =============================================================================

/// Generic methods required
pub trait Substitute {
    /// Substitute children of this node
    fn substitute(&self, children: &[usize]) -> Self;
}

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
    pub fn push(&'a mut self, kind: T) -> Ref<'a,T> {
	// Save current size of tree
	let index = self.nodes.len();
	// Push new node in place	
	self.nodes.push(kind);
	// Return its index
	Ref::new(self,index)
    }
}

impl<'a,T> SyntacticHeap<T>
where
    T : Substitute,
    for <'b> &'b T : IntoIterator<Item=usize>
{
    /// Clone node into this heap
    pub fn deep_clone(&'a mut self, index: usize) -> Ref<'a,T> {
        let mut n = &self.nodes[index];
	// Create child array
        let mut children : Vec<usize> = n.into_iter().collect();
        // Do the deep clone
	for i in 0..children.len() {
	    let r = self.deep_clone(children[i]);
	    children[i] = r.raw_index();
	}
	// Reborrow
	n = &self.nodes[index];
	// Final substitution
	self.push(n.substitute(&children[..]))
    }
}

impl<T> fmt::Display for SyntacticHeap<T>
where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let mut first = true;
	write!(f,"[");
	for item in &self.nodes {
	    if !first { write!(f,","); }
	    first = false;
	    write!(f,"{}",item);
	}
	write!(f,"]")
    }
}

impl<T> fmt::Debug for SyntacticHeap<T>
where T : fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let mut first = true;
	write!(f,"[");
	for item in &self.nodes {
	    if !first { write!(f,","); }
	    first = false;
	    write!(f,"{:?}",item);
	}
	write!(f,"]")
    }
}


// =============================================================================
// Ref
// =============================================================================

/// A temporary reference into a SyntacticHeap.  This identifies a
/// node within the heap.
#[derive(Copy,Clone)]
pub struct Ref<'a,T> {
    parent: &'a SyntacticHeap<T>,
    index: usize
}

/// Mechanism for constructing refs
impl<'a,T> Ref<'a,T> {
    pub fn new(parent: &'a SyntacticHeap<T>, index: usize) -> Self {
	Ref{parent,index}
    }

    pub fn get(&self) -> &T {
	&self.parent.nodes[self.index]
    }
    
    /// Get the raw index for the node this reference refers to in the
    /// enclosing SyntacticHeap.    
    pub fn raw_index(&self) -> usize {
	self.index
    }
}

impl<'a,T> PartialEq for Ref<'a,T> {
    fn eq(&self, other: &Ref<'a,T>) -> bool {
	// Coerce into pointers to enable comparison
	let p1 = self.parent as *const SyntacticHeap<T>;
	let p2 = other.parent as *const SyntacticHeap<T>;
	// Do the comparison
	(p1 == p2) && (self.index == other.index)
    }
}

impl<'a,T> Deref for Ref<'a,T> {
    type Target = T;
    
    fn deref(&self) -> &T {
	self.get()
    }
}

// Allow conversion from things to references, provided a suitable
// parent pointer is available.
// pub trait ToRef {
//     fn to_ref<'a,T>(&self, ast: &'a SyntacticHeap<T>) -> Ref<'a,T>;
// }

