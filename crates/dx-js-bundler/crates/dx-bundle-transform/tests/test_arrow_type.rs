use dx_bundle_transform::strip_typescript_simple;

#[test]
fn test_arrow_function_param_type() {
    let input = "function useEffect(fn: () => void, deps: any[]) { fn(); }";
    let output = strip_typescript_simple(input);
    println!("Input:  {}", input);
    println!("Output: {}", output);

    assert!(!output.contains(": ()"));
    assert!(!output.contains("=> void"));
    assert!(output.contains("function useEffect(fn, deps)"));
}
