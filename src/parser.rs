use crate::SyntacticHeap;

// ================================================
// Parser
// ================================================
type Rule<T> = fn(&[T])->Option<(usize,T)>;

/// A generic bottom up parser (i.e. based around shift-reduce
/// semantics).  In essence, it applies a given reduction rule as much
/// as possible in a left-to-right order.  This does not currently
/// support lookahead.
pub struct Parser<T> {
    reduce: Rule<T>
}

impl<T:Clone> Parser<T> {
    pub fn new(r: Rule<T>) -> Self {
	Self{reduce: r}
    }
    // A very simple reduction
    pub fn parse(&self, input: &[T]) -> Vec<T> {
	// Create empty stack
	let mut stack : Vec<T> = Vec::new();
	// Iterate over input stream
	for i in 0..input.len() {
	    // Shift
	    stack.push(input[i].clone());	    
	    // Reduce for as along as possible.
	    loop {
		// Reduce
		let r = (self.reduce)(&stack);
		// Decide what to do
		match r {
		    Some((n,t)) => {
			stack.truncate(stack.len()-n);
			stack.push(t);
		    }
		    None => {
			break
		    }
		}
	    }
	}
	stack
    }
}
