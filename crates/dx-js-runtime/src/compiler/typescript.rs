//! TypeScript Type System
//!
//! This module handles TypeScript type annotations, type checking,
//! and type inference to generate optimized code.

use crate::compiler::mir::{Type, PrimitiveType, TypeId};
use crate::error::DxResult;
use oxc_ast::ast::TSType;
use std::collections::HashMap;

/// TypeScript type analyzer
pub struct TypeScriptAnalyzer {
    /// Type aliases (type MyType = ...)
    type_aliases: HashMap<String, Type>,
    /// Interface definitions
    interfaces: HashMap<String, InterfaceType>,
    /// Generic type parameters in scope
    generic_params: Vec<String>,
}

/// Interface type definition
#[derive(Debug, Clone)]
pub struct InterfaceType {
    pub name: String,
    pub properties: Vec<PropertySignature>,
    pub methods: Vec<MethodSignature>,
    pub extends: Vec<String>,
}

/// Property signature in interface
#[derive(Debug, Clone)]
pub struct PropertySignature {
    pub name: String,
    pub ty: Type,
    pub optional: bool,
    pub readonly: bool,
}

/// Method signature in interface
#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub optional: bool,
}

impl TypeScriptAnalyzer {
    /// Create new TypeScript analyzer
    pub fn new() -> Self {
        Self {
            type_aliases: HashMap::new(),
            interfaces: HashMap::new(),
            generic_params: Vec::new(),
        }
    }

    /// Convert TypeScript type annotation to MIR Type
    pub fn convert_ts_type(&self, ts_type: &TSType) -> DxResult<Type> {
        match ts_type {
            TSType::TSAnyKeyword(_) => Ok(Type::Any),
            TSType::TSBooleanKeyword(_) => Ok(Type::Primitive(PrimitiveType::Bool)),
            TSType::TSNumberKeyword(_) => Ok(Type::Primitive(PrimitiveType::F64)),
            TSType::TSStringKeyword(_) => Ok(Type::Primitive(PrimitiveType::String)),
            TSType::TSNullKeyword(_) => Ok(Type::Primitive(PrimitiveType::Null)),
            TSType::TSUndefinedKeyword(_) => Ok(Type::Primitive(PrimitiveType::Undefined)),
            TSType::TSVoidKeyword(_) => Ok(Type::Primitive(PrimitiveType::Undefined)),
            
            TSType::TSArrayType(array) => {
                let element_type = self.convert_ts_type(&array.element_type)?;
                Ok(Type::Array(Box::new(element_type)))
            }
            
            TSType::TSTupleType(_tuple) => {
                // Tuples not yet supported in MIR, use Array
                Ok(Type::Array(Box::new(Type::Any)))
            }
            
            TSType::TSUnionType(_union) => {
                // Unions not yet supported in MIR, use Any
                Ok(Type::Any)
            }
            
            TSType::TSTypeLiteral(_literal) => {
                // Object type with specific properties
                // For now, use TypeId(0) as placeholder
                Ok(Type::Object(TypeId(0)))
            }
            
            TSType::TSTypeReference(type_ref) => {
                // Look up type alias or interface
                let type_name = type_ref.type_name.to_string();
                
                // Check if it's a built-in generic type
                match type_name.as_str() {
                    "Array" => {
                        if let Some(type_params) = &type_ref.type_parameters {
                            if let Some(first_param) = type_params.params.first() {
                                let element_type = self.convert_ts_type(first_param)?;
                                return Ok(Type::Array(Box::new(element_type)));
                            }
                        }
                        Ok(Type::Array(Box::new(Type::Any)))
                    }
                    "Promise" => {
                        // Promise not yet in MIR Type, use Any
                        Ok(Type::Any)
                    }
                    _ => {
                        // Check type aliases
                        if let Some(alias_type) = self.type_aliases.get(&type_name) {
                            Ok(alias_type.clone())
                        } else {
                            // Unknown type, default to Any
                            Ok(Type::Any)
                        }
                    }
                }
            }
            
            TSType::TSFunctionType(_func) => {
                // For now, use placeholder FunctionSignature
                // TODO: properly parse params and return type
                use crate::compiler::mir::FunctionSignature;
                Ok(Type::Function(FunctionSignature {
                    params: vec![],
                    return_type: Box::new(Type::Any),
                }))
            }
            
            _ => {
                // Unsupported TypeScript type, default to Any
                Ok(Type::Any)
            }
        }
    }

