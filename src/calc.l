%%
[0-9]+ "INTEGER"
println "PRINT_LN" 
let "ASSIGN"
fun "FUNCTION"
return "RETURN"
\+ "ADD"
\* "MUL"
\( "("
\) ")"
\{ "{"
\} "}"
; ";"
, ","
= "="
[a-zA-Z_][a-zA-Z0-9_]+ "T_ID"
[\t\n ]+ ;
//[^\n]*?$ ;
. "UNMATCHED"
