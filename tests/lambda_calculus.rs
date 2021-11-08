use std::fmt;

use syntactic_heap::SyntacticHeap;
use syntactic_heap::Node;
use syntactic_heap::Ref;

/// A minimalist term language based on the lambda calculus.  This is
/// intended to demonstrate some aspects only of the SyntacticHeap
/// API.
#[derive(Clone,Debug,PartialEq)]
enum Term {
    /// Function Delection `&x -> e`
    Fun(String,Expr),
    /// Function Application `e1 e2`
    App(Expr,Expr),
    /// Variable name
    Var(String)
}

impl Node for Term {
    fn len(&self) -> usize {
	match self {
	    Term::Fun(_,_) => 1,
	    Term::App(_,_) => 2,
	    _ => 0
	}
    }

    fn get(&self, ith: usize) -> Option<usize> {
	match self {
	    Term::Fun(_,e) => {
		match ith {
		    0 => Some(e.index),
		    _ => None
			
		}
	    }
	    Term::App(e1,e2) => {
		match ith {
		    0 => Some(e1.index),
		    1 => Some(e2.index),
		    _ => None
		}
	    }
	    _ => {
		None
	    }
	}
    }

    fn substitute(&self, children: &[usize]) -> Term {
	assert!(children.len() == self.len());
	//
	match self {
	    Term::Fun(s,e) => {
		Term::Fun(s.clone(),Expr{index:children[0]})
	    }
	    Term::App(e1,e2) => {
		Term::App(Expr{index:children[0]},Expr{index:children[1]})
	    }
	    Term::Var(s) => {
		Term::Var(s.clone())
	    }
	}
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"{:?}",self)
    }    
}

/// =======================================================
/// Expressions
/// =======================================================

/// An expression is an arbitrary term.
#[derive(Clone,Copy,Debug,PartialEq)]
struct Expr {
    index: usize
}

impl<'a> Expr {
    pub fn new(heap: &mut SyntacticHeap<Term>, term: Term) -> Self {
	let index = heap.push(term).raw_index();
	Expr{index}
    }
}

impl<'a> From<Ref<'a,Term>> for Expr {
    fn from(r:Ref<'a,Term>) -> Self {
	Expr{index:r.raw_index()}
    }
}

/// =======================================================
/// Tests
/// =======================================================

#[test]
fn test_01() {
    let x = "x".to_string();        
    // Initiailise heap
    let mut heap = SyntacticHeap::<Term>::new();
    // Create node(s)
    let r1 = heap.push(Term::Var(x.clone()));
    // Sanity check(s)
    assert_eq!(r1.raw_index(),0);
    assert_eq!(*r1.get(), Term::Var(x));
}

#[test]
fn test_02() {
    let x = "x".to_string();    
    // Initiailise heap
    let mut heap = SyntacticHeap::<Term>::new();
    // Create node(s)
    let r1 = heap.push(Term::Var(x.clone()));
    // Sanity check(s)
    assert_eq!(r1.raw_index(),0);
    assert_eq!(*r1, Term::Var(x));
}

#[test]
fn test_03() {
    let x = "x".to_string();    
    // Initiailise heap
    let mut heap = SyntacticHeap::<Term>::new();
    // Create node(s)
    let e1 = Expr::new(&mut heap,Term::Var(x.clone()));
    let r2 = heap.push(Term::Fun(x.clone(),e1));
    // Sanity check(s)
    assert_eq!(r2.raw_index(),1);
}

#[test]
fn test_04() {
    let x = "x".to_string();    
    // Initiailise heap
    let mut heap = SyntacticHeap::<Term>::new();
    // Create node(s)
    let e1 = Expr::new(&mut heap,Term::Var(x.clone()));
    let e2 = Expr::new(&mut heap,Term::Fun(x.clone(),e1));
    // Deep clone
    heap.deep_clone(e2.index);
    //
    println!("HEAP={}",heap);
    // Sanity Check(s)
    assert_eq!(heap.get(2), &Term::Var(x.clone()));
    assert_eq!(heap.get(3), &Term::Fun(x,Expr{index:2}));    
}

