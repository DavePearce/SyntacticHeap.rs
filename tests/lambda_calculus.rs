use syntactic_heap::SyntacticHeap;
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

/// =======================================================
/// Expressions
/// =======================================================

/// An expression is an arbitrary term.
#[derive(Clone,Copy,Debug,PartialEq)]
struct Expr {
    index: usize
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
    let e1 = Expr::from(heap.push(Term::Var(x.clone())));
    let r2 = heap.push(Term::Fun(x.clone(),e1));
    // Sanity check(s)
    assert_eq!(r2.raw_index(),1);    
}
