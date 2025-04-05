use roxmltree::Node;

/**
 * Gets a property from an XML node
 */
pub fn get_property(node: &Node, property: &str) -> Option<String> {
    if let Some(res) = node.children().find(|n| n.has_tag_name(property)) {
        if let Some(res2) = res.text() {
            return Some(res2.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use roxmltree::Document;

    use super::*;

    #[test]
    fn test_get_property() {
        let xml = r#"
			<root>
				<name>Test Name</name>
				<empty></empty>
				<missing />
			</root>
		"#;

        let doc = Document::parse(xml).unwrap();
        let root = doc.root_element();

        assert_eq!(get_property(&root, "name"), Some("Test Name".to_string()));

        assert_eq!(get_property(&root, "empty"), None);

        assert_eq!(get_property(&root, "nonexistent"), None);

        assert_eq!(get_property(&root, "missing"), None);
    }
}
