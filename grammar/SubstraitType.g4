grammar SubstraitType;

import SubstraitLexer;

// OPERATIONS
And      : A N D;
Or       : O R;
Assign   : ':=';

// COMPARE
Eq       : '=';
NotEquals: '!=';
Gte      : '>=';
Lte      : '<=';
Gt       : '>';
Lt       : '<';
Bang     : '!';


// MATH
Plus      : '+';
Minus : '-';
Asterisk : '*';
ForwardSlash   : '/';
Percent  : '%';

// ORGANIZE
OBracket : '[';
CBracket : ']';
OParen   : '(';
CParen   : ')';
SColon   : ';';
Comma    : ',';
QMark    : '?';
Colon    : ':';
SingleQuote: '\'';


Number
  : '-'? Int
  ;

Identifier
  : ('a'..'z' | 'A'..'Z' | '_' | '$') ('a'..'z' | 'A'..'Z' | '_' | '$' | Digit)*
  ;

Newline
  : ( '\r' '\n'?
    | '\n'
    )
  ;


fragment Int
  : '1'..'9' Digit*
  | '0'
  ;

fragment Digit
  : '0'..'9'
  ;

start: expr EOF;

scalarType
  : Boolean #Boolean
  | I8 #i8
  | I16 #i16
  | I32 #i32
  | I64 #i64
  | FP32 #fp32
  | FP64 #fp64
  | String #string
  | Binary #binary
  | Timestamp #timestamp
  | Timestamp_TZ #timestampTz
  | Date #date
  | Time #time
  | IntervalDay #intervalDay
  | Interval_Year #intervalYear
  | UUID #uuid
  | UserDefined Identifier #userDefined
  ;

parameterizedType
  : FixedChar isnull=QMark? Lt len=numericParameter Gt                                      #fixedChar
  | VarChar isnull=QMark? Lt len=numericParameter Gt                                        #varChar
  | FixedBinary isnull=QMark? Lt len=numericParameter Gt                                    #fixedBinary
  | Decimal isnull=QMark? Lt precision=numericParameter Comma scale=numericParameter Gt     #decimal
  | Precision_Timestamp isnull=QMark? Lt precision=numericParameter Gt                      #precisionTimestamp
  | Precision_Timestamp_TZ isnull=QMark? Lt precision=numericParameter Gt                   #precisionTimestampTZ
  | Struct isnull=QMark? Lt expr (Comma expr)* Gt                                           #struct
  | NStruct isnull=QMark? Lt Identifier expr (Comma Identifier expr)* Gt                    #nStruct
  | List isnull=QMark? Lt expr Gt                                                           #list
  | Map isnull=QMark? Lt key=expr Comma value=expr Gt                                       #map
  ;

numericParameter
  : Number #numericLiteral
  | Identifier #numericParameterName
  | expr #numericExpression
  ;

anyType: ANY;

type
  : scalarType isnull=QMark?
  | parameterizedType
  | anyType isnull=QMark?
  ;

//  : (OParen innerExpr CParen | innerExpr)

expr
  : OParen expr CParen #ParenExpression
  | Identifier Eq expr Newline+ (Identifier Eq expr Newline+)* finalType=type Newline* #MultilineDefinition
  | type #TypeLiteral
  | number=Number #LiteralNumber
  | identifier=Identifier isnull=QMark? #TypeParam
  | Identifier OParen (expr (Comma expr)*)? CParen #FunctionCall
  | left=expr op=(And | Or | Plus | Minus | Lt | Gt | Eq | NotEquals | Lte | Gte | Asterisk | ForwardSlash) right=expr #BinaryExpr
  | If ifExpr=expr Then thenExpr=expr Else elseExpr=expr #IfExpr
  | (Bang) expr #NotExpr
  | ifExpr=expr QMark thenExpr=expr Colon elseExpr=expr #Ternary
  ;

