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
    : version include dependency*
    ;

version
    : TripleHash (SubstraitScalarTest | SubstraitAggregateTest) Colon FormatVersion
    ;

include
    : TripleHash SubstraitInclude Colon StringLiteral (Comma StringLiteral)*
    ;

dependency
    : TripleHash SubstraitDependency Colon StringLiteral
    ;

testGroupDescription
    : DescriptionLine
    ;

testCase
    : functionName=identifier OParen arguments? CParen ( OBracket funcOptions CBracket )? Eq result
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
    | lambdaArg
    | Identifier  // Bare identifiers (for lambda parameters)
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

intArg: IntegerLiteral DoubleColon intType;

floatArg: numericLiteral DoubleColon floatType;

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
    : DateLiteral DoubleColon dateType
    ;

timeArg
    : TimeLiteral DoubleColon timeType
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

fixedCharArg
    : StringLiteral DoubleColon fixedCharType
    ;

varCharArg
    : StringLiteral DoubleColon varCharType
    ;

fixedBinaryArg
    : StringLiteral DoubleColon fixedBinaryType
    ;

precisionTimeArg
    : TimeLiteral DoubleColon precisionTimeType
    ;

precisionTimestampArg
    : TimestampLiteral DoubleColon precisionTimestampType
    ;

precisionTimestampTZArg
    : TimestampTzLiteral DoubleColon precisionTimestampTZType
    ;

listArg
    : literalList DoubleColon listType
    ;

lambdaArg
    : literalLambda DoubleColon funcType
    ;

literalList
    : OBracket (literal (Comma literal)*)? CBracket
    ;

literalLambda
    : OParen lambdaParameters Arrow lambdaBody CParen
    ;

lambdaParameters
    : Identifier                                    # singleParam
    | OParen Identifier (Comma Identifier)+ CParen  # tupleParams
    ;

lambdaBody
    : identifier OParen arguments CParen
    ; // For now, we only allow lambda bodies of the form func_name(arg1,arg2,...)

dataType
    : scalarType
    | parameterizedType
    ;

scalarType
  : booleanType                          #boolean
  | intType                              #int
  | floatType                            #float
  | stringType                           #string
  | binaryType                           #binary
  | timestampType                        #timestamp
  | timestampTZType                      #timestampTz
  | dateType                             #date
  | timeType                             #time
  | intervalYearType                     #intervalYear
  | UUID isnull=QMark?                   #uuid
  | UserDefined Identifier isnull=QMark? #userDefined
  ;

booleanType
    : (Bool | Boolean) isnull=QMark?
    ;

stringType
    : (Str | String) isnull=QMark?
    ;

binaryType
    : (Binary | VBin) isnull=QMark?
    ;

intType
    : (I8 | I16 | I32 | I64) isnull=QMark?
    ;

floatType
    : (FP32 | FP64) isnull=QMark?
    ;

dateType
    : Date isnull=QMark?
    ;

timeType
    : Time isnull=QMark?
    ;

timestampType
    : (Ts | Timestamp) isnull=QMark?
    ;

timestampTZType
    : (TsTZ | Timestamp_TZ) isnull=QMark?
    ;

intervalYearType
    : (IYear | Interval_Year) isnull=QMark?
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

funcType
    : Func isnull=QMark? OAngleBracket params=funcParameters Arrow returnType=dataType CAngleBracket
    ;

funcParameters
    : dataType                                  #singleFuncParam
    | OParen dataType (Comma dataType)* CParen  #funcParamsWithParens
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
    | funcType
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
