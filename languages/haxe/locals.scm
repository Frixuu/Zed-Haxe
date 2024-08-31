; Originally from https://github.com/vantreeseba/tree-sitter-haxe/blob/main/queries/locals.scm

[
 (block)
 (function_declaration)
] @scope @local.scope

; Definitions
(function_arg name: (identifier) @definition.parameter)
(variable_declaration name: (identifier) @local.definition)

; References
(block (identifier)) @local.reference
