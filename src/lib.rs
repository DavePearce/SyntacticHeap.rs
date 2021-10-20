use std::vec;

// =============================================================================
// SyntacticHeap
// =============================================================================

pub struct Node<T> {
    kind: T,
    children: Vec<usize>
}

pub struct SyntacticHeap<T> {
    nodes: Vec<Node<T>>
}

impl<T> SyntacticHeap<T> {
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
    pub fn push(&mut self, kind: T, children: &[usize]) -> usize {
	// Save current size of tree
	let idx = self.nodes.len();
	// Push new node in place	
	self.nodes.push(Node{kind,children:children.to_vec()});
	// Return its index
	idx
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    #[test]
    fn test_01() {
        let mut heap = crate::SyntacticHeap::<i32>::new();
	assert_eq!(heap.push(0,&[]),0);
    }
}
