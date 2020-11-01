use std::cell::Cell;
use std::fmt::Write;

use super::common::*;
use crate::config::*;
use crate::formatting::decoration::*;
use crate::formatting::list;
use crate::formatting::loc_hint::*;
use crate::formatting::util;
use crate::{
    cfg_write, cfg_write_helper, out_of_range_only_write, out_of_range_write, test_oneline, test_oneline_no_nl,
};

#[derive(Debug)]
pub struct TableConstructorOpts {
    pub is_iv_table: Option<bool>,
    pub is_single_child: Option<bool>,
    pub is_oneline: Cell<bool>,
}

impl TableConstructorOpts {
    pub const fn default() -> Self {
        TableConstructorOpts { is_iv_table: None, is_single_child: None, is_oneline: Cell::new(false) }
    }
}

#[derive(Debug)]
pub struct FieldsOpts {
    pub is_iv_table: Option<bool>,
    pub is_single_child: Option<bool>,
    pub has_indent: Cell<bool>,
}

impl FieldsOpts {
    pub const fn default() -> Self {
        FieldsOpts { is_iv_table: None, is_single_child: None, has_indent: Cell::new(false) }
    }
}

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

    TableConstructor(Loc, [Loc; 2], Box<Node>, TableConstructorOpts),
    TableConstructorEmpty(Loc, [Loc; 1]),
    Fields(Loc, Vec<(Loc, Node, Loc, String)>, FieldsOpts),
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

