use crate::SyntacticHeap;

// ================================================
// Grammar
// ================================================

#[derive(Clone,Debug,PartialEq)]
pub enum Node<S,T> {
    /// A fixed string of characters in the underlying stream.
    Terminal(fn(S)->T,Vec<S>),
    /// A named (non-terminal) rule. 
    NonTerminal(Vec<Term>),
    /// A set of rules grouped together.
    Group(Vec<Term>)    
}

// A grammar is itself a syntactic heap.
pub type Grammar<S,T> = SyntacticHeap<Node<S,T>>;

impl<S,T> Grammar<S,T> {
    pub fn parse(&self, input: &[S]) -> bool {
	false
    }
}

// ================================================
// Term
// ================================================

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Term(pub usize);

impl Term {
    pub fn new<S,T>(gr: &mut Grammar<S,T>, t : Node<S,T>) -> Self {
        // Create new node
        let index = gr.push(t).raw_index();
        // Done
        Term(index)
    }
}
