use tera::{Context, Tera};

use crate::errors::MailRenderError;

pub fn render_template(template: &str, context: Context) -> Result<String, MailRenderError> {
    if template.trim().is_empty() {
        return Err(MailRenderError::EmptyTemplateError);
    }

    Tera::one_off(template, &context, false).map_err(MailRenderError::RenderTemplateError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tera::Context;

    #[test]
    fn test_render_template() {
        let template = "Hello, {{ name }} your age is {{ age }}!";
        let context = Context::from_value(json! {
            {
                "name": "world",
                "age": 30
            }
        })
        .unwrap();

        let result = render_template(template, context).unwrap();
        assert_eq!(result, "Hello, world your age is 30!");
    }

    #[test]
    fn test_render_empty_template() {
        let template = "";
        let context = Context::new();

        let result = render_template(template, context);
        assert!(result.is_err());
    }
}