impl<'a> list::AnyListItem<'a, Node> for Node {
    fn list_item_prefix_hint(&self, _: &'a Config) -> &'a str {
        use Node::*;
        match self {
            Semicolon(..)
            | ArgsRoundBrackets(..)
            | ArgsRoundBracketsEmpty(..)
            | TableIndex(..)
            | TableMember(..)
            | FnMethodCall(..)
            | TableConstructor(..)
            | TableConstructorEmpty(..) => "",
            _ => " ",
        }
    }

    fn need_newline(
        &self, prev: &Node, parent: &Node, f: &mut String, cfg: &Config, buf: &str, state: &mut State,
    ) -> bool {
        use Node::*;
        match parent {
            StatementList(..) => cfg.fmt.newline_format_statement.is_some(),
            ExpList(..) => match cfg.fmt.newline_format_exp_list {
                Some(1) => match cfg.fmt.enable_oneline_exp_list {
                    Some(true) => test_oneline!(f, cfg, buf, state, self).is_none(),
                    _ => true,
                },
                _ => false,
            },
            ElseIfThenVec(..) => cfg.fmt.newline_format_if == Some(1),
            Fields(_, _, opts) => match cfg.fmt.newline_format_table_field.is_some() {
                true => {
                    if opts.is_iv_table == Some(true) && cfg.fmt.enable_oneline_iv_table_field == Some(true) {
                        if cfg.fmt.enable_oneline_table == Some(true) {
                            if let FieldSequential(_, nseq) = prev {
                                if let TableConstructor(_, _, _, prev_opts) = &**nseq {
                                    return prev_opts.is_oneline.get()
                                        || test_oneline_no_nl!(f, cfg, buf, state, self).is_some();
                                }

                                return test_oneline_no_nl!(f, cfg, buf, state, self).is_none();
                            }
                        }

                        return false;
                    }

                    if cfg.fmt.enable_oneline_kv_table_field == Some(true) && opts.is_iv_table == Some(false) {
                        return self.test_oneline_table_field(f, cfg, buf, state).is_none();
                    }

                    return true;
                }
                _ => false,
            },
            VarSuffixList(..) => match self {
                FnMethodCall(..) | TableMember(..) => match cfg.fmt.newline_format_var_suffix.is_some() {
                    true => match cfg.fmt.enable_oneline_var_suffix {
                        Some(true) => match self {
                            FnMethodCall(_, locs, n1, n2) => match &**n2 {
                                TableConstructorEmpty(..) | TableConstructor(..) | ArgsRoundBracketsEmpty(..) => {
                                    test_oneline!(f, cfg, buf, state, ":", CommentLocHint(&locs[0], ""), n1).is_none()
                                }
                                ArgsRoundBrackets(..) if cfg.fmt.newline_format_exp_list_first.is_some() => {
                                    test_oneline!(f, cfg, buf, state, ":", CommentLocHint(&locs[0], ""), n1).is_none()
                                }
                                _ => test_oneline!(f, cfg, buf, state, self).is_none(),
                            },
                            TableMember(..) => test_oneline!(f, cfg, buf, state, self).is_none(),
                            _ => false,
                        },
                        _ => true,
                    },
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }

    fn need_first_newline(&self, parent: &Node, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> bool {
        use Node::*;
        match parent {
            Fields(_, _, opts) => match cfg.fmt.newline_format_table_field.is_some() {
                true => {
                    if cfg.fmt.enable_oneline_table == Some(true)
                        && opts.is_iv_table == Some(true)
                        && cfg.fmt.enable_oneline_iv_table_field == Some(true)
                    {
                        return opts.has_indent.get() || opts.is_single_child != Some(true);
                    }

                    if cfg.fmt.enable_oneline_iv_table_field == Some(true) && opts.is_iv_table == Some(true)
                        || cfg.fmt.enable_oneline_kv_table_field == Some(true) && opts.is_iv_table == Some(false)
                    {
                        return self.test_oneline_table_field(f, cfg, buf, state) == None;
                    }

                    true
                }
                _ => false,
            },
            ExpList(..) => match cfg.fmt.newline_format_exp_list_first {
                // first!
                Some(1) => match cfg.fmt.enable_oneline_exp_list {
                    Some(true) => test_oneline!(f, cfg, buf, state, self).is_none(),
                    _ => true,
                },
                _ => false,
            },
            VarSuffixList(span, _) => {
                self.need_newline(&Node::Name(Loc(span.0, span.0), String::new()), parent, f, cfg, buf, state)
            }
            _ => false,
        }
    }
}

impl list::SepListOfItems<Node> for Node {
    fn items(&self) -> Option<&Vec<(Loc, Node, Loc, String)>> {
        use Node::*;
        match self {
            Fields(_, items, _)
            | ExpList(_, items)
            | NameList(_, items)
            | VarList(_, items)
            | ParList(_, items)
            | FuncName(_, items)
            | FuncNameSelf(_, _, items, _) => Some(items),
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
            Fields(..) => cfg.fmt.field_separator.clone(),
            ExpList(..) | NameList(..) | VarList(..) | ParList(..) => Some(",".to_string()),
            FuncName(..) | FuncNameSelf(..) => Some(".".to_string()),
            _ => None,
        }
    }

    fn trailing_separator(&self, cfg: &Config) -> Option<bool> {
        use Node::*;
        match self {
            Fields(_, _, opts) => {
                if cfg.fmt.enable_oneline_table == Some(true)
                    && (cfg.fmt.newline_format_table_field.is_none()
                        && cfg.fmt.newline_format_table_constructor.is_none()
                        || opts.is_iv_table == Some(true) && !opts.has_indent.get())
                {
                    Some(false)
                } else {
                    cfg.fmt.write_trailing_field_separator.clone()
                }
            }
            ExpList(..) | NameList(..) | VarList(..) | ParList(..) | FuncName(..) | FuncNameSelf(..) => Some(false),
            _ => None,
        }
    }

    fn need_newlines(&self, cfg: &Config) -> bool {
        use Node::*;
        match self {
            Fields(_, _, _) => cfg.fmt.newline_format_table_field.is_some(),
            ExpList(..) => cfg.fmt.newline_format_exp_list.is_some() || cfg.fmt.newline_format_exp_list_first.is_some(),
            _ => false,
        }
    }
}

impl list::ListOfItems<Node> for Node {
    fn items(&self) -> Option<&Vec<(Loc, Node)>> {
        use Node::*;
        match self {
            StatementList(_, items) | VarSuffixList(_, items) | ElseIfThenVec(_, items) => Some(items),
            _ => None,
        }
    }

    fn need_newlines(&self, cfg: &Config) -> bool {
        use Node::*;
        match self {
            StatementList(..) => cfg.fmt.newline_format_statement.is_some(),
            ElseIfThenVec(..) => cfg.fmt.newline_format_if.is_some(),
            VarSuffixList(..) => cfg.fmt.newline_format_var_suffix.is_some(),
            _ => false,
        }
    }
}

impl Node {
    fn test_oneline_table_cfg(&self, cfg: &Config) -> Option<Config> {
        if let Node::TableConstructor(..) = self {
            if cfg.fmt.max_width.is_some()
                && cfg.fmt.newline_format_table_constructor.is_some()
                && cfg.fmt.enable_oneline_table == Some(true)
            {
                // disable IfNewLine within table constructor
                // one-line tables are forced to have no trailing separator
                let mut test_cfg = cfg.clone();
                test_cfg.fmt.newline_format_table_constructor = None;
                test_cfg.fmt.newline_format_table_field = None;
                test_cfg.fmt.write_trailing_field_separator = Some(false);

                return Some(test_cfg);
            }
        }
        None
    }

    fn test_oneline_table_field(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> Option<String> {
        if cfg.fmt.max_width.is_some() && cfg.fmt.newline_format_table_field.is_some() {
            return test_oneline_no_nl!(f, cfg, buf, state, self);
        }
        None
    }

    fn test_oneline_if(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> Option<String> {
        if cfg.fmt.max_width.is_some() && cfg.fmt.enable_oneline_if == Some(true) && cfg.fmt.newline_format_if.is_some()
        {
            // disable IfNewLine within table constructor
            let mut test_cfg = cfg.clone();
            test_cfg.fmt.newline_format_if = None;

            return test_oneline_no_nl!(f, &test_cfg, buf, state, self);
        }
        None
    }

    fn test_oneline_function(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> Option<String> {
        if cfg.fmt.max_width.is_some()
            && cfg.fmt.newline_format_function.is_some()
            && ((state.function_nested_level == 0 && cfg.fmt.enable_oneline_top_level_function == Some(true))
                || (state.function_nested_level > 0 && cfg.fmt.enable_oneline_scoped_function == Some(true)))
        {
            // disable IfNewLine within function body
            let mut test_cfg = cfg.clone();
            test_cfg.fmt.newline_format_function = None;

            return test_oneline_no_nl!(f, &test_cfg, buf, state, self);
        }
        None
    }

    fn test_indent(
        &self, f: &mut String, cfg: &Config, buf: &str, state: &mut State, hint: CommentLocHint,
    ) -> Result<bool, std::fmt::Error> {
        let cfg_write_sep_list = list::cfg_write_sep_list::<Node, CommentLocHint>;
        let cfg_write_list = list::cfg_write_list::<Node, CommentLocHint>;

        let ind = match self {
            Node::VarSuffixList(_, suffs) => match suffs.len() {
                0 => false,
                _ => {
                    let mut test_f = f.clone();
                    let mut test_state = state.clone();
                    cfg_write!(&mut test_f, cfg, buf, &mut test_state, hint)?;
                    cfg.fmt.indent_var_suffix == Some(true)
                        && (cfg.fmt.indent_one_line_var_suffix == Some(true)
                            || cfg_write_list(&mut test_f, cfg, buf, &mut test_state, self) == Ok(true))
                }
            },
            Node::ExpList(_, exprs) => match exprs.len() {
                0 => false,
                // ExpList cannot indent it's first item without this flag
                1 if cfg.fmt.newline_format_exp_list_first.is_none() => {
                    cfg.fmt.indent_exp_list == Some(true) && cfg.fmt.indent_one_line_exp_list == Some(true)
                }
                _ => {
                    let mut test_f = f.clone();
                    let mut test_state = state.clone();
                    cfg_write!(&mut test_f, cfg, buf, &mut test_state, hint)?;
                    cfg.fmt.indent_exp_list == Some(true)
                        && (cfg.fmt.indent_one_line_exp_list == Some(true)
                            || cfg_write_sep_list(&mut test_f, cfg, buf, &mut test_state, self) == Ok(true))
                }
            },
            Node::Fields(_, fields, _) => match fields.len() {
                0 => false,
                _ => {
                    let mut test_f = f.clone();
                    let mut test_state = state.clone();
                    cfg_write!(&mut test_f, cfg, buf, &mut test_state, hint)?;

                    cfg_write_sep_list(&mut test_f, cfg, buf, &mut test_state, self) == Ok(true)
                }
            },
            _ => false,
        };

        Ok(ind)
    }
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> std::fmt::Result {
        use Node::*;

        #[allow(non_snake_case)]
        let Hint = CommentLocHint;
        let cfg_write_list = list::cfg_write_list::<Node, CommentLocHint>;
        let cfg_write_sep_list = list::cfg_write_sep_list::<Node, CommentLocHint>;

        match self {
            BinaryOp(span, locs, tok, l, r) => {
                out_of_range_write!(f, cfg, buf, state, span, l, locs[0], tok, locs[1], r);

                cfg_write!(f, cfg, buf, state, IncIndent(Some(tok.0)), l)?;

                let mut nl1 = cfg.fmt.newline_format_binary_op == Some(1);
                let mut nl2 = cfg.fmt.newline_format_binary_op == Some(2);
                if (nl1 || nl2) && cfg.fmt.enable_oneline_binary_op == Some(true) {
                    if test_oneline!(f, cfg, buf, state, Hint(&locs[0], " "), tok, Hint(&locs[1], " "), r).is_some() {
                        nl1 = false;
                        nl2 = false;
                    }
                }

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, IfNewLine(nl1, Hint(&locs[0], " ")), tok,
                           IfNewLine(nl2, Hint(&locs[1], " ")), r, DecIndent())
            }
            UnaryOp(span, locs, tok, r) => {
                out_of_range_write!(f, cfg, buf, state, span, tok, locs[0], r);

                cfg_write!(f, cfg, buf, state, tok, Hint(&locs[0], ""), r)
            }
            UnaryNot(span, locs, r) => {
                out_of_range_write!(f, cfg, buf, state, span, "not", locs[0], r);

                cfg_write!(f, cfg, buf, state, "not", Hint(&locs[0], " "), r)
            }

            Var(span, locs, n1, n2) => {
                out_of_range_write!(f, cfg, buf, state, span, n1, locs[0], n2);

                cfg_write!(f, cfg, buf, state, n1)?;
                let ind = n2.test_indent(f, cfg, buf, state, Hint(&locs[0], "")) == Ok(true);
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[0], ""), n2, If(ind, &DecIndent()))
            }
            RoundBrackets(span, locs, r) => {
                out_of_range_write!(f, cfg, buf, state, span, "(", locs[0], r, locs[1], ")");

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), r, Hint(&locs[1], ""), ")")
            }
            ArgsRoundBrackets(span, locs, r) => {
                out_of_range_write!(f, cfg, buf, state, span, "(", locs[0], r, locs[1], ")");

                cfg_write!(f, cfg, buf, state, "(")?;
                let ind = r.test_indent(f, cfg, buf, state, Hint(&locs[0], "")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[0], ""), r, If(ind, &DecIndent()),
                           Hint(&locs[1], ""), ")")
            }
            ArgsRoundBracketsEmpty(span, locs) => {
                out_of_range_write!(f, cfg, buf, state, span, "(", locs[0], ")");
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")")
            }

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

            TableConstructor(span, locs, n, opts) => {
                out_of_range_write!(f, cfg, buf, state, span, "{{", locs[0], n, locs[1], "}}");

                if let Some(test_cfg) = self.test_oneline_table_cfg(cfg) {
                    match test_oneline_no_nl!(f, &test_cfg, buf, state, self) {
                        Some(line) => {
                            opts.is_oneline.set(true);

                            #[cfg_attr(rustfmt, rustfmt_skip)]
                            return cfg_write!(f, cfg, buf, state, Str(&line));
                        }
                        _ => {}
                    }
                }

                let default_hint = String::new();
                let hint = cfg.fmt.hint_table_constructor.as_ref().unwrap_or(&default_hint);
                let mut nl = cfg.fmt.newline_format_table_constructor == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "{{", IncFuncLevel())?;

                let ind = match cfg.fmt.enable_oneline_table == Some(true)
                    && opts.is_iv_table == Some(true)
                    && opts.is_single_child == Some(true)
                {
                    false => true,
                    true => n.test_indent(f, cfg, buf, state, Hint(&locs[0], &hint)) == Ok(true),
                };

                if let Fields(_, _, fopts) = &**n {
                    fopts.has_indent.set(ind);
                }

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[0], &hint), n)?;

                if cfg.fmt.max_width.is_some() && util::get_len_after_newline(f, cfg) >= cfg.fmt.max_width.unwrap() {
                    nl = true;
                }
                nl = nl && ind;

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &DecIndent()), DecFuncLevel(),
                           IfNewLine(nl, Hint(&locs[1], &hint)), "}}")
            }
            TableConstructorEmpty(span, locs) => {
                out_of_range_write!(f, cfg, buf, state, span, "{{", locs[0], "}}");

                let default_hint = String::new();
                let hint = cfg.fmt.hint_table_constructor.as_ref().unwrap_or(&default_hint);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "{{", Hint(&locs[0], &hint), "}}")
            }
            Fields(span, _, _) => {
                out_of_range_only_write!(f, cfg, buf, state, span);

                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            FieldNamedBracket(span, locs, e1, e2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "[", locs[0], e1, locs[1], "]", locs[2], "=", locs[3], e2);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "[", Hint(&locs[0], ""), e1, Hint(&locs[1], ""), "]", Hint(&locs[2], " "),
                           "=", Hint(&locs[3], " "), e2)
            }
            FieldNamed(span, locs, e1, e2) => {
                out_of_range_write!(f, cfg, buf, state, span, e1, locs[0], "=", locs[1], e2);
                cfg_write!(f, cfg, buf, state, e1, Hint(&locs[0], " "), "=", Hint(&locs[1], " "), e2)
            }
            FieldSequential(span, e) => {
                out_of_range_write!(f, cfg, buf, state, span, e);
                cfg_write!(f, cfg, buf, state, e)
            }

            TableIndex(span, locs, e) => {
                out_of_range_write!(f, cfg, buf, state, span, "[", locs[0], e, locs[1], "]");
                cfg_write!(f, cfg, buf, state, "[", Hint(&locs[0], ""), e, Hint(&locs[1], ""), "]")
            }
            TableMember(span, locs, n) => {
                out_of_range_write!(f, cfg, buf, state, span, ".", locs[0], n);
                cfg_write!(f, cfg, buf, state, ".", Hint(&locs[0], ""), n)
            }
            ExpList(span, _) => {
                out_of_range_only_write!(f, cfg, buf, state, span);
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            NameList(span, _) => {
                out_of_range_only_write!(f, cfg, buf, state, span);
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            VarList(span, _) => {
                out_of_range_only_write!(f, cfg, buf, state, span);
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            StatementList(span, _) => {
                out_of_range_only_write!(f, cfg, buf, state, span);
                cfg_write_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            DoEnd(span, locs) => {
                out_of_range_write!(f, cfg, buf, state, span, "do", locs[0], "end");
                cfg_write!(f, cfg, buf, state, "do", Hint(&locs[0], " "), "end")
            }
            DoBEnd(span, locs, b) => {
                out_of_range_write!(f, cfg, buf, state, span, "do", locs[0], b, locs[1], "end");

                let nl = cfg.fmt.newline_format_do_end == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "do", IncIndent(None), IfNewLine(nl, Hint(&locs[0], " ")), b, DecIndent(),
                           IfNewLine(nl, Hint(&locs[1], " ")), "end")
            }
            VarsExprs(span, locs, n1, n2) => {
                out_of_range_write!(f, cfg, buf, state, span, n1, locs[0], "=", locs[1], n2);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, n1, Hint(&locs[0], " "), "=")?;
                let ind = n2.test_indent(f, cfg, buf, state, Hint(&locs[1], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[1], " "), n2, If(ind, &DecIndent()))
            }

            VarRoundSuffix(span, locs, n1, n2) => {
                out_of_range_write!(f, cfg, buf, state, span, "(", locs[0], n1, locs[1], ")");

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), ")")?;

                let ind = n2.test_indent(f, cfg, buf, state, Hint(&locs[0], "")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[2], ""), n2, If(ind, &DecIndent()))
            }
            VarSuffixList(span, _) => {
                out_of_range_only_write!(f, cfg, buf, state, span);
                cfg_write_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            FnMethodCall(span, locs, n1, n2) => {
                out_of_range_write!(f, cfg, buf, state, span, ":", locs[0], n1, locs[1], n2);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, ":", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), n2)
            }
            ParList(span, _) => {
                out_of_range_only_write!(f, cfg, buf, state, span);
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            FunctionDef(span, locs, n) => {
                out_of_range_write!(f, cfg, buf, state, span, "function", locs[0], n);
                cfg_write!(f, cfg, buf, state, "function", Hint(&locs[0], ""), n)
            }
            FuncBody(span, locs) => {
                out_of_range_write!(f, cfg, buf, state, span, "(", locs[0], ")", locs[1], "end");

                if let Some(line) = self.test_oneline_function(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_function == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")", IfNewLine(nl, Hint(&locs[1], " ")), "end")
            }
            FuncBodyB(span, locs, n2) => {
                out_of_range_write!(f, cfg, buf, state, span, "(", locs[0], ")", locs[1], n2, locs[2], "end");

                if let Some(line) = self.test_oneline_function(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_function == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")", IncIndent(None), IncFuncLevel(),
                           IfNewLine(nl, Hint(&locs[1], " ")), n2, DecIndent(), DecFuncLevel(),
                           IfNewLine(nl, Hint(&locs[2], " ")), "end")
            }
            FuncPBody(span, locs, n1) => {
                out_of_range_write!(f, cfg, buf, state, span, "(", locs[0], n1, locs[1], ")", locs[2], "end");

                if let Some(line) = self.test_oneline_function(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_function == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", IncIndent(None), Hint(&locs[0], ""), n1, Hint(&locs[1], ""),
                           DecIndent(), ")", IfNewLine(nl, Hint(&locs[2], " ")), "end")
            }
            FuncPBodyB(span, locs, n1, n2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "(", locs[0], n1, locs[1], ")", locs[2], n2, locs[3], "end");

                if let Some(line) = self.test_oneline_function(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_function == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", IncIndent(None), Hint(&locs[0], ""), n1, Hint(&locs[1], ""),
                           DecIndent(), ")", IncIndent(None), IncFuncLevel(), IfNewLine(nl, Hint(&locs[2], " ")), n2,
                           DecIndent(), DecFuncLevel(), IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            FuncName(span, _) => {
                out_of_range_only_write!(f, cfg, buf, state, span);
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            FuncNameSelf(span, locs, _, n) => {
                out_of_range_only_write!(f, cfg, buf, state, span);

                cfg_write_sep_list(f, cfg, buf, state, self)?;

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, Hint(&locs[0], ""), ":", Hint(&locs[1], ""), n)
            }
            FuncDecl(span, locs, n1, n2) => {
                out_of_range_write!(f, cfg, buf, state, span, "function", locs[0], n1, locs[1], n2);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "function", Hint(&locs[0], " "), n1, Hint(&locs[1], ""), n2)
            }
            LocalFuncDecl(span, locs, n1, n2) => {
                out_of_range_write!(f, cfg, buf, state, span, "local", locs[0], "function", locs[1], n1, locs[2], n2);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), "function", Hint(&locs[1], " "), n1,
                           Hint(&locs[2], ""), n2)
            }

            LocalNames(span, locs, n) => {
                out_of_range_write!(f, cfg, buf, state, span, "local", locs[0], n);

                cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), n)
            }
            LocalNamesExprs(span, locs, n1, n2) => {
                out_of_range_write!(f, cfg, buf, state, span, "local", locs[0], n1, locs[1], "=", locs[2], n2);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), n1, Hint(&locs[1], " "), "=")?;
                let ind = n2.test_indent(f, cfg, buf, state, Hint(&locs[2], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[2], " "), n2,
                           If(ind, &DecIndent()))
            }

            // if
            IfThen(span, locs, e1) => {
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IfNewLine(nl, Hint(&locs[2], " ")), "end")
            }
            IfThenB(span, locs, e1, b1) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], b1, locs[3],
                                    "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            IfThenElse(span, locs, e1) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], "else",
                                    locs[3], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), "else", IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            IfThenBElse(span, locs, e1, b1) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], b1, locs[3],
                                    "else", locs[4], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), "else", IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            IfThenElseB(span, locs, e1, b2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], "else",
                                    locs[3], b2, locs[4], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), "else", IncIndent(None), IfNewLine(nl, Hint(&locs[3], " ")), b2,
                           DecIndent(), IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            IfThenBElseB(span, locs, e1, b1, b2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], b1, locs[3],
                                    "else", locs[4], b2, locs[5], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), "else", IncIndent(None),
                           IfNewLine(nl, Hint(&locs[4], " ")), b2, DecIndent(),
                           IfNewLine(nl, Hint(&locs[5], " ")), "end")
            }
            IfThenElseIf(span, locs, e1, n) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], n, locs[3],
                                    "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IfNewLine(nl, Hint(&locs[2], " ")), n, IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            IfThenBElseIf(span, locs, e1, b1, n) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], b1, locs[3],
                                    n, locs[4], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), n, IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            IfThenElseIfElse(span, locs, e1, n) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], n, locs[3],
                                    "else", locs[4], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IfNewLine(nl, Hint(&locs[2], " ")), n, IfNewLine(nl, Hint(&locs[3], " ")), "else",
                           IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            IfThenBElseIfElse(span, locs, e1, b1, n) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], b1, locs[3],
                                    n, locs[4], "else", locs[5], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), n, IfNewLine(nl, Hint(&locs[4], " ")), "else",
                           IfNewLine(nl, Hint(&locs[5], " ")), "end")
            }
            IfThenElseIfElseB(span, locs, e1, n, b2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], n, locs[3],
                                    "else", locs[4], b2, locs[5], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IfNewLine(nl, Hint(&locs[2], " ")), n, IfNewLine(nl, Hint(&locs[3], " ")), "else",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[4], " ")), b2, DecIndent(),
                           IfNewLine(nl, Hint(&locs[5], " ")), "end")
            }
            IfThenBElseIfElseB(span, locs, e1, b1, n, b2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "if", locs[0], e1, locs[1], "then", locs[2], b1, locs[3],
                                    n, locs[4], "else", locs[5], b2, locs[6], "end");

                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), n, IfNewLine(nl, Hint(&locs[4], " ")), "else",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[5], " ")), b2, DecIndent(),
                           IfNewLine(nl, Hint(&locs[6], " ")), "end")
            }
            ElseIfThenVec(span, _) => {
                out_of_range_only_write!(f, cfg, buf, state, span);
                cfg_write_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            ElseIfThen(span, locs, e) => {
                out_of_range_write!(f, cfg, buf, state, span, "elseif", locs[0], e, locs[1], "then");

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "elseif", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "then")
            }
            ElseIfThenB(span, locs, e, b) => {
                out_of_range_write!(f, cfg, buf, state, span, "elseif", locs[0], e, locs[1], "then", locs[2], b);

                let nl = cfg.fmt.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "elseif", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b, DecIndent())
            }

            Name(_, s) => write!(f, "{}", s),
            Label(span, locs, n) => {
                out_of_range_write!(f, cfg, buf, state, span, "::", locs[0], n, locs[1], "::");
                cfg_write!(f, cfg, buf, state, "::", Hint(&locs[0], ""), n, Hint(&locs[1], ""), "::")
            }
            GoTo(span, locs, n) => {
                out_of_range_write!(f, cfg, buf, state, span, "goto", locs[0], n);
                cfg_write!(f, cfg, buf, state, "goto", Hint(&locs[0], " "), n)
            }
            WhileDo(span, locs, e) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "while", locs[0], e, locs[1], "do", locs[2], "end");

                let nl = cfg.fmt.newline_format_while == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "while", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "do",
                           IfNewLine(nl, Hint(&locs[2], " ")), "end")
            }
            WhileDoB(span, locs, e, n) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "while", locs[0], e, locs[1], "do", locs[2], n, locs[3],
                                    "end");

                let nl = cfg.fmt.newline_format_while == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "while", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "do",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), n, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            RepeatUntil(span, locs, e) => {
                out_of_range_write!(f, cfg, buf, state, span, "repeat", locs[0], "until", locs[1], e);

                let nl = cfg.fmt.newline_format_repeat_until == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "repeat", IfNewLine(nl, Hint(&locs[0], " ")), "until",
                           Hint(&locs[1], " "), e)
            }
            RepeatBUntil(span, locs, b, e) => {
                out_of_range_write!(f, cfg, buf, state, span, "repeat", locs[0], b, locs[1], "until", locs[2], e);

                let nl = cfg.fmt.newline_format_repeat_until == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "repeat", IncIndent(None), IfNewLine(nl, Hint(&locs[0], " ")), b,
                           DecIndent(), IfNewLine(nl, Hint(&locs[1], " ")), "until", Hint(&locs[2], " "), e)
            }

            ForInt(span, locs, n, e1, e2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "for", locs[0], n, locs[1], "=", locs[2], e1, locs[3], ",",
                                    locs[4], e2, locs[5], "do", locs[6], "end");

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                           Hint(&locs[5], " "), "do", Hint(&locs[6], " "), "end")
            }
            ForIntB(span, locs, n, e1, e2, b) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "for", locs[0], n, locs[1], "=", locs[2], e1, locs[3], ",",
                                    locs[4], e2, locs[5], "do", locs[6], b, locs[7], "end");

                let nl = cfg.fmt.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                           Hint(&locs[5], " "), "do", IncIndent(None), IfNewLine(nl, Hint(&locs[6], " ")), b,
                           DecIndent(), IfNewLine(nl, Hint(&locs[7], " ")), "end")
            }
            ForIntStep(span, locs, n, e1, e2, e3) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "for", locs[0], n, locs[1], "=", locs[2], e1, locs[3], ",",
                                    locs[4], e2, locs[5], ",", locs[6], e3, locs[7], "do", locs[8], "end");

                let nl = cfg.fmt.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2, Hint(&locs[5], ""),
                           ",", Hint(&locs[6], " "), e3, Hint(&locs[7], " "), "do", IfNewLine(nl, Hint(&locs[8], " ")),
                           "end")
            }
            ForIntStepB(span, locs, n, e1, e2, e3, b) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "for", locs[0], n, locs[1], "=", locs[2], e1, locs[3], ",",
                                    locs[4], e2, locs[5], ",", locs[6], e3, locs[7], "do", locs[8], b, locs[9], "end");

                let nl = cfg.fmt.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2, Hint(&locs[5], ""),
                           ",", Hint(&locs[6], " "), e3, Hint(&locs[7], " "), "do", IncIndent(None),
                           IfNewLine(nl, Hint(&locs[8], " ")), b, DecIndent(), IfNewLine(nl, Hint(&locs[9], " ")), "end")
            }
            ForRange(span, locs, n, e) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "for", locs[0], n, locs[1], "in", locs[2], e, locs[3], "do",
                                    locs[4], "end");

                let nl = cfg.fmt.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "in")?;

                let ind = e.test_indent(f, cfg, buf, state, Hint(&locs[2], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[2], " "), e, If(ind, &DecIndent()),
                           Hint(&locs[3], " "), "do", IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            ForRangeB(span, locs, n, e, b) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                out_of_range_write!(f, cfg, buf, state, span, "for", locs[0], n, locs[1], "in", locs[2], e, locs[3], "do",
                                    locs[4], b, locs[5], "end");

                let nl = cfg.fmt.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "in")?;
                let ind = e.test_indent(f, cfg, buf, state, Hint(&locs[2], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[2], " "), e, If(ind, &DecIndent()),
                           Hint(&locs[3], " "), "do", IncIndent(None), IfNewLine(nl, Hint(&locs[4], " ")), b,
                           DecIndent(), IfNewLine(nl, Hint(&locs[5], " ")), "end")
            }

            RetStatNone(_) => write!(f, "return"),
            RetStatExpr(span, locs, n) => {
                out_of_range_write!(f, cfg, buf, state, span, "return", locs[0], n);

                cfg_write!(f, cfg, buf, state, "return")?;
                let ind = n.test_indent(f, cfg, buf, state, Hint(&locs[0], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[0], " "), n, If(ind, &DecIndent()))
            }
            RetStatNoneComma(span, locs) => {
                out_of_range_write!(f, cfg, buf, state, span, "return", locs[0], ";");

                cfg_write!(f, cfg, buf, state, "return", Hint(&locs[0], ""), ";")
            }
            RetStatExprComma(span, locs, n) => {
                out_of_range_write!(f, cfg, buf, state, span, "return", locs[0], n, locs[1], ";");

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "return")?;

                let ind = n.test_indent(f, cfg, buf, state, Hint(&locs[0], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[0], " "), n, Hint(&locs[1], ""),
                           If(ind, &DecIndent()), ";"
                )
            }
            StatsRetStat(span, locs, n1, n2) => {
                out_of_range_write!(f, cfg, buf, state, span, n1, locs[0], n2);

                let nl = cfg.fmt.newline_format_statement.is_some();
                cfg_write!(f, cfg, buf, state, n1, IfNewLine(nl, Hint(&locs[0], " ")), n2)
            }
            Chunk(locl, n, locr) => {
                let nl = cfg.fmt.write_newline_at_eof == Some(true);
                cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, IfNewLine(nl, Hint(&locr, "")))
            }
            SheBangChunk(locl, n, locm, b, locr) => {
                let nl = cfg.fmt.write_newline_at_eof == Some(true);
                cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, Hint(&locm, ""), b, IfNewLine(nl, Hint(&locr, "")))
            }

            Semicolon(_) => write!(f, ";"),
            SheBang(_, s) => write!(f, "{}\n", s),
        }
    }
}
