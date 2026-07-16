#[test]
fn unordered_list() {
    let result = nomark::convert("- a\n- b").unwrap();
    assert_eq!(result, "- a\n- b");
}

#[test]
fn ordered_list() {
    let result = nomark::convert("1. a\n2. b").unwrap();
    assert_eq!(result, "~ a\n~ b");
}

#[test]
fn nested_unordered() {
    let result = nomark::convert("- a\n  - b").unwrap();
    assert_eq!(result, "- a\n-- b");
}

#[test]
fn nested_ordered() {
    let result = nomark::convert("1. a\n   1. b").unwrap();
    assert_eq!(result, "~ a\n~~ b");
}

#[test]
fn deep_nesting() {
    let result = nomark::convert("- a\n  - b\n    - c").unwrap();
    assert_eq!(result, "- a\n-- b\n--- c");
}

#[test]
fn task_list_done() {
    let result = nomark::convert("- [x] done").unwrap();
    assert_eq!(result, "- (x) done");
}

#[test]
fn task_list_todo() {
    let result = nomark::convert("- [ ] todo").unwrap();
    assert_eq!(result, "- ( ) todo");
}

#[test]
fn task_list_mixed() {
    let result = nomark::convert("- [x] done\n- [ ] todo").unwrap();
    assert_eq!(result, "- (x) done\n- ( ) todo");
}

#[test]
fn mixed_list_types() {
    let result = nomark::convert("1. a\n2. b\n\n- c\n- d").unwrap();
    assert_eq!(result, "~ a\n~ b\n\n- c\n- d");
}
