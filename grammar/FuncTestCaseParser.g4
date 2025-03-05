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
    : functionName=identifier OParen arguments CParen ( OBracket funcOptions CBracket )? Eq result
    ;

testGroup
    : testGroupDescription? (testCase)+                          #scalarFuncTestGroup
    | testGroupDescription? (aggFuncTestCase)+                   #aggregateFuncTestGroup
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
    | fixedCharArg
    | varCharArg
    | fixedBinaryArg
    | precisionTimeArg
    | precisionTimestampArg
    | precisionTimestampTZArg
    | listArg
    ;

aggFuncTestCase
    : aggFuncCall ( OBracket funcOptions CBracket )? Eq result
    ;

aggFuncCall
    : tableData funcName=identifier OParen qualifiedAggregateFuncArgs? CParen           #multiArgAggregateFuncCall
    | tableRows functName=identifier OParen aggregateFuncArgs? CParen                   #compactAggregateFuncCall
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

intArg: IntegerLiteral DoubleColon (I8 | I16 | I32 | I64) isnull=QMark?;

floatArg: numericLiteral DoubleColon (FP32 | FP64) isnull=QMark?;

decimalArg
    : numericLiteral DoubleColon decimalType isnull=QMark?
    ;

booleanArg
    : BooleanLiteral DoubleColon booleanType isnull=QMark?
    ;

stringArg
    : StringLiteral DoubleColon stringType isnull=QMark?
    ;

dateArg
    : DateLiteral DoubleColon Date isnull=QMark?
    ;

timeArg
    : TimeLiteral DoubleColon Time isnull=QMark?
    ;

timestampArg
    : TimestampLiteral DoubleColon timestampType isnull=QMark?
    ;

timestampTzArg
    : TimestampTzLiteral DoubleColon timestampTZType isnull=QMark?
    ;

intervalYearArg
    : IntervalYearLiteral DoubleColon intervalYearType isnull=QMark?
    ;

intervalDayArg
    : IntervalDayLiteral DoubleColon intervalDayType isnull=QMark?
    ;

fixedCharArg
    : StringLiteral DoubleColon fixedCharType isnull=QMark?
    ;

varCharArg
    : StringLiteral DoubleColon varCharType isnull=QMark?
    ;

fixedBinaryArg
    : StringLiteral DoubleColon fixedBinaryType isnull=QMark?
    ;

precisionTimeArg
    : TimeLiteral DoubleColon precisionTimeType isnull=QMark?
    ;

precisionTimestampArg
    : TimestampLiteral DoubleColon precisionTimestampType isnull=QMark?
    ;

precisionTimestampTZArg
    : TimestampTzLiteral DoubleColon precisionTimestampTZType isnull=QMark?
    ;

listArg
    : literalList DoubleColon listType isnull=QMark?
    ;

literalList
    : OBracket (literal (Comma literal)*)? CBracket
    ;

dataType
    : scalarType isnull=QMark?
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
    : (IDay | Interval_Day) isnull=QMark? (OAngleBracket len=numericParameter CAngleBracket)?
    ;

fixedCharType
    : (FChar | FixedChar) isnull=QMark? OAngleBracket len=numericParameter CAngleBracket
    ;

varCharType
    : (VChar | VarChar) isnull=QMark? OAngleBracket len=numericParameter CAngleBracket
    ;

fixedBinaryType
    : (FBin | FixedBinary) isnull=QMark? OAngleBracket len=numericParameter CAngleBracket
    ;

decimalType
    : (Dec | Decimal) isnull=QMark?
        (OAngleBracket precision=numericParameter Comma scale=numericParameter CAngleBracket)?
    ;

precisionTimeType
    : (PT | Precision_Time) isnull=QMark? OAngleBracket precision=numericParameter CAngleBracket
    ;

precisionTimestampType
    : (PTs | Precision_Timestamp) isnull=QMark? OAngleBracket precision=numericParameter CAngleBracket
    ;

precisionTimestampTZType
    : (PTsTZ | Precision_Timestamp_TZ) isnull=QMark? OAngleBracket precision=numericParameter CAngleBracket
    ;

listType
    : List isnull=QMark? OAngleBracket elemType=dataType CAngleBracket #list
    ;

parameterizedType
    : fixedCharType
    | varCharType
    | fixedBinaryType
    | decimalType
    | intervalDayType
    | precisionTimeType
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

funcOption
    : optionName Colon optionValue
    ;

optionName
    : Overflow | Rounding | NullHandling | SpacesOnly
    | Identifier
    ;

optionValue
    : Error | Saturate | Silent | TieToEven | NaN | Truncate | AcceptNulls | IgnoreNulls
    | BooleanLiteral
    | NullLiteral
    | Identifier
    ;

funcOptions
    : funcOption (Comma funcOption)*
    ;

nonReserved //  IMPORTANT: this rule must only contain tokens
    : And | Or | Truncate
    ;

identifier
    : nonReserved
    | Identifier
    ;
