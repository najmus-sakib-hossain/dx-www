use dx_bundle_transform::strip_typescript_simple;

#[test]
fn test_generic_function() {
    let input = r#"export function useState<T>(initial: T): [T, (value: T) => void] {
  let state = initial;
  return [state, setState];
}"#;

    let output = strip_typescript_simple(input);
    println!("Input:\n{}", input);
    println!("\nOutput:\n{}", output);

    // Should have function with params and body
    assert!(output.contains("function useState"));
    assert!(output.contains("(initial)"));
    assert!(output.contains("let state"));
    assert!(!output.contains("<T>"));
    assert!(!output.contains(": T"));
    assert!(!output.contains(": [T"));
}

#[test]
fn test_object_destructuring_type() {
    let input = "function Component({ count, onClick }: ComponentProps) { return null; }";
    let output = strip_typescript_simple(input);
    println!("Input: {}", input);
    println!("Output: {}", output);

    assert!(output.contains("({ count, onClick })"));
    assert!(!output.contains(": ComponentProps"));
}
