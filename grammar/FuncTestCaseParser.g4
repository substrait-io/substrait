parser grammar FuncTestCaseParser;

options {
    caseInsensitive = true;
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
    : SubstraitScalarTest FormatVersion
    ;

include
    : SubstraitInclude StringLiteral (Comma StringLiteral)*
    ;

testGroupDescription
    : DescriptionLine
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
    : DecimalLiteral | IntegerLiteral | floatLiteral
    ;

floatLiteral
    : FloatLiteral | NaN
    ;

nullArg: NullLiteral DoubleColon datatype;

i8Arg: IntegerLiteral DoubleColon I8;

i16Arg: IntegerLiteral DoubleColon I16;

i32Arg: IntegerLiteral DoubleColon I32;

i64Arg: IntegerLiteral DoubleColon I64;

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
    : BooleanLiteral DoubleColon Bool
    ;

stringArg
    : StringLiteral DoubleColon Str
    ;

dateArg
    : DateLiteral DoubleColon Date
    ;

timeArg
    : TimeLiteral DoubleColon Time
    ;

timestampArg
    : TimestampLiteral DoubleColon Ts
    ;

timestampTzArg
    : TimestampTzLiteral DoubleColon TsTZ
    ;

intervalYearArg
    : IntervalYearLiteral DoubleColon IYear
    ;

intervalDayArg
    : IntervalDayLiteral DoubleColon IDay
    ;

intervalYearLiteral
    : PeriodPrefix (years=IntegerLiteral YearPrefix) (months=IntegerLiteral MSuffix)?
    | PeriodPrefix (months=IntegerLiteral MSuffix)
    ;

intervalDayLiteral
    : PeriodPrefix (days=IntegerLiteral DaySuffix) (TimePrefix timeInterval)?
    | PeriodPrefix TimePrefix timeInterval
    ;

timeInterval
    : hours=IntegerLiteral HourSuffix (minutes=IntegerLiteral MSuffix)? (seconds=IntegerLiteral SecondSuffix)?
        (fractionalSeconds=IntegerLiteral FractionalSecondSuffix)?
    | minutes=IntegerLiteral MSuffix (seconds=IntegerLiteral SecondSuffix)? (fractionalSeconds=IntegerLiteral FractionalSecondSuffix)?
    | seconds=IntegerLiteral SecondSuffix (fractionalSeconds=IntegerLiteral FractionalSecondSuffix)?
    | fractionalSeconds=IntegerLiteral FractionalSecondSuffix
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
  : IntegerLiteral #integerLiteral
  ;

substraitError
    : ErrorResult | UndefineResult
    ;

func_option
    : option_name Colon option_value
    ;

option_name
    : Overflow | Rounding
    | Identifier
    ;

option_value
    : Error | Saturate | Silent | TieToEven | NaN
    ;

func_options
    : func_option (Comma func_option)*
    ;
