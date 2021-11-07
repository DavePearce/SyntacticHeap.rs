// =============================================================================
// SyntacticHeap
// =============================================================================

pub struct Node<T> {
    pub kind: T,
    pub children: Vec<usize>
}

pub struct SyntacticHeap<T> {
    nodes: Vec<Node<T>>
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
    pub fn get(&self, index: usize) -> &Node<T> {
	&self.nodes[index]
    }
    /// Push a new node onto the tree
    pub fn push(&'a mut self, kind: T, children: &[usize]) -> usize {
	// Map references into indices
	//let indices : Vec<usize> = children.iter().map(|r| r.index).collect();
	// Save current size of tree
	let idx = self.nodes.len();
	// Push new node in place	
	self.nodes.push(Node{kind,children:children.to_vec()});
	// Return its index
	idx
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

    pub fn children(&self) -> Vec<Ref<'a,T>> {
	// Determine node
	let node = self.parent.get(self.index);
	// Map node's children
	node.children.iter().map(|r| Ref::new(self.parent,*r)).collect()
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

/// Allow conversion from things to references, provided a suitable
/// parent pointer is available.
pub trait ToRef {
    fn to_ref<'a,T>(&self, ast: &'a SyntacticHeap<T>) -> Ref<'a,T>;
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    #[test]
    fn test_heap_01() {
        let mut heap = crate::SyntacticHeap::<i32>::new();
	assert_eq!(heap.push(0,&[]),0);
    }

    #[test]
    fn test_ref_01() {
	let mut heap = crate::SyntacticHeap::<i32>::new();
	let r1 = heap.push(0,&[]);
	let r2 = heap.push(1,&[r1]);
	//assert!(r2.children()[0] == r1);
    }    
}
