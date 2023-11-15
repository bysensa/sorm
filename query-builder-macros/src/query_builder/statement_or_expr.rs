use std::fmt::Display;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use super::{
    for_loop::ForLoop,
    generate_variable_name,
    statement_parser::{
        BeginTransactionStatement, CancelTransactionStatement, CommitTransactionStatement,
        LetStatement, ReturnStatement,
    },
};

pub(crate) enum Query {
    LetStatement(LetStatement),
    ForLoop {
        generated_ident: Ident,
        for_loop: ForLoop,
    },
    BeginTransaction,
    CommitTransaction,
    CancelTransaction,
    ReturnStatement(ReturnStatement),
    BreakStatement,
    ContinueStatement,
    Expr {
        generated_ident: Ident,
        expr: Expr,
    },
}

#[derive(Debug, PartialEq)]
enum TransactionType {
    Begin,
    Commit,
    Cancel,
    Invalid,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TransactionType::Begin => "begin transaction",
                TransactionType::Commit => "commit transaction",
                TransactionType::Cancel => "cancel transaction",
                TransactionType::Invalid => unreachable!(),
            }
        )
    }
}

impl TransactionType {
    pub fn is_transaction(input: ParseBuffer<'_>) -> TransactionType {
        let input_str = input
            .to_string()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();

        let likely_transaction = input.peek(Ident) && input.peek2(Ident) && input.peek3(Token![;]);

        if likely_transaction {
            if input_str.starts_with("begin transaction") {
                TransactionType::Begin
            } else if input_str.starts_with("commit transaction") {
                TransactionType::Commit
            } else if input_str.starts_with("cancel transaction") {
                TransactionType::Cancel
            } else {
                TransactionType::Invalid
            }
        } else {
            TransactionType::Invalid
        }
    }

    pub fn is_begin_transaction(input: ParseBuffer<'_>) -> bool {
        TransactionType::is_transaction(input) == TransactionType::Begin
    }

    pub fn is_commit_transaction(input: ParseBuffer<'_>) -> bool {
        TransactionType::is_transaction(input) == TransactionType::Commit
    }

    pub fn is_cancel_transaction(input: ParseBuffer<'_>) -> bool {
        TransactionType::is_transaction(input) == TransactionType::Cancel
    }
}

enum StatementType {
    Let,
    Expr,
    Return,
    Break,
    Continue,
    BeginTransaction,
    CommitTransaction,
    CancelTransaction,
}

impl<'a> From<&ParseBuffer<'a>> for StatementType {
    fn from(value: &ParseBuffer<'a>) -> Self {
        let input_str = value
            .to_string()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();

        if value.peek(Token![let]) {
            StatementType::Let
        } else if value.peek(Token![return]) {
            StatementType::Return
        } else if value.peek(Token![break]) {
            StatementType::Break
        } else if value.peek(Token![continue]) {
            StatementType::Continue
        }
        // else if TransactionType::is_begin_transaction(value) {
        //     StatementType::BeginTransaction
        // } else if TransactionType::is_commit_transaction(value) {
        //     StatementType::CommitTransaction
        // } else if TransactionType::is_cancel_transaction(value) {
        //     StatementType::CancelTransaction
        // }
        else if value.peek(Ident) && value.peek2(Ident) && value.peek3(Token![;]) {
            if input_str.starts_with("begin transaction") {
                StatementType::BeginTransaction
            } else if input_str.starts_with("commit transaction") {
                StatementType::CommitTransaction
            } else if input_str.starts_with("cancel transaction") {
                StatementType::CancelTransaction
            } else {
                StatementType::Expr
            }
        } else {
            StatementType::Expr
        }
    }
}

impl Parse for Query {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let statement_type = StatementType::from(input);

        match statement_type {
            StatementType::Let => {
                let let_statement = input.parse::<LetStatement>()?;
                Ok(Query::LetStatement(let_statement))
            }
            StatementType::Expr => {
                let expr = input.parse::<Expr>()?;
                let _end: Token![;] = input.parse()?;
                Ok(Query::Expr {
                    generated_ident: generate_variable_name(),
                    expr,
                })
            }
            StatementType::Return => {
                let _return: Token![return] = input.parse()?;
                let expr = input.parse::<Expr>()?;
                let _end: Token![;] = input.parse()?;
                Ok(Query::Expr {
                    generated_ident: generate_variable_name(),
                    expr,
                })
            }
            StatementType::Break => {
                let _break: Token![break] = input.parse()?;
                let _end: Token![;] = input.parse()?;
                Ok(Query::BreakStatement)
            }
            StatementType::Continue => todo!(),
            StatementType::BeginTransaction => {
                let begin_trans = input.parse::<BeginTransactionStatement>()?;
                Ok(Query::BeginTransaction)
            }
            StatementType::CommitTransaction => {
                let commit_transaction = input.parse::<CommitTransactionStatement>()?;
                Ok(Query::CommitTransaction)
            }
            StatementType::CancelTransaction => {
                let cancel_transaction = input.parse::<CancelTransactionStatement>()?;
                Ok(Query::CancelTransaction)
            }
        }
    }
}
