; Originally from https://github.com/tong/tree-sitter-haxe/blob/main/queries/folds.scm

[
  (block_comment)
  (conditional)
] @fold

[
  (EArrayDecl)
  (EBlock)
  (EFunction)
  (EObjectDecl)
  (TAnonymous)
] @fold

[
  (AbstractType)
  (ClassType)
  (DefType)
  (EnumType)
] @fold

[
  (EFor)
  (EIf)
  (ESwitch)
  (ETry)
  (EWhile)
] @fold
