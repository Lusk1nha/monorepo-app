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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_template() {
        let dir = tempdir().unwrap();
        let template_dir = dir.path();

        let template_name = "test_template.html";
        let template_path = template_dir.join(template_name);

        let mut file = File::create(&template_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let result = load_template(template_dir, template_name);
        assert_eq!(result.is_err(), false, "Failed to load template");

        dir.close().unwrap();
    }

    #[test]
    fn test_load_invalid_template() {
        let dir = tempdir().unwrap();
        let template_dir = dir.path();

        let template_name = "invalid_template.html";

        let result = load_template(template_dir, template_name);
        assert_eq!(result.is_err(), true);

        dir.close().unwrap();
    }
}
