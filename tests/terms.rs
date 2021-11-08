use syntactic_heap::SyntacticHeap;
use syntactic_heap::Ref;

/// A very minimalist term language used to test out the SyntacticHeap
/// API.
enum Term {
    TypeInt,
    TypeArray(Type)
}

struct Type {
    index: usize
}

impl From<Ref> for Type {
    fn from(r:Ref) -> Self {
	/// FIXME: should check this is really a type!
	Type{index:r.raw_index()}
    }
}

#[test]
fn test_01() {
    let mut heap = SyntacticHeap::<Term>::new();
    let t1 = heap.push(Term::TypeInt);
    assert_eq!(t1.raw_index(),0);
    assert!(matches!(heap.get(0), Term::TypeInt));
}

#[test]
fn test_02() {
    let mut heap = SyntacticHeap::<Term>::new();
    let t1 = Type::from(heap.push(Term::TypeInt));
    let t2 = heap.push(Term::TypeArray(t1));    
    assert_eq!(t2.raw_index(),1);
    assert!(matches!(heap.get(1), Term::TypeArray(_)));   
}
