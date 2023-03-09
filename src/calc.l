%%
[0-9]+ "INTEGER"
\+ "ADD"
\* "MUL"
\( "("
\) ")"
; ";"
println "PRINT_LN" 
[\t\n ]+ ;
. "UNMATCHED"
