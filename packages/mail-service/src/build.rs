use std::path::PathBuf;

use tera::Context;

use crate::{errors::MailBuilderError, load::load_template, render::render_template};

#[derive(Clone)]
pub struct EmailBuilder {
    template_dir: PathBuf,
}

impl EmailBuilder {
    pub fn new(template_dir: PathBuf) -> Self {
        Self { template_dir }
    }

    pub fn build_template(
        &self,
        template: &str,
        context: Option<Context>,
    ) -> Result<String, MailBuilderError> {
        let template = load_template(&self.template_dir, template)?;
        let render = render_template(&template, context.unwrap_or_default())?;
        Ok(render)
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_build_template() {
        let template_dir = PathBuf::from("tests/templates");
        let builder = EmailBuilder::new(template_dir);

        let template = "valid_template.html";
        let context = Context::from_value(serde_json::json!({
            "name": "world",
            "age": 30
        }))
        .unwrap();

        let result = builder.build_template(template, Some(context));
        assert_eq!(result.is_err(), false);
    }

    #[test]
    fn test_build_invalid_template() {
        let template_dir = PathBuf::from("tests/templates");
        let builder = EmailBuilder::new(template_dir);

        let template = "invalid_template.html";
        let context = Context::new();

        let result = builder.build_template(template, Some(context));
        assert_eq!(result.is_err(), true);
    }
}
