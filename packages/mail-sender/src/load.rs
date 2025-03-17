use std::fs;
use std::path::Path;

use crate::errors::MailLoadError;

pub fn load_template(template_dir: &Path, template_name: &str) -> Result<String, MailLoadError> {
    let template_path = template_dir.join(template_name);

    if !template_path.exists() {
        return Err(MailLoadError::LoadTemplateError(format!(
            "Template '{}' not found at path: {}",
            template_name,
            template_path.display()
        )));
    }

    fs::read_to_string(&template_path).map_err(|e| {
        MailLoadError::LoadTemplateError(format!(
            "Failed to load template '{}': {}",
            template_name, e
        ))
    })
}
