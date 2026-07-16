#[test]
fn backslash_escaped() {
    let result = nomark::convert("a\\b").unwrap();
    assert_eq!(result, "a\\\\b");
}

#[test]
fn curly_braces_escaped() {
    let result = nomark::convert("a{b}c").unwrap();
    assert_eq!(result, "a\\{b\\}c");
}

#[test]
fn square_brackets_escaped() {
    let result = nomark::convert("a[b]c").unwrap();
    assert_eq!(result, "a\\[b\\]c");
}

#[test]
fn empty_input() {
    let result = nomark::convert("").unwrap();
    assert_eq!(result, "");
}

#[test]
fn only_whitespace() {
    let result = nomark::convert("   ").unwrap();
    assert_eq!(result, "");
}

#[test]
fn code_inline_not_escaped() {
    let result = nomark::convert("`{hi}`").unwrap();
    assert_eq!(result, "`{hi}`");
}

#[test]
fn code_block_not_escaped() {
    let result = nomark::convert("```\n{a}\n```").unwrap();
    assert_eq!(result, "@code \n{a}\n@end");
}

#[test]
fn html_is_stripped() {
    let result = nomark::convert("text <tag>more").unwrap();
    assert_eq!(result, "text more");
}

#[test]
fn bold_with_braces_in_text() {
    let result = nomark::convert("**b** and {braces}").unwrap();
    assert_eq!(result, "*b* and \\{braces\\}");
}
