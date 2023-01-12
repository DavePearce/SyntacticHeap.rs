use std::fmt;
use syntactic_heap::SyntacticHeap;
use syntactic_heap::parser::{Parser};

#[derive(Clone,Debug,PartialEq)]
enum Token {
    Comma,
    Exp(Expr)
}

#[derive(Clone,Debug,PartialEq)]
enum Expr {
    Number(usize),
    List(Vec<Expr>)
}

use Token::*;
use Expr::*;

// "" => [n]
fn rule1(input: &[Token]) -> Option<(usize,Token)> {
    let n = input.len();
    //
    if n == 0 {
        Some((1,Token::Exp(List(vec![]))))
    } else { None }
}

// "n" ... => [n]
fn rule2(input: &[Token]) -> Option<(usize,Token)> {
    let n = input.len();
    //
    if n >= 1 {
        match input[0] {
            Token::Exp(Number(val)) => {
                Some((1,Token::Exp(List(vec![Number(val)]))))
            }
            _ => None
        }        
    } else { None }
}

// "n" => [n]
fn rule3(input: &[Token]) -> Option<(usize,Token)> {
    let n = input.len();
    //
    if n >= 3 {
        match (&input[0],&input[1],&input[2]) {
            (Exp(List(items)),Comma,Exp(Number(v))) => {
                let mut nitems = items.clone();
                nitems.push(Number(*v));
                Some((3,Exp(List(nitems))))
            }
            _ => None
        }        
    } else { None }
}


fn reduce(input: &[Token]) -> Option<(usize,Token)> {
    rule1(input).or_else(|| rule2(input)).or_else(|| rule3(input))
}	

/// =======================================================
/// Tests
/// =======================================================

#[ignore] // because doesn't get called on empty input
#[test]
fn test_lists_01() {
    let input = vec![];
    // Build grammar
    let g = Parser::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Exp(List(Vec::new()))]);
}

#[test]
fn test_lists_02() {
    let input = vec![Exp(Number(0))];
    // Build grammar
    let g = Parser::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Exp(List(vec![Number(0)]))]);    
}

#[test]
fn test_lists_03() {
    let input = vec![Comma];
    // Build grammar
    let g = Parser::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Comma]);    
}

#[test]
fn test_lists_04() {
    let input = vec![Exp(Number(0)),Comma,Exp(Number(1))];
    // Build grammar
    let g = Parser::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Exp(List(vec![Number(0),Number(1)]))]);    
}

#[test]
fn test_lists_05() {
    let input = vec![Exp(Number(0)),Comma,Comma];
    // Build grammar
    let g = Parser::new(reduce);
    // Attempt to parse
    assert_eq!(g.parse(&input),vec![Exp(List(vec![Number(0)])),Comma,Comma]);    
}