    /// Register type alias
    pub fn register_type_alias(&mut self, name: String, ty: Type) {
        self.type_aliases.insert(name, ty);
    }

    /// Register interface
    pub fn register_interface(&mut self, interface: InterfaceType) {
        self.interfaces.insert(interface.name.clone(), interface);
    }

    /// Infer type from expression
    pub fn infer_type(&self, expr: &oxc_ast::ast::Expression) -> Type {
        use oxc_ast::ast::Expression;
        
        match expr {
            Expression::BooleanLiteral(_) => Type::Primitive(PrimitiveType::Bool),
            Expression::NumericLiteral(_) => Type::Primitive(PrimitiveType::F64),
            Expression::StringLiteral(_) => Type::Primitive(PrimitiveType::String),
            Expression::NullLiteral(_) => Type::Primitive(PrimitiveType::Null),
            Expression::Identifier(_) => Type::Any, // Would need symbol table
            Expression::ArrayExpression(_) => Type::Array(Box::new(Type::Any)),
            Expression::ObjectExpression(_) => Type::Object(TypeId(0)),
            Expression::ArrowFunctionExpression(_) => {
                use crate::compiler::mir::FunctionSignature;
                Type::Function(FunctionSignature {
                    params: vec![],
                    return_type: Box::new(Type::Any),
                })
            },
            Expression::FunctionExpression(_) => {
                use crate::compiler::mir::FunctionSignature;
                Type::Function(FunctionSignature {
                    params: vec![],
                    return_type: Box::new(Type::Any),
                })
            },
            _ => Type::Any,
        }
    }

    /// Type check: Verify that value_type is assignable to target_type
    pub fn is_assignable(&self, value_type: &Type, target_type: &Type) -> bool {
        match (value_type, target_type) {
            // Any is assignable to and from anything
            (Type::Any, _) | (_, Type::Any) => true,
            
            // Exact type match
            (Type::Primitive(a), Type::Primitive(b)) => a == b,
            
            // Array covariance
            (Type::Array(a), Type::Array(b)) => self.is_assignable(a, b),
            
            // Union type handling - not yet supported in MIR
            // (removed for now)
            
            // Object types
            (Type::Object(_), Type::Object(_)) => true,
            
            _ => false,
        }
    }

    /// Generate optimized code based on type information
    pub fn optimize_for_type(&self, ty: &Type) -> OptimizationHint {
        match ty {
            Type::Primitive(PrimitiveType::I32) => OptimizationHint::UseI32,
            Type::Primitive(PrimitiveType::F64) => OptimizationHint::UseF64,
            Type::Primitive(PrimitiveType::Bool) => OptimizationHint::UseBool,
            Type::Array(_) => OptimizationHint::UseTypedArray,
            Type::Function { .. } => OptimizationHint::Monomorphize,
            _ => OptimizationHint::None,
        }
    }
}

/// Optimization hints derived from type information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationHint {
    None,
    UseI32,
    UseF64,
    UseBool,
    UseTypedArray,
    Monomorphize,
}

impl Default for TypeScriptAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_type_checking() {
        let analyzer = TypeScriptAnalyzer::new();
        
        let i32_type = Type::Primitive(PrimitiveType::I32);
        let f64_type = Type::Primitive(PrimitiveType::F64);
        let bool_type = Type::Primitive(PrimitiveType::Bool);
        
        assert!(analyzer.is_assignable(&i32_type, &f64_type));
        assert!(analyzer.is_assignable(&f64_type, &i32_type));
        assert!(!analyzer.is_assignable(&bool_type, &i32_type));
    }

    #[test]
    fn test_any_type() {
        let analyzer = TypeScriptAnalyzer::new();
        
        let any_type = Type::Any;
        let i32_type = Type::Primitive(PrimitiveType::I32);
        
        assert!(analyzer.is_assignable(&any_type, &i32_type));
        assert!(analyzer.is_assignable(&i32_type, &any_type));
    }
}
