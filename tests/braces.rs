use std::fmt;
use syntactic_heap::SyntacticHeap;
use syntactic_heap::parser::{Grammar,Node,Term};

/// A minimalist language of matching braces.
#[derive(Clone,Debug,PartialEq)]
enum Expr {
    Empty,
    LeftBrace,
    RightBrace,
    Braced(usize)
}

type AbstractSyntaxTree = SyntacticHeap<Expr>;

/// =======================================================
/// Tests
/// =======================================================

#[test]
fn test_braces_01() {
    let input = "()";
    let chars : Vec<char> = input.chars().collect();
    // Extract grammar
    let g = build_grammar();
    // Attempt to parse something
    assert!(g.parse(&chars));
}

/// =======================================================
/// Helper
/// =======================================================

fn build_grammar() -> Grammar<char,Expr> {
    let mut g : Grammar<char,Expr> = Grammar::new();
    let lbrace = Term::new(&mut g, Node::Terminal(|vs| Expr::LeftBrace,vec!['(']));
    let rbrace = Term::new(&mut g, Node::Terminal(|vs| Expr::RightBrace,vec![')']));
    // FIXME: this is not general enough yet.
    let rule = Term::new(&mut g, Node::NonTerminal(vec![lbrace,rbrace]));
    //
    g
}
