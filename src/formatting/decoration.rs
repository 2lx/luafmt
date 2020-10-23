use super::loc_hint::*;
use super::util::*;
use crate::config::*;
use std::fmt::Write;

pub struct NewLineDecor<LocHint>(pub LocHint, pub bool);
impl<'a, 'b, LocHint> ConfiguredWrite for NewLineDecor<LocHint>
where
    LocHint: ConfiguredWrite + LocHintConstructible<'a, 'b>,
{
    fn configured_write(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> std::fmt::Result {
        if !self.1 {
            return self.0.configured_write(f, cfg, buf, state);
        }

        // erase hint
        let loc_hint = LocHint::new(self.0.get_loc(), "");
        let mut comment_block = String::new();
        match loc_hint.configured_write(&mut comment_block, cfg, buf, state) {
            Ok(..) => {
                let trimmed = trim_end_spaces_and_tabs(&comment_block);
                if !trimmed.is_empty() {
                    write!(f, "{}", &trimmed)?;

                    if trimmed.chars().last() != Some('\n') {
                        write!(f, "\n")?;
                    }
                } else {
                    // if there was trailing spaces in the line, we need remove them.
                    // But if at this point we are in the CommentLocHint, we have a pseudo-string, and we cannot
                    // remove the first letter.
                    if f.len() > 1 && (f.chars().last() == Some(' ') || f.chars().last() == Some('\t')) {
                        f.pop();
                    }

                    // if the last lexem is one-line comment or shebang
                    if f.chars().last() != Some('\n') {
                        write!(f, "\n")?;
                    }
                }

                write_indent(f, cfg, state)?;
            }
            err @ Err(..) => return err,
        }
        Ok(())
    }
}

pub struct IndentIncDecor(pub Option<&'static str>);
impl ConfiguredWrite for IndentIncDecor {
    fn configured_write(&self, _: &mut String, _: &Config, _: &str, state: &mut State) -> std::fmt::Result {
        if self.0.is_none() || state.stack_indent.last() != Some(&self.0) {
            state.indent_level += 1;
        }
        state.stack_indent.push(self.0.clone());

        Ok(())
    }
}

pub struct IndentDecDecor();
impl ConfiguredWrite for IndentDecDecor {
    fn configured_write(&self, _: &mut String, _: &Config, _: &str, state: &mut State) -> std::fmt::Result {
        let last = state.stack_indent.pop();
        if last.is_some() && last.unwrap().is_none() || state.stack_indent.last() != last.as_ref() {
            state.indent_level -= 1;
        }

        Ok(())
    }
}

pub struct FuncLevelIncDecor();
impl ConfiguredWrite for FuncLevelIncDecor {
    fn configured_write(&self, _: &mut String, _: &Config, _: &str, state: &mut State) -> std::fmt::Result {
        state.function_nested_level += 1;
        Ok(())
    }
}

pub struct FuncLevelDecDecor();
impl ConfiguredWrite for FuncLevelDecDecor {
    fn configured_write(&self, _: &mut String, _: &Config, _: &str, state: &mut State) -> std::fmt::Result {
        state.function_nested_level -= 1;
        Ok(())
    }
}

#[test]
fn test_decors() -> std::fmt::Result {
    use crate::{cfg_write, cfg_write_helper};

    let cfg = Config::default();
    let mut state = State::default();
    let mut buf = String::new();

    // 1
    cfg_write!(&mut buf, &cfg, "", &mut state, IndentIncDecor(Some("1")))?;
    assert_eq!(state.indent_level, 1);
    assert_eq!(state.stack_indent, vec![Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, IndentDecDecor())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    // 2
    cfg_write!(&mut buf, &cfg, "", &mut state, IndentIncDecor(Some("1")), IndentIncDecor(Some("2")))?;
    assert_eq!(state.indent_level, 2);
    assert_eq!(state.stack_indent, vec![Some("1"), Some("2")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, IndentDecDecor(), IndentDecDecor())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    // 3
    cfg_write!(&mut buf, &cfg, "", &mut state, IndentIncDecor(Some("1")), IndentIncDecor(Some("1")))?;
    assert_eq!(state.indent_level, 1);
    assert_eq!(state.stack_indent, vec![Some("1"), Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, IndentDecDecor(), IndentDecDecor())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    // 4
    cfg_write!(
        &mut buf,
        &cfg,
        "",
        &mut state,
        IndentIncDecor(Some("1")),
        IndentIncDecor(None),
        IndentIncDecor(Some("1"))
    )?;
    assert_eq!(state.indent_level, 3);
    assert_eq!(state.stack_indent, vec![Some("1"), None, Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, IndentDecDecor(), IndentDecDecor(), IndentDecDecor())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    // 5
    cfg_write!(
        &mut buf,
        &cfg,
        "",
        &mut state,
        IndentIncDecor(None),
        IndentIncDecor(None),
        IndentIncDecor(Some("1")),
        IndentIncDecor(Some("1"))
    )?;
    assert_eq!(state.indent_level, 3);
    assert_eq!(state.stack_indent, vec![None, None, Some("1"), Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, IndentDecDecor())?;
    assert_eq!(state.indent_level, 3);
    assert_eq!(state.stack_indent, vec![None, None, Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, IndentDecDecor())?;
    assert_eq!(state.indent_level, 2);
    assert_eq!(state.stack_indent, vec![None, None]);

    cfg_write!(&mut buf, &cfg, "", &mut state, IndentDecDecor())?;
    assert_eq!(state.indent_level, 1);
    assert_eq!(state.stack_indent, vec![None]);

    cfg_write!(&mut buf, &cfg, "", &mut state, IndentDecDecor())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    Ok(())
}
