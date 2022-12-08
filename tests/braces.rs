use std::fmt;
use syntactic_heap::SyntacticHeap;
use syntactic_heap::parser::{Grammar};

#[derive(Copy,Clone,Debug,PartialEq)]
enum Expr {
    Left,  // "("
    Right, // ")"
    Match  // "(" Expr ")" 
}

use Expr::*;

fn reduce(input: &[Expr]) -> Option<(usize,Expr)> {
    let n = input.len();
    //
    if n >= 2 {
	let w1 = input[n-1];	
	let w2 = input[n-2];
	match (w2,w1) {
	    (Left,Right) => { return Some((2,Match)); }
	    (Match,Match) => { return Some((2,Match)); }
	    _ => {}
	}
	//
	if n >= 3 {
	    let w3 = input[n-3];
	    match (w3,w2,w1) {
		(Left,Match,Right) => { return Some((3,Match)); }
		_ => {}	    
	    }
	}	
    }
    //
    None
}

/// =======================================================
/// Tests
/// =======================================================

#[test]
fn test_braces_01() {
    let input = vec![Expr::Left];
    // Build grammar
    let g = Grammar::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),input);
}

#[test]
fn test_braces_02() {
    let input = vec![Expr::Right];
    // Build grammar
    let g = Grammar::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),input);
}

#[test]
fn test_braces_03() {
    let input = vec![Expr::Left,Expr::Right];
    // Build grammar
    let g = Grammar::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Expr::Match]);
}

#[test]
fn test_braces_04() {
    let input = vec![Expr::Left,Expr::Right,Expr::Left];
    // Build grammar
    let g = Grammar::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Expr::Match,Expr::Left]);
}

#[test]
fn test_braces_05() {
    let input = vec![Expr::Left,Expr::Right,Expr::Right];
    // Build grammar
    let g = Grammar::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Expr::Match,Expr::Right]);
}

#[test]
fn test_braces_06() {
    let input = vec![Expr::Left,Expr::Right,Expr::Left, Expr::Right];
    // Build grammar
    let g = Grammar::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Expr::Match]);
}

#[test]
fn test_braces_07() {
    let input = vec![Expr::Left,Expr::Left,Expr::Right, Expr::Right];
    // Build grammar
    let g = Grammar::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Expr::Match]);
}
