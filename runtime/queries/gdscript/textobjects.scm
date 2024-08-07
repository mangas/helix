
(class_definition
  (body) @class.inside) @class.around

(function_definition
  (body) @function.inside) @function.around

(lambda (body) @function.inside) @function.around

(parameters 
  [
    (identifier)
    (typed_parameter)
    (default_parameter)    
    (typed_default_parameter)  
  ] @parameter.inside @parameter.around)

(arguments (_expression) @parameter.inside @parameter.around)

[
  (const_statement)
  (variable_statement)
  (pair)
  (enumerator)
] @entry.around

(comment) @comment.inside
(comment)+ @comment.around
