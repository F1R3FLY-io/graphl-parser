use graph_to_rholang_parser::parse;

#[test]
fn test_parse() {
    let result = parse(c"{0}");
    assert_eq!(result)
}
