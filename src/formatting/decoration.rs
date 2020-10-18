use super::util::*;
use super::loc_hint::*;
use crate::config::*;
use std::fmt;

pub struct NewLineDecor<LocHint>(pub LocHint, pub bool);
impl<'a, 'b, LocHint> ConfiguredWrite for NewLineDecor<LocHint>
where
    LocHint: ConfiguredWrite + LocHintConstructible<'a, 'b>,
{
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str, state: &mut State) -> fmt::Result {
        if !self.1 {
            return self.0.configured_write(f, cfg, buf, state)
        }

        // erase hint
        let loc_hint = LocHint::new(self.0.get_loc(), "");
        let mut comment_block = String::new();
        match loc_hint.configured_write(&mut comment_block, cfg, buf, state) {
            Ok(..) => {
                let trimmed = trim_end_spaces_and_tabs(&comment_block);
                write!(f, "{}", &trimmed)?;

                if trimmed.chars().last() != Some('\n') {
                    write!(f, "\n")?;
                }

                write_indent(f, cfg, state)?;
            }
            err @ Err(..) => return err,
        }
        Ok(())
    }
}

pub struct IndentDecor(pub isize);
impl ConfiguredWrite for IndentDecor {
    fn configured_write(&self, _: &mut dyn fmt::Write, _: &Config, _: &str, state: &mut State) -> fmt::Result {
        state.indent_level += self.0;

        Ok(())
    }
}
