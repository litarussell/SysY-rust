//! Definition of the Abstract Syntax Tree (AST). 
//! 
//!
//! Currently, AST is defined as follows:
//!
//! CompUnit  ::= FuncDef;
//!
//! FuncDef   ::= FuncType Ident "(" ")" Block;
//! FuncType  ::= "int";
//!
//! Block     ::= "{" Stmt "}";
//! Stmt      ::= "return" Number ";";
//! Number    ::= INT_CONST;
//! 
#[derive(Debug)]
pub struct CompUnit {
  pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
  pub func_type: FuncType,
  pub ident: String,
  pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
  Int,
}

#[derive(Debug)]
pub struct Block {
  pub stmt: Stmt
}

#[derive(Debug)]
pub struct Stmt {
  pub num: Number
}


#[derive(Debug)]
pub struct Number {
  pub value: i32
}