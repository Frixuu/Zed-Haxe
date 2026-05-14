; ─── Type declarations ─────────────────────────────────────────────────────

(ClassType
  [
    "public"
    "private"
    "abstract"
    "extern"
    "final"
  ]* @context
  "class" @context
  name: (type_name) @name) @item

(ClassType
  [
    "public"
    "private"
    "abstract"
    "extern"
    "final"
  ]* @context
  "interface" @context
  name: (type_name) @name) @item

(AbstractType
  [
    "public"
    "private"
    "enum"
  ]* @context
  "abstract" @context
  name: (type_name) @name) @item

(DefType
  [
    "public"
    "private"
    "extern"
  ]* @context
  "typedef" @context
  name: (type_name) @name) @item

(EnumType
  [
    "public"
    "private"
    "extern"
  ]* @context
  "enum" @context
  name: (type_name) @name) @item

; ─── Class body members ────────────────────────────────────────────────────

; `macro` is a modifier on a `function` declaration, not a separate keyword —
; one rule covers both regular and macro methods.
(ClassMethod
  [
    "public"
    "private"
    "static"
    "override"
    "inline"
    "dynamic"
    "final"
    "macro"
  ]* @context
  "function" @context
  name: (identifier) @name
  "(" @context
  ")" @context) @item

(ClassVar
  [
    "public"
    "private"
    "static"
    "inline"
    "dynamic"
  ]* @context
  ["var" "final"] @context
  name: (identifier) @name) @item

; ─── Enum constructors (nest under parent EnumType @item) ──────────────────

(EnumConstructor
  name: (identifier) @name) @item

; ─── Annotations attach to the following outline item ─────────────────────
; Metadata (@:native(...), @:noCompletion, etc.) and doc/line comments are
; surfaced to the Assistant for edit-step generation.

(MetaDataEntry) @annotation

[
  (line_comment)
  (block_comment)
] @annotation
