; Originally from https://github.com/tong/tree-sitter-haxe/blob/main/queries/locals.scm
[
  (EBlock)
  (EFunction)
] @scope @local.scope

(FunctionArg
  name: (identifier) @definition.parameter)

; (EVars name: (identifier) @local.definition)
; (EBlock (identifier)) @local.reference
(identifier) @local.reference
