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
