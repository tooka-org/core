use tooka_core::rule::Rule;
use tooka_core::template;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[test]
fn test_generated_template_is_valid() {
    // Generate the template YAML string
    let yaml = template::generate_rule_template_yaml()
        .expect("Failed to generate template YAML");

    // Write it to a temporary file (simulate real-world use)
    let path = Path::new("test_template.yaml");
    {
        let mut file = File::create(&path).expect("Failed to create temp YAML file");
        file.write_all(yaml.as_bytes()).expect("Failed to write YAML content");
    }

    // Deserialize it into a Rule struct
    let rule: Rule = serde_yaml::from_str(&yaml).expect("Failed to deserialize generated YAML");

    // Validate the rule
    rule.validate().expect("Validation failed for generated rule");

    // Cleanup: remove the temporary file
    fs::remove_file(path).expect("Failed to delete temp YAML file");
}
