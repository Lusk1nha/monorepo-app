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
