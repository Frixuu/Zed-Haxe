; Originally from https://github.com/tong/tree-sitter-haxe/blob/main/queries/indents.scm

; Bracketed bodies, `{ … }`, `[ … ]` and `( … )` spanning several lines.
[
  (ClassType)
  (AbstractType)
  (EnumType)
  (TAnonymous)
  (EBlock)
  (EObjectDecl)
  (EArrayDecl)
  (ECall)
  (ENew)
  (ESwitch)
  (switch_case)
  (switch_default)
] @indent.begin

; An `if` indents its branches, the `else if` chain all at one level:
;
;   if (x)
;     f();
;   else if (y) {
;     g();
;   } else
;     h();
;
; Known limitation: a then-block opening on a later line than the `if`, with
; the brace on its own line or after a multi-line condition, is indented
; one level too far. Queries can't tell whether two nodes share a line.
((EIf) @indent.begin
  (#not-has-parent? @indent.begin EIf))

; A braced branch opening on a later line than the `if` adds no level of
; its own, or its contents would be indented by both.
(EIf
  else: (EBlock) @indent.dedent)

(EIf
  else: (EIf
    if: (EBlock) @indent.dedent))

; The same for `catch` blocks.
;
; Known limitation: in `try f() catch (e) {` on one line, the block's
; contents get no level at all.
(ETry) @indent.begin

(ETry
  body: (EBlock) @indent.dedent)

(ETry
  "catch" @indent.branch)

; Loop and function bodies without braces go on the next line, one level in.
(EFor
  body: (_) @_body
  (#not-kind-eq? @_body "EBlock")) @indent.begin

(EWhile
  body: (_) @_body
  (#not-kind-eq? @_body "EBlock")) @indent.begin

(EFunction
  body: (_) @_body
  (#not-kind-eq? @_body "EBlock")) @indent.begin

(ClassMethod
  body: (_) @_body
  (#not-kind-eq? @_body "EBlock")) @indent.begin

; `else` lines up with its `if`.
(EIf
  "else" @indent.branch)

; A closing bracket lines up with the line that opened it.
[
  "}"
  "]"
  ")"
] @indent.branch

; So does an opening brace on a line of its own, for braces that would
; otherwise be indented by the enclosing node. Not `(` or `[`, which may
; start a statement, `(x : T).f();`.
[
  (ClassType "{" @indent.branch)
  (AbstractType "{" @indent.branch)
  (EnumType "{" @indent.branch)
  (ESwitch "{" @indent.branch)
  (EIf (EBlock . "{" @indent.branch))
  (ETry (EBlock . "{" @indent.branch))
]

"}" @indent.end

; Keep the author's indentation inside comments and multi-line strings.
[
  (block_comment)
  (String)
  (ERROR)
] @indent.auto
