%%
[0-9]+ "INTEGER_LITERAL"
(true|false) "BOOLEAN_LITERAL"
\+ "ADD"
\* "MUL"
\< "LESS_THAN"
> "GREATER_THAN"
\( "("
\) ")"
\{ "{"
\} "}"
: "COLON"
\? "QUESTION"
; ";"
, ","
= "="
== "EQEQ"
!= "NOTEQ"
&& "AND"
\|\| "OR"
if "IF"
else "ELSE"
fun "FUNCTION"
let "LET" 
return "RETURN"
println "PRINT_LN" 
[a-zA-Z0-9_]+ "IDENTIFIER"
[\t\n ]+ ;
//[^\n]*?$ ;
. "UNMATCHED"
