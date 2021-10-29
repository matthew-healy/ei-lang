# Grammar

program = statement* ;

statement            = expression_statement ;
expression_statement = expression ";" ;

expression           = function_application | primary ;
function_application = IDENTIFIER "(" arguments? ")" ;
arguments            = expression ("," expression)* ;
primary              = STRING | IDENTIFIER ;