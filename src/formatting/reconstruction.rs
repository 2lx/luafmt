use crate::config::*;
use crate::formatting::util;
use crate::parser::common::Loc;
use crate::parser::lua_ast::Node;

fn update_pos_range(span: &Loc, state: &mut State) {
    use std::cmp::{max, min};

    if state.block_nested_level == 1 {
        match state.pos_range {
            Some((l, r)) if r > span.0 && l < span.1 => {
                state.pos_range = Some((min(span.0, l), max(span.1, r)));
            }
            _ => {}
        }
    }
}

pub fn reconstruct_node_tree(node: &mut Node, cfg: &Config, state: &mut State) {
    use Node::*;
    match node {
        UnaryOp(span, _, _, n)
        | UnaryNot(span, _, n)
        | RoundBrackets(span, _, n)
        | ArgsRoundBrackets(span, _, n)
        | FieldSequential(span, n)
        | TableIndex(span, _, n)
        | TableMember(span, _, n)
        | FunctionDef(span, _, n)
        | FuncBodyB(span, _, n)
        | FuncPBody(span, _, n)
        | LocalNames(span, _, n)
        | IfThen(span, _, n)
        | IfThenElse(span, _, n)
        | ElseIfThen(span, _, n)
        | Label(span, _, n)
        | GoTo(span, _, n)
        | WhileDo(span, _, n)
        | RepeatUntil(span, _, n)
        | RetStatExpr(span, _, n)
        | RetStatExprComma(span, _, n)
        | Chunk(span, n, _)
        | DoBEnd(span, _, n) => {
            update_pos_range(span, state);

            reconstruct_node_tree(&mut *n, cfg, state);
        }

        BinaryOp(span, _, _, n1, n2)
        | Var(span, _, n1, n2)
        | FieldNamedBracket(span, _, n1, n2)
        | FieldNamed(span, _, n1, n2)
        | VarRoundSuffix(span, _, n1, n2)
        | FnMethodCall(span, _, n1, n2)
        | FuncPBodyB(span, _, n1, n2)
        | FuncDecl(span, _, n1, n2)
        | LocalFuncDecl(span, _, n1, n2)
        | LocalNamesExprs(span, _, n1, n2)
        | IfThenB(span, _, n1, n2)
        | IfThenBElse(span, _, n1, n2)
        | IfThenElseB(span, _, n1, n2)
        | IfThenElseIf(span, _, n1, n2)
        | IfThenElseIfElse(span, _, n1, n2)
        | ElseIfThenB(span, _, n1, n2)
        | WhileDoB(span, _, n1, n2)
        | RepeatBUntil(span, _, n1, n2)
        | ForRange(span, _, n1, n2)
        | StatsRetStat(span, _, n1, n2)
        | SheBangChunk(span, n1, _, n2, _)
        | VarsExprs(span, _, n1, n2) => {
            update_pos_range(span, state);

            reconstruct_node_tree(&mut *n1, cfg, state);
            reconstruct_node_tree(&mut *n2, cfg, state);
        }

        IfThenBElseIf(span, _, n1, n2, n3)
        | IfThenBElseIfElse(span, _, n1, n2, n3)
        | IfThenElseIfElseB(span, _, n1, n2, n3)
        | ForInt(span, _, n1, n2, n3)
        | ForRangeB(span, _, n1, n2, n3)
        | IfThenBElseB(span, _, n1, n2, n3) => {
            update_pos_range(span, state);

            reconstruct_node_tree(&mut *n1, cfg, state);
            reconstruct_node_tree(&mut *n2, cfg, state);
            reconstruct_node_tree(&mut *n3, cfg, state);
        }

        ForIntB(span, _, n1, n2, n3, n4)
        | ForIntStep(span, _, n1, n2, n3, n4)
        | IfThenBElseIfElseB(span, _, n1, n2, n3, n4) => {
            update_pos_range(span, state);

            reconstruct_node_tree(&mut *n1, cfg, state);
            reconstruct_node_tree(&mut *n2, cfg, state);
            reconstruct_node_tree(&mut *n3, cfg, state);
            reconstruct_node_tree(&mut *n4, cfg, state);
        }

        ForIntStepB(span, _, n1, n2, n3, n4, n5) => {
            update_pos_range(span, state);

            reconstruct_node_tree(&mut *n1, cfg, state);
            reconstruct_node_tree(&mut *n2, cfg, state);
            reconstruct_node_tree(&mut *n3, cfg, state);
            reconstruct_node_tree(&mut *n4, cfg, state);
            reconstruct_node_tree(&mut *n5, cfg, state);
        }

        ArgsRoundBracketsEmpty(span, _)
        | Nil(span)
        | False(span)
        | True(span)
        | VarArg(span)
        | Break(span)
        | Numeral(span, _)
        | NormalStringLiteral(span, _)
        | MultiLineStringLiteral(span, _, _)
        | TableConstructorEmpty(span, _)
        | DoEnd(span, _)
        | Name(span, _)
        | RetStatNone(span)
        | RetStatNoneComma(span, _)
        | Semicolon(span)
        | SheBang(span, _)
        | FuncBody(span, _) => {
            update_pos_range(span, state);
        }

        ExpList(span, v) | NameList(span, v) | VarList(span, v) | ParList(span, v) | FuncName(span, v) => {
            update_pos_range(span, state);

            for (_, node, _, _) in v {
                reconstruct_node_tree(node, cfg, state);
            }
        }
        VarSuffixList(span, v) | ElseIfThenVec(span, v) => {
            update_pos_range(span, state);

            for (_, node) in v {
                reconstruct_node_tree(node, cfg, state);
            }
        }
        StatementList(_, v) => {
            state.block_nested_level += 1;
            for (_, node) in v {
                reconstruct_node_tree(node, cfg, state);
            }
            state.block_nested_level -= 1;
        }
        FuncNameSelf(span, _, v, n) => {
            update_pos_range(span, state);

            for (_, node, _, _) in v {
                reconstruct_node_tree(node, cfg, state);
            }
            reconstruct_node_tree(&mut *n, cfg, state);
        }

        // custom
        Fields(_, v, opts) => {
            let mut is_iv_table = true;
            let has_single_child = v.len() == 1;

            for (_, node, _, _) in v {
                match node {
                    FieldSequential(_, e) => {
                        if let TableConstructor(_, _, _, nested_opts) = &mut **e {
                            nested_opts.is_single_child = Some(has_single_child);
                        }
                    }
                    _ => {
                        is_iv_table = false;
                    }
                }
                reconstruct_node_tree(node, cfg, state);
            }

            opts.is_iv_table = Some(is_iv_table);
        }
        TableConstructor(span, _, r, opts) => {
            update_pos_range(span, state);

            if opts.is_single_child.is_none() {
                opts.is_single_child = Some(true);
            }

            if let Fields(_, _, field_opts) = &mut **r {
                field_opts.is_single_child = opts.is_single_child;
            }

            reconstruct_node_tree(&mut *r, cfg, state);

            if let Fields(_, _, field_opts) = &**r {
                opts.is_iv_table = field_opts.is_iv_table;
            }
        }
        CharStringLiteral(span, s) => {
            update_pos_range(span, state);

            if cfg.fmt.convert_charstring_to_normalstring == Some(true) {
                *node = NormalStringLiteral(span.clone(), util::charstring_to_normalstring(&s));
            }
        }
    };
}
