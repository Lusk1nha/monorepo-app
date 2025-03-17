use tera::{Context, Tera};

use crate::errors::MailRenderError;

pub fn render_template(template: &str, context: Context) -> Result<String, MailRenderError> {
    if template.trim().is_empty() {
        return Err(MailRenderError::EmptyTemplateError);
    }

    Tera::one_off(template, &context, false).map_err(MailRenderError::RenderTemplateError)
}
