use handlebars::*;

#[derive(Clone, Copy)]
pub struct EqualsHelper;

impl HelperDef for EqualsHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'reg, 'rc>,
        handlebars: &'reg Handlebars<'_>,
        context: &'rc Context,
        render_context: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let a = helper
            .param(0)
            .map(|v| v.value().as_object().unwrap())
            .ok_or_else(|| RenderError::new("Parameter not found!"))?;

        let b = helper
            .param(1)
            .map(|v| v.value().as_object().unwrap())
            .ok_or_else(|| RenderError::new("Parameter not found!"))?;

        let template = if a == b {
            helper.template()
        } else {
            helper.inverse()
        };

        match template {
            Some(ref t) => t.render(handlebars, context, render_context, out),
            None => Ok(()),
        }
    }
}
