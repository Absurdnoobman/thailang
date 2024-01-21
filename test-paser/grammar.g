%lex

\s+     return "";
[0-9]+|[๐-๙]+     return "Number";

"{"     return "{";
"}"     return "}";
";"     return ";";

","     return ",";

"-"     return "-";
"+"     return "+";
"/"     return "/";
"*"     return "*";
"^"

"("     return "(";
")"     return ")";

"โปรแกรม" return "programe";
"คืนค่า"  return "Return";

/lex

%left +
%left *
%left ^

%%

Programe:
    โปรแกรม IdentifierName ; StatementList

StatementList:
    Statement
    | StatementList Statement

Statement:
    FunctionDeclaration
    | ClassDeclaration

FunctionDeclaration:
    ReturnType IdentifierName ( ParametersList ) FuncCodeBlock

Parameter:
    Type IdentifierName

ParametersList:
    Parameter
    | ParametersList , Parameter

FuncCodeBlock:
    { MultipleExpression ReturnExpression}
    | { Expression ReturnExpression }

ReturnExpression:
    คืนค่า BooleanExpression

IdentifierName:
    StringLiteral