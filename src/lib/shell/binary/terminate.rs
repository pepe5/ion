use crate::{
    parser::Terminator,
    shell::{status::*, FlowLogic, Shell},
};

pub(crate) fn terminate_script_quotes<I: Iterator<Item = u8>>(shell: &mut Shell, lines: I) -> i32 {
    let mut lines = lines.peekable();
    while lines.peek().is_some() {
        match Terminator::new(&mut lines).terminate() {
            Ok(stmt) => shell.on_command(&stmt),
            Err(_) => {
                eprintln!("ion: unterminated quote in script");
                return FAILURE;
            }
        }
    }

    if shell.flow_control.unclosed_block() {
        let open_block = shell.flow_control.block.last().unwrap();
        eprintln!("ion: unexpected end of script: expected end block for `{}`", open_block.short(),);
        FAILURE
    } else {
        SUCCESS
    }
}
