#[derive(Debug)]
pub struct LinkReferenceDefinition {
    label: String,
    destination: String,
    title: Option<String>,
}

impl LinkReferenceDefinition {
    pub fn new(label: String, destination: String, title: Option<String>) -> Self {
        LinkReferenceDefinition {
            label,
            destination,
            title,
        }
    }

    fn label_matches(&self, target_label: &str) -> bool {
        self.label == target_label
    }
}
// If there are several matching definitions, the first one takes precedence:
// matching of labels is case-insensitive
pub fn has_matching_label(
    link_definitions: &[LinkReferenceDefinition],
    target_label: &str,
) -> bool {
    for definition in link_definitions {
        if definition.label_matches(target_label) {
            return true;
        }
    }
    false
}

pub fn is_link_reference_definition(text: &str) -> bool {
    false
}
