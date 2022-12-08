use crate::SyntacticHeap;

// ================================================
// Grammar
// ================================================

// A grammar is itself a syntactic heap.
pub struct Grammar<T> {
    reduce: fn(&[T])->Option<(usize,T)>
}

impl<T:Copy> Grammar<T> {
    pub fn new(r: fn(&[T])->Option<(usize,T)>) -> Self {
	Grammar{reduce: r}
    }
    // A very simple reduction
    pub fn parse(&self, input: &[T]) -> Vec<T> {
	// Create empty stack
	let mut stack : Vec<T> = Vec::new();
	// Iterate over input stream
	for i in 0..input.len() {
	    // Shift
	    stack.push(input[i]);	    
	    // Reduce
	    let r = (self.reduce)(&stack);
	    // Decide what to do.
	    match r {
		Some((n,t)) => {
		    stack.truncate(stack.len()-n);
		    stack.push(t);
		}
		None => {}
	    }	    
	}
	stack
    }
}
