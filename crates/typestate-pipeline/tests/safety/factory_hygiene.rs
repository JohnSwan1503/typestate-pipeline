#[path = "factory_hygiene/tests/default_expression_can_call_user_scope_function.rs"]
mod default_expression_can_call_user_scope_function;

#[path = "factory_hygiene/tests/struct_with_field_names_matching_macro_internals_compiles.rs"]
mod struct_with_field_names_matching_macro_internals_compiles;

// ---------------------------------------------------------------------------
// Field names that match macro-internal bindings under the old codegen.
// ---------------------------------------------------------------------------

#[test]
fn struct_with_field_names_matching_macro_internals_compiles() {
    struct_with_field_names_matching_macro_internals_compiles::main();
}

// ---------------------------------------------------------------------------
// `default = …` expression with user-declared bindings of its own.
// ---------------------------------------------------------------------------

#[test]
pub fn default_expression_can_call_user_scope_function() {
    default_expression_can_call_user_scope_function::main();
}
