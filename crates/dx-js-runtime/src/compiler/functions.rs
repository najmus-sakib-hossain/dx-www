//! Function and Class Compilation
//!
//! Handles:
//! - Function declarations
//! - Function expressions
//! - Arrow functions
//! - Closures and captured variables
//! - Class declarations
//! - Constructors, methods, properties
//! - Inheritance and super calls
//! - Private fields

use crate::compiler::mir::*;
use crate::compiler::statements::StatementLowerer;
use crate::error::DxResult;
use oxc_ast::ast::*;
use std::collections::HashMap;

/// Function compilation context
pub struct FunctionCompiler {
    /// Next function ID
    next_id: u32,
    /// Compiled functions
    functions: Vec<TypedFunction>,
    /// Closure capture information
    captures: HashMap<String, LocalId>,
}

impl FunctionCompiler {
    pub fn new() -> Self {
        Self {
            next_id: 1, // 0 is reserved for main
            functions: Vec::new(),
            captures: HashMap::new(),
        }
    }

    /// Compile a function declaration
    pub fn compile_function_decl(&mut self, func: &Function) -> DxResult<FunctionId> {
        let func_id = FunctionId(self.next_id);
        self.next_id += 1;

        let name = func
            .id
            .as_ref()
            .map(|id| id.name.to_string())
            .unwrap_or_else(|| format!("__anon_{}", func_id.0));

        let mut builder = FunctionBuilder::new(func_id, name);

        // Add parameters
        for (i, param) in func.params.items.iter().enumerate() {
            let param_name = self.extract_param_name(param, i);
            builder.add_param(param_name, Type::Any);
        }

        // Set return type (for now, always f64)
        builder.return_type = Type::Primitive(PrimitiveType::F64);

        // Compile function body
        if let Some(body) = &func.body {
            let mut lowerer = StatementLowerer::new(builder);
            for stmt in &body.statements {
                lowerer.lower_statement(stmt)?;
            }
            builder = lowerer.finish();
        }

        let typed_func = builder.build();
        self.functions.push(typed_func);

        Ok(func_id)
    }

    /// Compile an arrow function
    pub fn compile_arrow_function(
        &mut self,
        arrow: &ArrowFunctionExpression,
    ) -> DxResult<FunctionId> {
        let func_id = FunctionId(self.next_id);
        self.next_id += 1;

        let name = format!("__arrow_{}", func_id.0);
        let mut builder = FunctionBuilder::new(func_id, name);

        // Add parameters
        for (i, param) in arrow.params.items.iter().enumerate() {
            let param_name = self.extract_param_name(param, i);
            builder.add_param(param_name, Type::Any);
        }

        // Compile body
        if arrow.expression {
            // Expression body: () => expr
            // We need to return the expression value
            // For now, just create empty function
        } else {
            // Statement body: () => { ... }
            // arrow.body is a FunctionBody, not Statement
            let body = &arrow.body;
            let mut lowerer = StatementLowerer::new(builder);
            for stmt in &body.statements {
                lowerer.lower_statement(stmt)?;
            }
            builder = lowerer.finish();
        }

        let typed_func = builder.build();
        self.functions.push(typed_func);

        Ok(func_id)
    }

    /// Extract parameter name from binding pattern
    fn extract_param_name(&self, param: &FormalParameter, index: usize) -> String {
        match &param.pattern.kind {
            BindingPatternKind::BindingIdentifier(ident) => ident.name.to_string(),
            _ => format!("__param_{}", index),
        }
    }

    /// Get all compiled functions
    pub fn get_functions(self) -> Vec<TypedFunction> {
        self.functions
    }
}

/// Class compilation context
pub struct ClassCompiler {
    /// Next class ID
    next_id: u32,
    /// Compiled classes (as type layouts)
    classes: Vec<TypeLayout>,
    /// Class methods (as functions)
    methods: Vec<TypedFunction>,
    /// Next function ID for methods
    next_func_id: u32,
}

impl ClassCompiler {
    pub fn new(starting_func_id: u32) -> Self {
        Self {
            next_id: 0,
            classes: Vec::new(),
            methods: Vec::new(),
            next_func_id: starting_func_id,
        }
    }

    /// Compile a class declaration
    pub fn compile_class(&mut self, class: &Class) -> DxResult<TypeId> {
        let type_id = TypeId(self.next_id);
        self.next_id += 1;

        let class_name = class
            .id
            .as_ref()
            .map(|id| id.name.to_string())
            .unwrap_or_else(|| format!("__class_{}", type_id.0));

        let mut fields = Vec::new();
        let mut offset = 0u32;

        // Process class elements
        for element in &class.body.body {
            match element {
                ClassElement::PropertyDefinition(prop) => {
                    let field_name = self.extract_property_name(&prop.key);
                    fields.push(FieldLayout {
                        name: field_name,
                        offset,
                        ty: Type::Any,
                    });
                    offset += 8; // 8 bytes per field (pointer size)
                }
                ClassElement::MethodDefinition(method) => {
                    // Compile method as a function
                    let method_name = format!("{}::{}", class_name, self.extract_property_name(&method.key));
                    let func_id = FunctionId(self.next_func_id);
                    self.next_func_id += 1;

                    // method.value is already a Function
                    let func_value = &*method.value;
                    {
                        let mut builder = FunctionBuilder::new(func_id, method_name);

                        // Add 'this' parameter
                        builder.add_param("this".to_string(), Type::Object(type_id));

                        // Add other parameters
                        for (i, param) in func_value.params.items.iter().enumerate() {
                            let param_name = self.extract_param_name(param, i);
                            builder.add_param(param_name, Type::Any);
                        }

                        // Compile method body
                        if let Some(body) = &func_value.body {
                            let mut lowerer = StatementLowerer::new(builder);
                            for stmt in &body.statements {
                                lowerer.lower_statement(stmt)?;
                            }
                            builder = lowerer.finish();
                        }

                        self.methods.push(builder.build());
                    }
                }
                ClassElement::StaticBlock(_) => {
                    // Static initialization block
                    // TODO: Implement static blocks
                }
                _ => {
                    // Other element types (accessors, etc.)
                }
            }
        }

        // Create type layout
        let layout = TypeLayout {
            size: offset,
            alignment: 8,
            fields,
        };

        self.classes.push(layout);

        Ok(type_id)
    }

    /// Extract property name from property key
    fn extract_property_name(&self, key: &PropertyKey) -> String {
        match key {
            PropertyKey::StaticIdentifier(ident) => ident.name.to_string(),
            PropertyKey::PrivateIdentifier(ident) => format!("#{}", ident.name),
            PropertyKey::NumericLiteral(lit) => lit.value.to_string(),
            PropertyKey::StringLiteral(lit) => lit.value.to_string(),
            _ => "__unknown__".to_string(),
        }
    }

    fn extract_param_name(&self, param: &FormalParameter, index: usize) -> String {
        match &param.pattern.kind {
            BindingPatternKind::BindingIdentifier(ident) => ident.name.to_string(),
            _ => format!("__param_{}", index),
        }
    }

    /// Get compiled classes and methods
    pub fn get_classes(self) -> (Vec<TypeLayout>, Vec<TypedFunction>) {
        (self.classes, self.methods)
    }
}
