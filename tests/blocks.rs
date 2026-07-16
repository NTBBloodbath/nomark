#[test]
fn heading_h1() {
    let result = nomark::convert("# H1").unwrap();
    assert_eq!(result, "* H1");
}

#[test]
fn heading_h2() {
    let result = nomark::convert("## H2").unwrap();
    assert_eq!(result, "** H2");
}

#[test]
fn heading_h3() {
    let result = nomark::convert("### H3").unwrap();
    assert_eq!(result, "*** H3");
}

#[test]
fn heading_h6() {
    let result = nomark::convert("###### H6").unwrap();
    assert_eq!(result, "****** H6");
}

#[test]
fn multiple_headings() {
    let result = nomark::convert("# H1\n\n## H2").unwrap();
    assert_eq!(result, "* H1\n\n** H2");
}

#[test]
fn code_block_no_lang() {
    let result = nomark::convert("```\nfn main() {}\n```").unwrap();
    assert_eq!(result, "@code \nfn main() {}\n@end");
}

#[test]
fn code_block_with_lang() {
    let result = nomark::convert("```rust\nfn main() {}\n```").unwrap();
    assert_eq!(result, "@code rust\nfn main() {}\n@end");
}

#[test]
fn code_block_multiline() {
    let result = nomark::convert("```\nline1\nline2\nline3\n```").unwrap();
    assert_eq!(result, "@code \nline1\nline2\nline3\n@end");
}

#[test]
fn blockquote_simple() {
    let result = nomark::convert("> quote").unwrap();
    assert_eq!(result, "> quote");
}

#[test]
fn blockquote_multiline() {
    let result = nomark::convert("> line 1\n> line 2").unwrap();
    assert_eq!(result, "> line 1 line 2");
}

#[test]
fn blockquote_multiple_paragraphs() {
    let result = nomark::convert("> para 1\n>\n> para 2").unwrap();
    assert_eq!(result, "> para 1\n\n> para 2");
}

#[test]
fn horizontal_rule() {
    let result = nomark::convert("---").unwrap();
    assert_eq!(result, "___");
}

#[test]
fn multiple_horizontal_rules() {
    let result = nomark::convert("---\n\n---").unwrap();
    assert_eq!(result, "___\n___");
}

#[test]
fn paragraph_separation() {
    let result = nomark::convert("para one\n\npara two").unwrap();
    assert_eq!(result, "para one\n\npara two");
}

#[test]
fn heading_with_paragraph() {
    let result = nomark::convert("# Title\n\nContent here.").unwrap();
    assert_eq!(result, "* Title\n\nContent here.");
}
