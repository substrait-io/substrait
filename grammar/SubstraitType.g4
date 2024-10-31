grammar SubstraitType;

options {
    caseInsensitive = true;
}

import SubstraitLexer;

startRule: expr EOF;

typeStatement: typeDef EOF;

scalarType
  : Boolean                 #boolean
  | I8                      #i8
  | I16                     #i16
  | I32                     #i32
  | I64                     #i64
  | FP32                    #fp32
  | FP64                    #fp64
  | String                  #string
  | Binary                  #binary
  | Timestamp               #timestamp
  | Timestamp_TZ            #timestampTz
  | Date                    #date
  | Time                    #time
  | Interval_Year           #intervalYear
  | UUID                    #uuid
  ;

parameterizedType
  : FixedChar isnull=QMark? Lt length=numericParameter Gt                                   #fixedChar
  | VarChar isnull=QMark? Lt length=numericParameter Gt                                     #varChar
  | FixedBinary isnull=QMark? Lt length=numericParameter Gt                                 #fixedBinary
  | Decimal isnull=QMark? Lt precision=numericParameter Comma scale=numericParameter Gt     #decimal
  | Interval_Day isnull=QMark? Lt precision=numericParameter Gt                             #precisionIntervalDay
  | Precision_Timestamp isnull=QMark? Lt precision=numericParameter Gt                      #precisionTimestamp
  | Precision_Timestamp_TZ isnull=QMark? Lt precision=numericParameter Gt                   #precisionTimestampTZ
  | Struct isnull=QMark? Lt expr (Comma expr)* Gt                                           #struct
  | NStruct isnull=QMark? Lt Identifier expr (Comma Identifier expr)* Gt                    #nStruct
  | List isnull=QMark? Lt expr Gt                                                           #list
  | Map isnull=QMark? Lt key=expr Comma value=expr Gt                                       #map
  | UserDefined Identifier isnull=QMark? (Lt expr (Comma expr)* Gt)?                        #userDefined
  ;

numericParameter
  : Number              #numericLiteral
  | Identifier          #numericParameterName
  | expr                #numericExpression
  ;

anyType
  : Any isnull=QMark?
  | AnyVar isnull=QMark?
  ;

typeDef
  : scalarType isnull=QMark?
  | parameterizedType
  | anyType
  ;

expr
  : OParen expr CParen                                                                      #ParenExpression
  | Identifier Eq expr Newline+ (Identifier Eq expr Newline+)* finalType=typeDef Newline*   #MultilineDefinition
  | typeDef                                                                                 #TypeLiteral
  | number=Number                                                                           #LiteralNumber
  | identifier=Identifier isnull=QMark?                                                     #ParameterName
  | Identifier OParen (expr (Comma expr)*)? CParen                                          #FunctionCall
  | left=expr op=(And | Or | Plus | Minus | Lt | Gt | Eq | Ne |
        Lte | Gte | Asterisk | ForwardSlash) right=expr                                     #BinaryExpr
  | If ifExpr=expr Then thenExpr=expr Else elseExpr=expr                                    #IfExpr
  | (Bang) expr                                                                             #NotExpr
  | ifExpr=expr QMark thenExpr=expr Colon elseExpr=expr                                     #Ternary
  ;
