use std::fmt;
use syntactic_heap::SyntacticHeap;
use syntactic_heap::Substitute;
use syntactic_heap::Ref;
use Term::*;

/// A minimalist term language based on the lambda calculus.  This is
/// intended to demonstrate some aspects of the SyntacticHeap API
/// (though not all).
#[derive(Clone,Debug,PartialEq)]
enum Term {
    /// Function Delection `&x -> e`
    Fun(String,Expr),
    /// Function Application `e1 e2`
    App(Expr,Expr),
    /// Variable name
    Var(String)
}

/// Enable substitution for terms.  This is necessary so they can be
/// cloned, for example.
impl Substitute for Term {
    fn substitute(&self, children: &[usize]) -> Term {
	match self {
	    Fun(s,_) => {
		Fun(s.clone(),Expr{index:children[0]})
	    }
	    App(_,_) => {
		App(Expr{index:children[0]},Expr{index:children[1]})
	    }
	    Var(s) => {
		Var(s.clone())
	    }
	}
    }
}

/// Enable iteration of children for a given term.  This allows a
/// syntactic heap to traverse the dependency graph, for example to do
/// garbage collection or cloning, etc.  At the moment, this actually
/// takes a pretty inefficient approach using Vec's IntoIterator
/// support.  That's nice and compact, but means we end up cloning
/// more than necessary.
impl IntoIterator for &Term {
    type Item = usize;
    type IntoIter = <Vec<usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
	match self {
	    Fun(_,e) => {
		vec![e.index].into_iter()
	    }
	    App(e1,e2) => {
		vec![e1.index,e2.index].into_iter()
	    }
	    _ => {
		vec![].into_iter()
	    }
	}
    }
}

/// Simple debug output for terms.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"{:?}",self)
    }    
}

/// =======================================================
/// Expressions
/// =======================================================

/// An expression refers to an arbitrary term in the heap.  Creating a
/// new reference results in it being pushed onto the heap.
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
fn test_heap_01() {
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
fn test_heap_02() {
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
fn test_heap_03() {
    let x = "x".to_string();    
    // Initiailise heap
    let mut heap = SyntacticHeap::<Term>::new();
    // Create node(s)
    let e1 = Expr::new(&mut heap,Term::Var(x.clone()));
    let r2 = heap.push(Term::Fun(x.clone(),e1));
    // Sanity check(s)
    assert_eq!(r2.raw_index(),1);
}

//

#[test]
fn test_iter_01() {
    let x = "x".to_string();        
    // Initiailise heap
    let t = Term::Var(x);
    // Sanity check(s)
    for _i in &t {
	assert!(false);
    }
}

#[test]
fn test_iter_02() {
    let x = "x".to_string();        
    // Initiailise heap
    let t = Term::Fun(x, Expr{index:123});
    // Sanity check(s)
    for i in &t {
	assert_eq!(i,123);
    }
}

#[test]
fn test_iter_03() {
    // Initiailise Term
    let t = Term::App(Expr{index:123},Expr{index:234});
    // Sanity check(s)
    for i in &t {
	assert!((i == 123) || (i == 234));
    }
}

//

#[test]
fn test_clone_01() {
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
