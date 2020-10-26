use crate::config::Config;
use crate::parser::lua_ast::Node;
use crate::formatting::util;

pub fn reconstruct_node_tree(node: &mut Node, cfg: &Config) {
    use Node::*;
    match node {
        UnaryOp(_, _, _, n)
        | UnaryNot(_, _, n)
        | RoundBrackets(_, _, n)
        | ArgsRoundBrackets(_, _, n)
        | FieldSequential(_, n)
        | TableIndex(_, _, n)
        | TableMember(_, _, n)
        | FunctionDef(_, _, n)
        | FuncBodyB(_, _, n)
        | FuncPBody(_, _, n)
        | LocalNames(_, _, n)
        | IfThen(_, _, n)
        | IfThenElse(_, _, n)
        | ElseIfThen(_, _, n)
        | Label(_, _, n)
        | GoTo(_, _, n)
        | WhileDo(_, _, n)
        | RepeatUntil(_, _, n)
        | RetStatExpr(_, _, n)
        | RetStatExprComma(_, _, n)
        | Chunk(_, n, _)
        | DoBEnd(_, _, n) => {
            reconstruct_node_tree(&mut *n, cfg);
        }

        BinaryOp(_, _, _, n1, n2)
        | Var(_, _, n1, n2)
        | FieldNamedBracket(_, _, n1, n2)
        | FieldNamed(_, _, n1, n2)
        | VarRoundSuffix(_, _, n1, n2)
        | FnMethodCall(_, _, n1, n2)
        | FuncPBodyB(_, _, n1, n2)
        | FuncDecl(_, _, n1, n2)
        | LocalFuncDecl(_, _, n1, n2)
        | LocalNamesExprs(_, _, n1, n2)
        | IfThenB(_, _, n1, n2)
        | IfThenBElse(_, _, n1, n2)
        | IfThenElseB(_, _, n1, n2)
        | IfThenElseIf(_, _, n1, n2)
        | IfThenElseIfElse(_, _, n1, n2)
        | ElseIfThenB(_, _, n1, n2)
        | WhileDoB(_, _, n1, n2)
        | RepeatBUntil(_, _, n1, n2)
        | ForRange(_, _, n1, n2)
        | StatsRetStat(_, _, n1, n2)
        | SheBangChunk(_, n1, _, n2, _)
        | VarsExprs(_, _, n1, n2) => {
            reconstruct_node_tree(&mut *n1, cfg);
            reconstruct_node_tree(&mut *n2, cfg);
        }

        IfThenBElseIf(_, _, n1, n2, n3)
        | IfThenBElseIfElse(_, _, n1, n2, n3)
        | IfThenElseIfElseB(_, _, n1, n2, n3)
        | ForInt(_, _, n1, n2, n3)
        | ForRangeB(_, _, n1, n2, n3)
        | IfThenBElseB(_, _, n1, n2, n3) => {
            reconstruct_node_tree(&mut *n1, cfg);
            reconstruct_node_tree(&mut *n2, cfg);
            reconstruct_node_tree(&mut *n3, cfg);
        }

        ForIntB(_, _, n1, n2, n3, n4)
        | ForIntStep(_, _, n1, n2, n3, n4)
        | IfThenBElseIfElseB(_, _, n1, n2, n3, n4) => {
            reconstruct_node_tree(&mut *n1, cfg);
            reconstruct_node_tree(&mut *n2, cfg);
            reconstruct_node_tree(&mut *n3, cfg);
            reconstruct_node_tree(&mut *n4, cfg);
        }

        ForIntStepB(_, _, n1, n2, n3, n4, n5) => {
            reconstruct_node_tree(&mut *n1, cfg);
            reconstruct_node_tree(&mut *n2, cfg);
            reconstruct_node_tree(&mut *n3, cfg);
            reconstruct_node_tree(&mut *n4, cfg);
            reconstruct_node_tree(&mut *n5, cfg);
        }

        ArgsRoundBracketsEmpty(..)
        | Nil(..)
        | False(..)
        | True(..)
        | VarArg(..)
        | Break(..)
        | Numeral(..)
        | NormalStringLiteral(..)
        | MultiLineStringLiteral(..)
        | TableConstructorEmpty(..)
        | DoEnd(..)
        | Name(..)
        | RetStatNone(..)
        | RetStatNoneComma(..)
        | Semicolon(..)
        | SheBang(..)
        | FuncBody(..) => {}

        ExpList(_, v) | NameList(_, v) | VarList(_, v) | ParList(_, v) | FuncName(_, v) => {
            for (_, node, _, _) in v {
                reconstruct_node_tree(node, cfg);
            }
        }
        StatementList(_, v) | VarSuffixList(_, v) | ElseIfThenVec(_, v) => {
            for (_, node) in v {
                reconstruct_node_tree(node, cfg);
            }
        }
        FuncNameSelf(_, _, v, n) => {
            for (_, node, _, _) in v {
                reconstruct_node_tree(node, cfg);
            }
            reconstruct_node_tree(&mut *n, cfg);
        }

        // custom
        Fields(_, v, opts) => {
            let mut is_all_sequential = true;
            let has_single_child = v.len() == 1;

            for (_, node, _, _) in v {
                match node {
                    FieldSequential(_, e) => {
                        if let TableConstructor(_, _, _, nested_opts) = &mut **e {
                            nested_opts.is_single_child = Some(has_single_child);
                            nested_opts.children_of_single_child = opts.is_single_child;
                        }
                    }
                    _ => { is_all_sequential = false; }
                }
                reconstruct_node_tree(node, cfg);
            }

            opts.is_all_sequential = Some(is_all_sequential);
            opts.has_single_child = Some(has_single_child);
        }
        TableConstructor(_, _, r, opts) => {
            if opts.children_of_single_child.is_none() {
                opts.children_of_single_child = Some(true);
            }
            if opts.is_single_child.is_none() {
                opts.is_single_child = Some(true);
            }

            if let Fields(_, _, field_opts) = &mut **r {
                field_opts.is_single_child = opts.is_single_child;
                field_opts.children_of_single_child = opts.children_of_single_child;
            }

            reconstruct_node_tree(&mut *r, cfg);

            if let Fields(_, _, field_opts) = &**r {
                opts.is_all_sequential = field_opts.is_all_sequential;
                opts.has_single_child = field_opts.has_single_child;
            }
        }
        CharStringLiteral(pos, s) => {
            if cfg.convert_charstring_to_normalstring == Some(true) {
                *node = NormalStringLiteral(pos.clone(), util::charstring_to_normalstring(&s));
            }
        }
    };
}
