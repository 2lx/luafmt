use super::loc_hint::*;
use super::util::*;
use crate::config::*;
use std::fmt::Write;

pub struct IfNewLine<LocHint>(pub bool, pub LocHint);
impl<'a, 'b, LocHint> ConfiguredWrite for IfNewLine<LocHint>
where
    LocHint: ConfiguredWrite + LocHintConstructible<'a, 'b>,
{
    fn configured_write(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> std::fmt::Result {
        if !self.0 {
            return self.1.configured_write(f, cfg, buf, state);
        }

        // erase hint
        let loc_hint = LocHint::new(self.1.get_loc(), "");
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

pub struct IncIndent(pub Option<&'static str>);
impl ConfiguredWrite for IncIndent {
    fn configured_write(&self, _: &mut String, _: &Config, _: &str, state: &mut State) -> std::fmt::Result {
        if self.0.is_none() || state.stack_indent.last() != Some(&self.0) {
            state.indent_level += 1;
        }
        state.stack_indent.push(self.0.clone());

        Ok(())
    }
}

pub struct DecIndent();
impl ConfiguredWrite for DecIndent {
    fn configured_write(&self, _: &mut String, _: &Config, _: &str, state: &mut State) -> std::fmt::Result {
        let last = state.stack_indent.pop();
        if last.is_some() && last.unwrap().is_none() || state.stack_indent.last() != last.as_ref() {
            state.indent_level -= 1;
        }

        Ok(())
    }
}

pub struct IncFuncLevel();
impl ConfiguredWrite for IncFuncLevel {
    fn configured_write(&self, _: &mut String, _: &Config, _: &str, state: &mut State) -> std::fmt::Result {
        state.function_nested_level += 1;
        Ok(())
    }
}

pub struct DecFuncLevel();
impl ConfiguredWrite for DecFuncLevel {
    fn configured_write(&self, _: &mut String, _: &Config, _: &str, state: &mut State) -> std::fmt::Result {
        state.function_nested_level -= 1;
        Ok(())
    }
}

pub struct If<'a>(pub bool, pub &'a dyn ConfiguredWrite);
impl ConfiguredWrite for If<'_> {
    fn configured_write(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> std::fmt::Result {
        match self.0 {
            true => self.1.configured_write(f, cfg, buf, state),
            _ => Ok(()),
        }
    }
}

#[test]
fn test_decors() -> std::fmt::Result {
    use crate::{cfg_write, cfg_write_helper};

    let cfg = Config::default();
    let mut state = State::default();
    let mut buf = String::new();

    // 1
    cfg_write!(&mut buf, &cfg, "", &mut state, IncIndent(Some("1")))?;
    assert_eq!(state.indent_level, 1);
    assert_eq!(state.stack_indent, vec![Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, DecIndent())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    // 2
    cfg_write!(&mut buf, &cfg, "", &mut state, IncIndent(Some("1")), IncIndent(Some("2")))?;
    assert_eq!(state.indent_level, 2);
    assert_eq!(state.stack_indent, vec![Some("1"), Some("2")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, DecIndent(), DecIndent())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    // 3
    cfg_write!(&mut buf, &cfg, "", &mut state, IncIndent(Some("1")), IncIndent(Some("1")))?;
    assert_eq!(state.indent_level, 1);
    assert_eq!(state.stack_indent, vec![Some("1"), Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, DecIndent(), DecIndent())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    // 4
    cfg_write!(&mut buf, &cfg, "", &mut state, IncIndent(Some("1")), IncIndent(None), IncIndent(Some("1")))?;
    assert_eq!(state.indent_level, 3);
    assert_eq!(state.stack_indent, vec![Some("1"), None, Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, DecIndent(), DecIndent(), DecIndent())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    // 5
    cfg_write!(
        &mut buf,
        &cfg,
        "",
        &mut state,
        IncIndent(None),
        IncIndent(None),
        IncIndent(Some("1")),
        IncIndent(Some("1"))
    )?;
    assert_eq!(state.indent_level, 3);
    assert_eq!(state.stack_indent, vec![None, None, Some("1"), Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, DecIndent())?;
    assert_eq!(state.indent_level, 3);
    assert_eq!(state.stack_indent, vec![None, None, Some("1")]);

    cfg_write!(&mut buf, &cfg, "", &mut state, DecIndent())?;
    assert_eq!(state.indent_level, 2);
    assert_eq!(state.stack_indent, vec![None, None]);

    cfg_write!(&mut buf, &cfg, "", &mut state, DecIndent())?;
    assert_eq!(state.indent_level, 1);
    assert_eq!(state.stack_indent, vec![None]);

    cfg_write!(&mut buf, &cfg, "", &mut state, DecIndent())?;
    assert_eq!(state.indent_level, 0);
    assert_eq!(state.stack_indent, vec![]);

    Ok(())
}
