parser grammar FuncTestCaseParser;

options {
    tokenVocab=SubstraitLexer;
    tokenVocab=FuncTestCaseLexer;
}

doc
    : header testGroup+ EOF
    ;

header
    : version include
    ;

version
    : SUBSTRAIT_SCALAR_TEST FORMAT_VERSION
    ;

include
    : SUBSTRAIT_INCLUDE STRING_LITERAL (Comma STRING_LITERAL)*
    ;

testGroupDescription
    : DESCRIPTION_LINE
    ;

testCase
    : functionName=Identifier OParen arguments CParen ( OBracket func_options CBracket )? Eq result
    ;

testGroup
    : testGroupDescription (testCase)+
    ;

arguments
    : argument (Comma argument)*
    ;

result
    : argument
    | substraitError
    ;

argument
    : nullArg
    | i8Arg | i16Arg | i32Arg | i64Arg
    | fp32Arg | fp64Arg
    | booleanArg
    | stringArg
    | decimalArg
    | dateArg
    | timeArg
    | timestampArg
    | timestampTzArg
    | intervalYearArg
    | intervalDayArg
    ;

numericLiteral
    : DECIMAL_LITERAL | INTEGER_LITERAL | FLOAT_LITERAL
    ;

nullArg: NULL_LITERAL DoubleColon datatype;

i8Arg: INTEGER_LITERAL DoubleColon I8;

i16Arg: INTEGER_LITERAL DoubleColon I16;

i32Arg: INTEGER_LITERAL DoubleColon I32;

i64Arg: INTEGER_LITERAL DoubleColon I64;

fp32Arg
    : numericLiteral DoubleColon FP32
    ;

fp64Arg
    : numericLiteral DoubleColon FP64
    ;

decimalArg
    : numericLiteral DoubleColon decimalType
    ;

booleanArg
    : BOOLEAN_LITERAL DoubleColon Bool
    ;

stringArg
    : STRING_LITERAL DoubleColon Str
    ;

dateArg
    : DATE_LITERAL DoubleColon Date
    ;

timeArg
    : TIME_LITERAL DoubleColon Time
    ;

timestampArg
    : TIMESTAMP_LITERAL DoubleColon Ts
    ;

timestampTzArg
    : TIMESTAMP_TZ_LITERAL DoubleColon TsTZ
    ;

intervalYearArg
    : INTERVAL_YEAR_LITERAL DoubleColon IYear
    ;

intervalDayArg
    : INTERVAL_DAY_LITERAL DoubleColon IDay
    ;

intervalYearLiteral
    : PERIOD_PREFIX (years=INTEGER_LITERAL YEAR_SUFFIX) (months=INTEGER_LITERAL M_SUFFIX)?
    | PERIOD_PREFIX (months=INTEGER_LITERAL M_SUFFIX)
    ;

intervalDayLiteral
    : PERIOD_PREFIX (days=INTEGER_LITERAL DAY_SUFFIX) (TIME_PREFIX timeInterval)?
    | PERIOD_PREFIX TIME_PREFIX timeInterval
    ;

timeInterval
    : hours=INTEGER_LITERAL HOUR_SUFFIX (minutes=INTEGER_LITERAL M_SUFFIX)? (seconds=INTEGER_LITERAL SECOND_SUFFIX)?
        (fractionalSeconds=INTEGER_LITERAL FRACTIONAL_SECOND_SUFFIX)?
    | minutes=INTEGER_LITERAL M_SUFFIX (seconds=INTEGER_LITERAL SECOND_SUFFIX)? (fractionalSeconds=INTEGER_LITERAL FRACTIONAL_SECOND_SUFFIX)?
    | seconds=INTEGER_LITERAL SECOND_SUFFIX (fractionalSeconds=INTEGER_LITERAL FRACTIONAL_SECOND_SUFFIX)?
    | fractionalSeconds=INTEGER_LITERAL FRACTIONAL_SECOND_SUFFIX
    ;

datatype
    : scalarType
    | parameterizedType
    ;

scalarType
  : Bool #Boolean
  | I8 #i8
  | I16 #i16
  | I32 #i32
  | I64 #i64
  | FP32 #fp32
  | FP64 #fp64
  | Str #string
  | Binary #binary
  | Ts #timestamp
  | TsTZ #timestampTz
  | Date #date
  | Time #time
  | IDay #intervalDay
  | IYear #intervalYear
  | UUID #uuid
  | UserDefined Identifier #userDefined
  ;

fixedCharType
    : FChar isnull=QMark? OAngleBracket len=numericParameter CAngleBracket #fixedChar
    ;

varCharType
    : VChar isnull=QMark? OAngleBracket len=numericParameter CAngleBracket #varChar
    ;

fixedBinaryType
    : FBin isnull=QMark? OAngleBracket len=numericParameter CAngleBracket #fixedBinary
    ;

decimalType
    : Dec isnull=QMark? (OAngleBracket precision=numericParameter Comma scale=numericParameter CAngleBracket)?  #decimal
    ;

precisionTimestampType
    : PTs isnull=QMark? OAngleBracket precision=numericParameter CAngleBracket #precisionTimestamp
    ;

precisionTimestampTZType
    : PTsTZ isnull=QMark? OAngleBracket precision=numericParameter CAngleBracket #precisionTimestampTZ
    ;

parameterizedType
    : fixedCharType
    | varCharType
    | fixedBinaryType
    | decimalType
    | precisionTimestampType
    | precisionTimestampTZType
// TODO implement the rest of the parameterized types
//  | Struct isnull='?'? Lt expr (Comma expr)* Gt #struct
//  | NStruct isnull='?'? Lt Identifier expr (Comma Identifier expr)* Gt #nStruct
//  | List isnull='?'? Lt expr Gt #list
//  | Map isnull='?'? Lt key=expr Comma value=expr Gt #map
  ;

numericParameter
  : INTEGER_LITERAL #integerLiteral
  ;

substraitError
    : ERROR_RESULT | UNDEFINED_RESULT
    ;

func_option
    : option_name Colon option_value
    ;

option_name
    : OVERFLOW | ROUNDING
    | Identifier
    ;

option_value
    : ERROR | SATURATE | SILENT | TIE_TO_EVEN | NAN
    ;

func_options
    : func_option (Comma func_option)*
    ;
