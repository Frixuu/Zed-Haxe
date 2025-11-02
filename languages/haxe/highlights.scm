; Originally from https://github.com/tong/tree-sitter-haxe/blob/main/queries/highlights.scm

(comment) @comment
(line_comment) @comment.line
(block_comment) @comment.block

(identifier) @variable
(type_name) @type

(wildcard_pattern) @constant.builtin

["{" "}" "[" "]" "(" ")"] @punctuation.bracket
["<" ">"] @punctuation.bracket
["," ";" ":" "..."] @punctuation.delimiter
["?" "??"] @punctuation.special
["->" "=>"] @keyword.operator

[
  "break"
  "case"
  "cast"
  "catch"
  "continue"
  "default"
  "do"
  "else"
  "enum"
  "extends"
  "extern"
  "for"
  "if"
  "implements"
  "in"
  "new"
  "package"
  "switch"
  "throw"
  "try"
  "untyped"
  "var"
  "while"
] @keyword

[
  "abstract"
  "class"
  "enum"
  "interface"
  "typedef"
] @keyword.declaration

["return"] @keyword.return
["function"] @keyword.function
["as"] @keyword.operator

[
  "public"
  "private"
  "final"
  "inline"
  "override"
  "static"
  "dynamic"
  "macro"
] @keyword.modifier

; "macro" @macro

(Int) @number
(Float) @number.float
(String (escape_sequence) @string.escape)
(String (fragment) @string)
(String (interpolation) @string.special)
(Regexp) @string.regex

(true) @boolean
(false) @boolean

(null) @constant.builtin
(super) @variable.builtin
(this) @variable.builtin

(package_name) @namespace

[
 (import)
 (using)
] @keyword.import

(import
  path: (package_name) @namespace
  module: (type_name) @module)
(import alias: (identifier) @type)
(import sub: (identifier) @property)
(import (wildcard) @constant)

(using
  path: (package_name) @namespace
  type: (type_name) @type)

;---------------------------------------------------


(ComplexType "->" @keyword.operator)

(TypePath
  pack: (package_name) @namespace
  name: (type_name) @type
  sub: (identifier) @property)

(TypeParameter
  name: (type_name) @type.parameter)
(TypeParameter
  name: (type_name) @type.parameter
  [":"] @punctuation.delimiter
  constraint: (ComplexType
    (TypePath
      name: (type_name) @type)))

(FunctionArg name: (identifier) @variable.parameter)
(FunctionArg "=" @punctuation.special)


(ComplexType "&" @operator)

; Declarations ----------------------------------------------------------------

[
  (AbstractType)
  (ClassType)
  (EnumType)
  (DefType)
] @type.definition

(ClassVar type: (ComplexType) @type)
(ClassVar
  (property_accessor
    get: (property_access) @property
    set: (property_access) @property))

(ClassMethod
  "function" @keyword.function
  name: (identifier) @function.method)
(ClassMethod "macro" name: (identifier) @function.macro)

(EnumConstructor name: (identifier) @constant)

(TAnonymous) @type.builtin
(AnonymousField name: (identifier) @variable.member)

; Expressions -----------------------------------------------------------------

(EBinop op: _ @operator)
(EUnop op: _ @operator)
(ETernary "?" @operator)
(ETernary ":" @operator)

(EField name: (identifier) @property)
(EField "." @punctuation.delimiter)
(ECast type: (ComplexType) @type)
(ECall callee: (identifier) @function.call)
(ECall callee: (EField name: (identifier) @function.call))
(EFor
  key: (identifier) @variable
  value: (identifier) @variable
  var: (identifier) @variable
)
(EFunction name: (identifier) @function)
(ENew (TypePath) @type)
(EObjectDecl name: (identifier) @property)
(EObjectDecl name: (String) @property)
; (ETry (identifier) @variable)
(ESwitch "switch" @keyword.control)
(switch_case "case" @keyword.control)
(switch_default "default" @keyword.control)
(EThrow expr: (_) @keyword.exception)
(EUntyped "untyped" @keyword.debug)
(EVars
  "final" @keyword
  name: (identifier) @variable)

; macro reification -----------------------------------------------------------

"macro" @macro
(macro (reification)) @macro

; Metadata --------------------------------------------------------------------

(MetaDataEntry
  [
    "@" @attribute
    ":" @attribute
    name: (identifier) @attribute
    params: (_) @attribute.parameter
    "(" @punctuation.bracket
    ")" @punctuation.bracket
    (ECall callee: (EField name: (identifier) @function.call))
  ])

(type_trace
  "$type" @keyword.debug)

[
  (conditional)
  (conditional_elseif)
  (conditional_else)
  (conditional_end)
  (conditional_error)
] @keyword.directive
