use crate::ast::Ctx;

pub fn make_error_msg(ctx: &Ctx, error_reason: String) -> String {
    format!(
        "File \"<{}>\", line {}, in <{}>\n    {}",
        ctx.file, ctx.line, ctx.module, error_reason
    )
}
