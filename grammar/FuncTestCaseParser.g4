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
    : TripleHash (SubstraitScalarTest | SubstraitAggregateTest) Colon FormatVersion
    ;

include
    : TripleHash SubstraitInclude Colon StringLiteral (Comma StringLiteral)*
    ;

testGroupDescription
    : DescriptionLine
    ;

testCase
    : functionName=identifier OParen arguments CParen ( OBracket func_options CBracket )? Eq result
    ;

testGroup
    : testGroupDescription (testCase)+                          #scalarFuncTestGroup
    | testGroupDescription (aggFuncTestCase)+                   #aggregateFuncTestGroup
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
    | intArg
    | floatArg
    | booleanArg
    | stringArg
    | decimalArg
    | dateArg
    | timeArg
    | timestampArg
    | timestampTzArg
    | intervalYearArg
    | intervalDayArg
    | listArg
    ;

aggFuncTestCase
    : aggFuncCall ( OBracket func_options CBracket )? Eq result
    ;

aggFuncCall
    : tableData funcName=identifier OParen qualifiedAggregateFuncArgs CParen            #multiArgAggregateFuncCall
    | tableRows functName=identifier OParen aggregateFuncArgs CParen                    #compactAggregateFuncCall
    | functName=identifier OParen dataColumn CParen                                     #singleArgAggregateFuncCall
    ;

tableData
    : Define tableName=Identifier OParen dataType (Comma dataType)* CParen Eq tableRows
    ;

tableRows
    : OParen (columnValues (Comma columnValues)*)? CParen
    ;

dataColumn
    : columnValues DoubleColon dataType
    ;

columnValues
    : OParen (literal (Comma literal)*)? CParen
    ;

literal
    : NullLiteral
    | numericLiteral
    | BooleanLiteral
    | StringLiteral
    | DateLiteral
    | TimeLiteral
    | TimestampLiteral
    | TimestampTzLiteral
    | IntervalYearLiteral
    | IntervalDayLiteral
    ;

qualifiedAggregateFuncArgs
    : qualifiedAggregateFuncArg (Comma qualifiedAggregateFuncArg)*
    ;

aggregateFuncArgs
    : aggregateFuncArg (Comma aggregateFuncArg)*
    ;

qualifiedAggregateFuncArg
    : tableName=Identifier Dot ColumnName
    | argument
    ;

aggregateFuncArg
    : ColumnName DoubleColon dataType
    | argument
    ;

numericLiteral
    : DecimalLiteral | IntegerLiteral | floatLiteral
    ;

floatLiteral
    : FloatLiteral | NaN
    ;

nullArg: NullLiteral DoubleColon dataType;

intArg: IntegerLiteral DoubleColon (I8 | I16 | I32 | I64);

floatArg: numericLiteral DoubleColon (FP32 | FP64);

decimalArg
    : numericLiteral DoubleColon decimalType
    ;

booleanArg
    : BooleanLiteral DoubleColon booleanType
    ;

stringArg
    : StringLiteral DoubleColon stringType
    ;

dateArg
    : DateLiteral DoubleColon Date
    ;

timeArg
    : TimeLiteral DoubleColon Time
    ;

timestampArg
    : TimestampLiteral DoubleColon timestampType
    ;

timestampTzArg
    : TimestampTzLiteral DoubleColon timestampTZType
    ;

intervalYearArg
    : IntervalYearLiteral DoubleColon intervalYearType
    ;

intervalDayArg
    : IntervalDayLiteral DoubleColon intervalDayType
    ;

listArg
    : literalList DoubleColon listType
    ;

literalList
    : OBracket (literal (Comma literal)*)? CBracket
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

dataType
    : scalarType
    | parameterizedType
    ;

scalarType
  : booleanType             #boolean
  | I8                      #i8
  | I16                     #i16
  | I32                     #i32
  | I64                     #i64
  | FP32                    #fp32
  | FP64                    #fp64
  | stringType              #string
  | binaryType              #binary
  | timestampType           #timestamp
  | timestampTZType         #timestampTz
  | Date                    #date
  | Time                    #time
  | intervalDayType         #intervalDay
  | intervalYearType        #intervalYear
  | UUID                    #uuid
  | UserDefined Identifier  #userDefined
  ;

booleanType
    : (Bool | Boolean)
    ;

stringType
    : (Str | String)
    ;

binaryType
    : (Binary | VBin)
    ;

timestampType
    : (Ts | Timestamp)
    ;

timestampTZType
    : (TsTZ | Timestamp_TZ)
    ;

intervalYearType
    : (IYear | Interval_Year)
    ;

intervalDayType
    : (IDay | Interval_Day)
    ;

fixedCharType
    : (FChar | FixedChar) isnull=QMark? OAngleBracket len=numericParameter CAngleBracket #fixedChar
    ;

varCharType
    : (VChar | VarChar) isnull=QMark? OAngleBracket len=numericParameter CAngleBracket #varChar
    ;

fixedBinaryType
    : (FBin | FixedBinary) isnull=QMark? OAngleBracket len=numericParameter CAngleBracket #fixedBinary
    ;

decimalType
    : (Dec | Decimal) isnull=QMark?
        (OAngleBracket precision=numericParameter Comma scale=numericParameter CAngleBracket)?  #decimal
    ;

precisionTimestampType
    : (PTs | Precision_Timestamp) isnull=QMark?
        OAngleBracket precision=numericParameter CAngleBracket #precisionTimestamp
    ;

precisionTimestampTZType
    : (PTsTZ | Precision_Timestamp_TZ) isnull=QMark?
        OAngleBracket precision=numericParameter CAngleBracket #precisionTimestampTZ
    ;

listType
    : List isnull=QMark? OAngleBracket elemType=dataType CAngleBracket #list
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
    : Overflow | Rounding | NullHandling | SpacesOnly
    | Identifier
    ;

option_value
    : Error | Saturate | Silent | TieToEven | NaN | Truncate | AcceptNulls | IgnoreNulls
    | BooleanLiteral
    | NullLiteral
    | Identifier
    ;

func_options
    : func_option (Comma func_option)*
    ;

nonReserved //  IMPORTANT: this rule must only contain tokens
    : And | Or | Truncate
    ;

identifier
    : nonReserved
    | Identifier
    ;
