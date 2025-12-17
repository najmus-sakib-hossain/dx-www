#[test]
fn test_full_file_strip() {
    use dx_bundle_transform::strip_typescript_simple;
    use dx_bundle_transform::transform_jsx;

    let input = r#"export function add(a: number, b: number): number {
  return a + b;
}

export function multiply(x: number, y: number): number {
  return x * y;
}

export const PI: number = 3.14159;

export class Calculator {
  private history: number[] = [];

  compute(a: number, b: number): number {
    const result = add(multiply(a, b), PI);
    this.history.push(result);
    return result;
  }

  getHistory(): number[] {
    return this.history;
  }
}

export interface MathOp {
  execute(x: number, y: number): number;
}

export const operations: Record<string, MathOp> = {
  add: { execute: (x, y) => x + y },
  sub: { execute: (x, y) => x - y },
};"#;

    // First apply JSX transform
    let after_jsx = transform_jsx(input);
    println!("=== INPUT ===\n{}", input);
    println!("\n=== AFTER JSX ===\n{}", after_jsx);

    // Then apply TS strip
    let output = strip_typescript_simple(&after_jsx);
    println!("\n=== OUTPUT ===\n{}", output);

    // Check all TypeScript is removed
    assert!(!output.contains(": number"), "Should remove all : number type annotations");
    assert!(!output.contains(": number[]"), "Should remove array type annotations");
    assert!(!output.contains("private "), "Should remove private modifier");
    assert!(!output.contains("Record<"), "Should remove Record generic");
    assert!(output.contains("operations"), "Should keep operations variable");
    assert!(output.contains("execute"), "Should keep object content");
}
