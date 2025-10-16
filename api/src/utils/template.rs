use std::fs;
use std::collections::HashMap;

pub fn load_template(template_name: &str, variables: HashMap<&str, &str>) -> Result<String, Box<dyn std::error::Error>> {
    let template_path = format!("templates/{}.html", template_name);
    let mut content = fs::read_to_string(template_path)?;
    
    for (key, value) in variables {
        let placeholder = format!("{{{{{}}}}}", key);
        content = content.replace(&placeholder, value);
    }
    
    Ok(content)
}