; Originally from https://github.com/tong/tree-sitter-haxe/blob/main/queries/highlights.scm

(comment) @comment
(line_comment) @comment.line
(block_comment) @comment.block

(identifier) @variable
(type_name) @type

(wildcard_pattern) @constant.builtin

["{" "}" "[" "]" "(" ")"] @punctuation.bracket
["<" ">"] @punctuation.bracket
["." "," ";" ":" "..."] @punctuation.delimiter
["?" "??"] @punctuation.special
["->" "=>"] @keyword.operator

[
  "extends"
  "from"
  "implements"
  "package"
  "to"
  "untyped"
  "var"
] @keyword

[
  "if"
  "else"
  "switch"
  "case"
  "default"
] @keyword.conditional

[
  "for"
  "while"
  "do"
  "break"
  "continue"
] @keyword.repeat

[
  "try"
  "catch"
  "throw"
] @keyword.exception

[
  "as"
  "cast"
  "in"
  "new"
] @keyword.operator

[
  "abstract"
  "class"
  "enum"
  "interface"
  "typedef"
] @keyword.type

["return"] @keyword.return
["function"] @keyword.function

[
  "public"
  "private"
  "final"
  "inline"
  "override"
  "overload"
  "static"
  "dynamic"
  "macro"
  "extern"
] @keyword.modifier
(ClassType "abstract" @keyword.modifier)
(ClassMethod "abstract" @keyword.modifier)

(String) @string
(String (escape_sequence) @string.escape)
(String (interpolation) @string.special)
(Regexp) @string.regexp

(Int) @number
(Float) @number.float

(true) @boolean
(false) @boolean

(null) @constant.builtin
(super) @variable.builtin
(this) @variable.builtin

(package_name) @module

(import "import" @keyword.import)
(using "using" @keyword.import)
(import alias: (identifier) @type)
(import sub: (identifier) @property)
(import (wildcard) @constant)

; Types -----------------------------------------------------------------------

(ComplexType "->" @keyword.operator)
(ComplexType "&" @operator)

(TypePath sub: (identifier) @property)

(TypeParameter name: (type_name) @type.parameter)

(FunctionArg name: (identifier) @variable.parameter)
(FunctionArg "=" @punctuation.special)
(FunctionTypeArg name: (identifier) @variable.parameter)

; Declarations ----------------------------------------------------------------

(AbstractType name: (type_name) @type.definition)
(ClassType name: (type_name) @type.definition)
(EnumType name: (type_name) @type.definition)
(DefType name: (type_name) @type.definition)

(ClassVar "=" @operator)
(ClassVar
  (property_accessor
    get: (property_access) @property
    set: (property_access) @property))

(ClassMethod name: (identifier) @function.method)
(ClassMethod "macro" name: (identifier) @function.macro)

(EnumConstructor name: (identifier) @constant)

(Field name: (identifier) @variable.member)

; Expressions -----------------------------------------------------------------

(EBinop op: _ @operator)
(EUnop op: _ @operator)
(ETernary "?" @operator)
(ETernary ":" @operator)

(EField name: (identifier) @property)
(ECall callee: (identifier) @function.call)
(ECall callee: (EField name: (identifier) @function.call))
(EFunction name: (identifier) @function)
(EObjectDecl name: (identifier) @property)
(EObjectDecl name: (String) @property)
(EUntyped "untyped" @keyword.debug)
(EVars "final" @keyword)
(EVars "=" @operator)

; Macros ----------------------------------------------------------------------

(macro "macro" @keyword)
(reification) @function.macro

(type_trace "$type" @keyword.debug)

; Metadata --------------------------------------------------------------------

(MetaDataEntry "@" @attribute)
(MetaDataEntry ":" @attribute)
(MetaDataEntry name: (identifier) @attribute)

; Conditional compilation -----------------------------------------------------

["#if" "#elseif" "#else" "#error"] @keyword.directive
(conditional_end) @keyword.directive
(compile_condition (identifier) @constant.macro)
