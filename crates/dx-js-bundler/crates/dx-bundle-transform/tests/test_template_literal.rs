use dx_bundle_transform::strip_typescript_simple;

#[test]
fn test_template_literal() {
    let input = r#"const helpers = {
  format: (value: number): string => `Count: ${value}`,
  parse: (str: string): number => parseInt(str, 10),
};"#;
    let output = strip_typescript_simple(input);
    println!("Input:\n{}", input);
    println!("\nOutput:\n{}", output);

    assert!(output.contains("`Count:"));
    assert!(output.contains("${value}"));
    assert!(!output.contains(": number"));
    assert!(!output.contains(": string"));
}
