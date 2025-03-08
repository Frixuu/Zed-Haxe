(class_declaration
    body: (
        _
        "{"
        (_)* @class.inside
        "}"
    )
) @class.around

(function_declaration
    body: (
        _
        "{"
        (_)* @function.inside
        "}"
    )
) @function.around
