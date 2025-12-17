#[test]
fn test_variable_type_annotation() {
    use dx_bundle_transform::strip_typescript_simple;

    // Test export const with type
    let input = "export const PI: number = 3.14159;";
    let output = strip_typescript_simple(input);
    println!("Input 1: {}", input);
    println!("Output 1: {}", output);

    // Test private class field
    let input2 = "private history: number[] = [];";
    let output2 = strip_typescript_simple(input2);
    println!("Input 2: {}", input2);
    println!("Output 2: {}", output2);

    // Test exports.name: Type pattern
    let input3 = "exports.PI: number = 3.14159;";
    let output3 = strip_typescript_simple(input3);
    println!("Input 3: {}", input3);
    println!("Output 3: {}", output3);

    // Now assert
    assert!(!output.contains(": number"), "Should remove type annotation from const");
    assert!(output.contains("PI"), "Should keep variable name");
    assert!(output.contains("3.14159"), "Should keep value");

    assert!(!output2.contains("private"), "Should remove private modifier");
    assert!(!output2.contains(": number[]"), "Should remove type annotation");
    assert!(output2.contains("history"), "Should keep field name");

    assert!(!output3.contains(": number"), "Should remove type from exports");
    assert!(output3.contains("exports.PI"), "Should keep exports.name");
}
