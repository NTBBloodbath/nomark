#[test]
fn no_frontmatter() {
    let result = nomark::convert("# Hello").unwrap();
    assert_eq!(result, "* Hello");
}

#[test]
fn simple_frontmatter() {
    let md = "---\ntitle: Test\n---\n\n# Hello";
    let result = nomark::convert(md).unwrap();
    assert_eq!(result, "@document.meta\ntitle: Test\n@end\n\n* Hello");
}

#[test]
fn frontmatter_multiple_fields() {
    let md = "---\ntitle: Test\nauthor: Me\n---\n\nContent";
    let result = nomark::convert(md).unwrap();
    assert!(result.starts_with("@document.meta"));
    assert!(result.contains("title: Test"));
    assert!(result.contains("author: Me"));
    assert!(result.contains("@end"));
    assert!(result.contains("Content"));
}

#[test]
fn frontmatter_empty() {
    let result = nomark::convert("---\n---\n\n# Hello").unwrap();
    assert_eq!(result, "___\n___\n* Hello");
}

#[test]
fn frontmatter_preserves_yaml_structure() {
    let md = "---\nnested:\n  key: value\nlist:\n  - one\n  - two\n---\n\nBody";
    let result = nomark::convert(md).unwrap();
    assert!(result.starts_with("@document.meta"));
    assert!(result.contains("nested:"));
    assert!(result.contains("  key: value"));
    assert!(result.contains("list:"));
    assert!(result.contains("  - one"));
    assert!(result.contains("  - two"));
    assert!(result.contains("@end"));
}
