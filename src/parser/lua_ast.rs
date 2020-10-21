use std::fmt::Write;

use super::common::*;
use crate::config::*;
use crate::{cfg_write, cfg_write_helper};
use crate::formatting::loc_hint::*;
use crate::formatting::list;
use crate::formatting::decoration::*;

#[derive(Debug)]
pub enum Node {
    BinaryOp(Loc, [Loc; 2], Str<'static>, Box<Node>, Box<Node>),
    UnaryOp(Loc, [Loc; 1], Str<'static>, Box<Node>),
    UnaryNot(Loc, [Loc; 1], Box<Node>),

    Var(Loc, [Loc; 1], Box<Node>, Box<Node>),
    RoundBrackets(Loc, [Loc; 2], Box<Node>),
    ArgsRoundBrackets(Loc, [Loc; 2], Box<Node>),
    ArgsRoundBracketsEmpty(Loc, [Loc; 1]),

    Nil(Loc),
    False(Loc),
    True(Loc),
    VarArg(Loc),
    Break(Loc),
    Numeral(Loc, String),
    NormalStringLiteral(Loc, String),
    CharStringLiteral(Loc, String),
    MultiLineStringLiteral(Loc, usize, String),

    TableConstructor(Loc, [Loc; 2], Box<Node>),
    TableConstructorEmpty(Loc, [Loc; 1]),
    Fields(Loc, Vec<(Loc, Node, Loc, String)>),
    FieldNamedBracket(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FieldNamed(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FieldSequential(Loc, Box<Node>),

    TableIndex(Loc, [Loc; 2], Box<Node>),
    TableMember(Loc, [Loc; 1], Box<Node>),
    ExpList(Loc, Vec<(Loc, Node, Loc, String)>),
    NameList(Loc, Vec<(Loc, Node, Loc, String)>),
    ParList(Loc, Vec<(Loc, Node, Loc, String)>),
    VarList(Loc, Vec<(Loc, Node, Loc, String)>),
    VarRoundSuffix(Loc, [Loc; 3], Box<Node>, Box<Node>),
    VarSuffixList(Loc, Vec<(Loc, Node)>),
    FnMethodCall(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FunctionDef(Loc, [Loc; 1], Box<Node>),
    FuncBody(Loc, [Loc; 2]),
    FuncBodyB(Loc, [Loc; 3], Box<Node>),
    FuncPBody(Loc, [Loc; 3], Box<Node>),
    FuncPBodyB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FuncName(Loc, Vec<(Loc, Node, Loc, String)>),
    FuncNameSelf(Loc, [Loc; 2], Vec<(Loc, Node, Loc, String)>, Box<Node>),
    FuncDecl(Loc, [Loc; 2], Box<Node>, Box<Node>),
    LocalFuncDecl(Loc, [Loc; 3], Box<Node>, Box<Node>),

    StatementList(Loc, Vec<(Loc, Node)>),
    DoEnd(Loc, [Loc; 1]),
    DoBEnd(Loc, [Loc; 2], Box<Node>),
    VarsExprs(Loc, [Loc; 2], Box<Node>, Box<Node>),
    Name(Loc, String),
    Label(Loc, [Loc; 2], Box<Node>),
    GoTo(Loc, [Loc; 1], Box<Node>),
    WhileDo(Loc, [Loc; 3], Box<Node>),
    WhileDoB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    RepeatUntil(Loc, [Loc; 2], Box<Node>),
    RepeatBUntil(Loc, [Loc; 3], Box<Node>, Box<Node>),
    ForInt(Loc, [Loc; 7], Box<Node>, Box<Node>, Box<Node>),
    ForIntB(Loc, [Loc; 8], Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ForIntStep(Loc, [Loc; 9], Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ForIntStepB(Loc, [Loc; 10], Box<Node>, Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ForRange(Loc, [Loc; 5], Box<Node>, Box<Node>),
    ForRangeB(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),

    LocalNames(Loc, [Loc; 1], Box<Node>),
    LocalNamesExprs(Loc, [Loc; 3], Box<Node>, Box<Node>),

    IfThen(Loc, [Loc; 3], Box<Node>),
    IfThenB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    IfThenElse(Loc, [Loc; 4], Box<Node>),
    IfThenBElse(Loc, [Loc; 5], Box<Node>, Box<Node>),
    IfThenElseB(Loc, [Loc; 5], Box<Node>, Box<Node>),
    IfThenBElseB(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),
    IfThenElseIf(Loc, [Loc; 4], Box<Node>, Box<Node>),
    IfThenBElseIf(Loc, [Loc; 5], Box<Node>, Box<Node>, Box<Node>),
    IfThenElseIfElse(Loc, [Loc; 5], Box<Node>, Box<Node>),
    IfThenBElseIfElse(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),
    IfThenElseIfElseB(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),
    IfThenBElseIfElseB(Loc, [Loc; 7], Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ElseIfThenVec(Loc, Vec<(Loc, Node)>),
    ElseIfThen(Loc, [Loc; 2], Box<Node>),
    ElseIfThenB(Loc, [Loc; 3], Box<Node>, Box<Node>),

    RetStatNone(Loc),
    RetStatExpr(Loc, [Loc; 1], Box<Node>),
    RetStatNoneComma(Loc, [Loc; 1]),
    RetStatExprComma(Loc, [Loc; 2], Box<Node>),
    StatsRetStat(Loc, [Loc; 1], Box<Node>, Box<Node>),
    Chunk(Loc, Box<Node>, Loc),
    SheBangChunk(Loc, Box<Node>, Loc, Box<Node>, Loc),
    Semicolon(Loc),
    SheBang(Loc, String),
}

impl<'a> list::NoSepListItem<'a> for Node {
    fn list_item_prefix_hint(&self, _: &'a Config) -> &'a str {
        use Node::*;
        match self {
            Semicolon(..) | ArgsRoundBrackets(..) | ArgsRoundBracketsEmpty(..) | TableIndex(..)
                | TableMember(..) | FnMethodCall(..) | TableConstructor(..) | TableConstructorEmpty(..) => "",
            _ => " ",
        }
    }

    fn list_item_suffix_hint(&self, _: &'a Config) -> &'a str {
        // use Node::*;
        match self {
            _ => "",
        }
    }

    fn need_indent(&self, cfg: &'a Config) -> bool {
        use Node::*;
        match self {
            VarsExprs(..) | RepeatUntil(..) | RepeatBUntil(..) | LocalNames(..) | LocalNamesExprs(..) | Break(..)
            | GoTo(..) | DoEnd(..) | DoBEnd(..) | WhileDo(..) | WhileDoB(..) | IfThen(..) | IfThenB(..)
            | IfThenElse(..) | IfThenBElse(..) | IfThenElseB(..) | IfThenBElseB(..) | IfThenElseIf(..)
            | IfThenBElseIf(..) | IfThenElseIfElse(..) | IfThenBElseIfElse(..) | IfThenElseIfElseB(..)
            | IfThenBElseIfElseB(..) | ForInt(..) | ForIntB(..) | ForIntStep(..) | ForIntStepB(..)
            | ForRange(..) | ForRangeB(..) | FuncDecl(..) | LocalFuncDecl(..) | Var(..) | VarRoundSuffix(..)
            | RoundBrackets(..) =>
                cfg.indentation_string.is_some() && cfg.indent_every_statement == Some(true),
            ElseIfThen(..) | ElseIfThenB(..)
                => cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1),
            FieldNamedBracket(..) | FieldNamed(..) | FieldSequential(..) =>
                cfg.indentation_string.is_some() && cfg.table_indent_format == Some(1),
            _ => false,
        }
    }
}

impl list::SepListOfItems<Node> for Node {
    fn items(&self) -> Option<&Vec::<(Loc, Node, Loc, String)>> {
        use Node::*;
        match self {
            Fields(_, items) | ExpList(_, items) | NameList(_, items) | VarList(_, items) | ParList(_, items)
                | FuncName(_, items) | FuncNameSelf(_, _, items, _) => Some(items),
            _ => None,
        }
    }

    fn element_prefix_hint(&self) -> &str {
        use Node::*;
        match self {
            Fields(..) | ExpList(..) | NameList(..) | VarList(..) | ParList(..) => " ",
            FuncName(..) | FuncNameSelf(..) => "",
            _ => "",
        }
    }

    fn separator(&self, cfg: &Config) -> Option<String> {
        use Node::*;
        match self {
            Fields(..) => cfg.field_separator.clone(),
            ExpList(..) | NameList(..) | VarList(..) | ParList(..) => Some(",".to_string()),
            FuncName(..) | FuncNameSelf(..) => Some(".".to_string()),
            _ => None,
        }
    }

    fn trailing_separator(&self, cfg: &Config) -> Option<bool> {
        use Node::*;
        match self {
            Fields(..) => cfg.write_trailing_field_separator.clone(),
            ExpList(..) | NameList(..) | VarList(..) | ParList(..) | FuncName(..) | FuncNameSelf(..) => Some(false),
            _ => None,
        }
    }

    fn need_indent_items(&self, cfg: &Config) -> bool {
        use Node::*;
        match self {
            Fields(..) => cfg.table_indent_format == Some(1),
            _ => false,
        }
    }
}


impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> std::fmt::Result {
        use Node::*;

        #[allow(non_snake_case)]
        let Hint = CommentLocHint;
        let cfg_write_list_items = list::cfg_write_list_items::<Node, CommentLocHint>;
        let cfg_write_sep_list = list::cfg_write_sep_list::<Node, CommentLocHint>;

        match self {
            BinaryOp(_, locs, tok, l, r) => {
                let nl1 = cfg.indentation_string.is_some() && cfg.binary_op_indent_format == Some(1);
                let nl2 = cfg.indentation_string.is_some() && cfg.binary_op_indent_format == Some(2);
                cfg_write!(f, cfg, buf, state, IndentDecor(1), l, NewLineDecor(Hint(&locs[0], " "), nl1), tok,
                           NewLineDecor(Hint(&locs[1], " "), nl2), r, IndentDecor(-1))
            }
            UnaryOp(_, locs, tok, r) => cfg_write!(f, cfg, buf, state, tok, Hint(&locs[0], ""), r),
            UnaryNot(_, locs, r) => cfg_write!(f, cfg, buf, state, "not", Hint(&locs[0], " "), r),

            Var(_, locs, n1, n2) => cfg_write!(f, cfg, buf, state, n1, Hint(&locs[0], ""), n2),
            RoundBrackets(_, locs, r) => {
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), r, Hint(&locs[1], ""), ")")
            }
            ArgsRoundBrackets(_, locs, r) => {
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), r, Hint(&locs[1], ""), ")")
            }
            ArgsRoundBracketsEmpty(_, locs) => cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")"),

            Nil(_) => write!(f, "nil"),
            False(_) => write!(f, "false"),
            True(_) => write!(f, "true"),
            VarArg(_) => write!(f, "..."),
            Break(_) => write!(f, "break"),

            // literals
            Numeral(_, s) => write!(f, "{}", s),
            NormalStringLiteral(_, s) => write!(f, "\"{}\"", s),
            CharStringLiteral(_, s) => write!(f, "'{}'", s),
            MultiLineStringLiteral(_, level, s) => {
                let level_str = (0..*level).map(|_| "=").collect::<String>();
                write!(f, "[{}[{}]{}]", level_str, s, level_str)
            }

            TableConstructor(_, locs, r) => {
                let default_hint = String::new();
                let hint = cfg.hint_table_constructor.as_ref().unwrap_or(&default_hint);
                let nl = cfg.indentation_string.is_some() && cfg.table_indent_format == Some(1);

                cfg_write!(f, cfg, buf, state, "{{", IndentDecor(1), NewLineDecor(Hint(&locs[0], &hint), nl), r,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[1], &hint), nl), "}}")
            }
            TableConstructorEmpty(_, locs) => {
                let default_hint = String::new();
                let hint = cfg.hint_table_constructor.as_ref().unwrap_or(&default_hint);

                cfg_write!(f, cfg, buf, state, "{{", Hint(&locs[0], &hint), "}}")
            }
            Fields(..) => {
                cfg_write_sep_list(f, cfg, buf, state, self)
            }
            FieldNamedBracket(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, state, "[", Hint(&locs[0], ""), e1, Hint(&locs[1], ""), "]", Hint(&locs[2], " "),
                           "=", Hint(&locs[3], " "), e2)
            }
            FieldNamed(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, state, e1, Hint(&locs[0], " "), "=", Hint(&locs[1], " "), e2)
            }
            FieldSequential(_, e) => cfg_write!(f, cfg, buf, state, e),

            TableIndex(_, locs, e) => cfg_write!(f, cfg, buf, state, "[", Hint(&locs[0], ""), e, Hint(&locs[1], ""), "]"),
            TableMember(_, locs, n) => cfg_write!(f, cfg, buf, state, ".", Hint(&locs[0], ""), n),
            ExpList(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            NameList(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            VarList(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            StatementList(_, stts) => cfg_write_list_items(f, cfg, buf, state, stts),
            DoEnd(_, locs) => cfg_write!(f, cfg, buf, state, "do", Hint(&locs[0], " "), "end"),
            DoBEnd(_, locs, b) => {
                let nl = cfg.indentation_string.is_some() && cfg.do_end_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "do", IndentDecor(1), NewLineDecor(Hint(&locs[0], " "), nl), b,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[1], " "), nl), "end")
            }
            VarsExprs(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, n1, Hint(&locs[0], " "), "=", Hint(&locs[1], " "), n2)
            }

            VarRoundSuffix(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), ")", Hint(&locs[2], ""),
                           n2)
            }
            VarSuffixList(_, suffs) => cfg_write_list_items(f, cfg, buf, state, suffs),
            FnMethodCall(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, ":", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), n2)
            }
            ParList(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            FunctionDef(_, locs, n) => cfg_write!(f, cfg, buf, state, "function", Hint(&locs[0], ""), n),
            FuncBody(_, locs) => {
                let nl = cfg.indentation_string.is_some() && cfg.function_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")", NewLineDecor(Hint(&locs[1], " "), nl),
                           "end")
            }
            FuncBodyB(_, locs, n2) => {
                let nl = cfg.indentation_string.is_some() && cfg.function_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")",
                           IndentDecor(1), NewLineDecor(Hint(&locs[1], " "), nl), n2,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[2], " "), nl), "end")
            }
            FuncPBody(_, locs, n1) => {
                let nl = cfg.indentation_string.is_some() && cfg.function_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), ")",
                           NewLineDecor(Hint(&locs[2], " "), nl), "end")
            }
            FuncPBodyB(_, locs, n1, n2) => {
                let nl = cfg.indentation_string.is_some() && cfg.function_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), ")",
                           IndentDecor(1), NewLineDecor(Hint(&locs[2], " "), nl), n2,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[3], " "), nl), "end")
            }
            FuncName(..) => cfg_write_sep_list(f, cfg, buf, state, self),
            FuncNameSelf(_, locs, _, n) => {
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                cfg_write!(f, cfg, buf, state, Hint(&locs[0], ""), ":", Hint(&locs[1], ""), n)
            }
            FuncDecl(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, "function", Hint(&locs[0], " "), n1, Hint(&locs[1], ""), n2)
            }
            LocalFuncDecl(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), "function", Hint(&locs[1], " "), n1,
                           Hint(&locs[2], ""), n2)
            }

            LocalNames(_, locs, n) => cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), n),
            LocalNamesExprs(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), n1, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), n2)
            }

            // if
            IfThen(_, locs, e1) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           NewLineDecor(Hint(&locs[2], " "), nl), "end")
            }
            IfThenB(_, locs, e1, b1) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IndentDecor(1), NewLineDecor(Hint(&locs[2], " "), nl), b1,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[3], " "), nl), "end")
            }
            IfThenElse(_, locs, e1) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), "else", NewLineDecor(Hint(&locs[3], " "), nl), "end")
            }
            IfThenBElse(_, locs, e1, b1) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IndentDecor(1), NewLineDecor(Hint(&locs[2], " "), nl), b1,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[3], " "), nl), "else",
                           NewLineDecor(Hint(&locs[4], " "), nl), "end")
            }
            IfThenElseB(_, locs, e1, b2) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), "else", IndentDecor(1), NewLineDecor(Hint(&locs[3], " "), nl), b2,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[4], " "), nl), "end")
            }
            IfThenBElseB(_, locs, e1, b1, b2) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IndentDecor(1), NewLineDecor(Hint(&locs[2], " "), nl), b1,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[3], " "), nl), "else",
                           IndentDecor(1), NewLineDecor(Hint(&locs[4], " "), nl), b2,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[5], " "), nl), "end")
            }
            IfThenElseIf(_, locs, e1, n) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           NewLineDecor(Hint(&locs[2], " "), nl), n,
                           NewLineDecor(Hint(&locs[3], " "), nl), "end")
            }
            IfThenBElseIf(_, locs, e1, b1, n) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IndentDecor(1), NewLineDecor(Hint(&locs[2], " "), nl), b1,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[3], " "), nl), n,
                           NewLineDecor(Hint(&locs[4], " "), nl), "end")
            }
            IfThenElseIfElse(_, locs, e1, n) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           NewLineDecor(Hint(&locs[2], " "), nl), n,
                           NewLineDecor(Hint(&locs[3], " "), nl), "else",
                           NewLineDecor(Hint(&locs[4], " "), nl), "end")
            }
            IfThenBElseIfElse(_, locs, e1, b1, n) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IndentDecor(1), NewLineDecor(Hint(&locs[2], " "), nl), b1,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[3], " "), nl), n,
                           NewLineDecor(Hint(&locs[4], " "), nl), "else",
                           NewLineDecor(Hint(&locs[5], " "), nl), "end")
            }
            IfThenElseIfElseB(_, locs, e1, n, b2) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           NewLineDecor(Hint(&locs[2], " "), nl), n,
                           NewLineDecor(Hint(&locs[3], " "), nl), "else",
                           IndentDecor(1), NewLineDecor(Hint(&locs[4], " "), nl), b2,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[5], " "), nl), "end")
            }
            IfThenBElseIfElseB(_, locs, e1, b1, n, b2) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IndentDecor(1), NewLineDecor(Hint(&locs[2], " "), nl), b1,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[3], " "), nl), n,
                           NewLineDecor(Hint(&locs[4], " "), nl), "else",
                           IndentDecor(1), NewLineDecor(Hint(&locs[5], " "), nl), b2,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[6], " "), nl), "end")
            }
            ElseIfThenVec(_, elems) => cfg_write_list_items(f, cfg, buf, state, elems),
            ElseIfThen(_, locs, e) => {
                cfg_write!(f, cfg, buf, state, "elseif", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "then")
            }
            ElseIfThenB(_, locs, e, b) => {
                let nl = cfg.indentation_string.is_some() && cfg.if_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "elseif", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "then",
                            IndentDecor(1), NewLineDecor(Hint(&locs[2], " "), nl), b, IndentDecor(-1))
            }

            Name(_, s) => write!(f, "{}", s),
            Label(_, locs, n) => cfg_write!(f, cfg, buf, state, "::", Hint(&locs[0], ""), n, Hint(&locs[1], ""), "::"),
            GoTo(_, locs, n) => cfg_write!(f, cfg, buf, state, "goto", Hint(&locs[0], " "), n),
            WhileDo(_, locs, e) => {
                let nl = cfg.indentation_string.is_some() && cfg.while_do_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "while", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "do",
                           NewLineDecor(Hint(&locs[2], " "), nl), "end")
            }
            WhileDoB(_, locs, e, n) => {
                let nl = cfg.indentation_string.is_some() && cfg.while_do_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "while", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "do",
                           IndentDecor(1), NewLineDecor(Hint(&locs[2], " "), nl), n,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[3], " "), nl), "end")
            }
            RepeatUntil(_, locs, e) => {
                let nl = cfg.indentation_string.is_some() && cfg.repeat_until_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "repeat", NewLineDecor(Hint(&locs[0], " "), nl), "until", Hint(&locs[1], " "), e)
            }
            RepeatBUntil(_, locs, b, e) => {
                let nl = cfg.indentation_string.is_some() && cfg.repeat_until_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "repeat",
                           IndentDecor(1), NewLineDecor(Hint(&locs[0], " "), nl), b,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[1], " "), nl), "until",
                           Hint(&locs[2], " "), e)
            }

            ForInt(_, locs, n, e1, e2) => {
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2, Hint(&locs[5], " "),
                           "do", Hint(&locs[6], " "), "end")
            }
            ForIntB(_, locs, n, e1, e2, b) => {
                let nl = cfg.indentation_string.is_some() && cfg.for_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                           Hint(&locs[5], " "), "do", IndentDecor(1), NewLineDecor(Hint(&locs[6], " "), nl), b,
                           IndentDecor(-1), NewLineDecor(Hint(&locs[7], " "), nl), "end")
            }
            ForIntStep(_, locs, n, e1, e2, e3) => {
                let nl = cfg.indentation_string.is_some() && cfg.for_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2, Hint(&locs[5], ""),
                           ",", Hint(&locs[6], " "), e3, Hint(&locs[7], " "), "do", NewLineDecor(Hint(&locs[8], " "), nl),
                           "end")
            },
            ForIntStepB(_, locs, n, e1, e2, e3, b) => {
                let nl = cfg.indentation_string.is_some() && cfg.for_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                           Hint(&locs[5], ""), ",", Hint(&locs[6], " "), e3, Hint(&locs[7], " "), "do",
                           IndentDecor(1), NewLineDecor(Hint(&locs[8], " "), nl), b, IndentDecor(-1),
                           NewLineDecor(Hint(&locs[9], " "), nl), "end")
            },
            ForRange(_, locs, n, e) => {
                let nl = cfg.indentation_string.is_some() && cfg.for_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "in",
                           Hint(&locs[2], " "), e, Hint(&locs[3], " "), "do", NewLineDecor(Hint(&locs[4], " "), nl),
                           "end")
            }
            ForRangeB(_, locs, n, e, b) => {
                let nl = cfg.indentation_string.is_some() && cfg.for_indent_format == Some(1);
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "in",
                           Hint(&locs[2], " "), e, Hint(&locs[3], " "), "do", IndentDecor(1),
                           NewLineDecor(Hint(&locs[4], " "), nl), b, IndentDecor(-1),
                           NewLineDecor(Hint(&locs[5], " "), nl), "end")
            }

            RetStatNone(_) => write!(f, "return"),
            RetStatExpr(_, locs, n) => cfg_write!(f, cfg, buf, state, "return", Hint(&locs[0], " "), n),
            RetStatNoneComma(_, locs) => cfg_write!(f, cfg, buf, state, "return", Hint(&locs[0], ""), ";"),
            RetStatExprComma(_, locs, n) => {
                cfg_write!(f, cfg, buf, state, "return", Hint(&locs[0], " "), n, Hint(&locs[1], ""), ";")
            }
            StatsRetStat(_, locs, n1, n2) => {
                let nl = cfg.indentation_string.is_some() && cfg.indent_every_statement == Some(true);
                cfg_write!(f, cfg, buf, state, n1, NewLineDecor(Hint(&locs[0], " "), nl), n2)
            }
            Chunk(locl, n, locr) => cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, Hint(&locr, "")),
            SheBangChunk(locl, n, locm, b, locr) => {
                cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, Hint(&locm, ""), b, Hint(&locr, ""))
            }

            Semicolon(_) => write!(f, ";"),
            SheBang(_, s) => write!(f, "{}\n", s),
        }
    }
}
