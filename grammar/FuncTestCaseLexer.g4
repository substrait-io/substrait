lexer grammar FuncTestCaseLexer;

import SubstraitLexer;

options {
    caseInsensitive = true;
}

Whitespace    : [ \t\n\r]+ -> channel(HIDDEN) ;

TripleHash: '###';
SubstraitScalarTest: 'SUBSTRAIT_SCALAR_TEST';
SubstraitAggregateTest: 'SUBSTRAIT_AGGREGATE_TEST';
SubstraitInclude: 'SUBSTRAIT_INCLUDE';

FormatVersion
    : 'v' DIGIT+ ('.' DIGIT+)?
    ;

DescriptionLine
    : '# ' ~[\r\n]* '\r'? '\n'
    ;

Define: 'DEFINE';
ErrorResult: '<!ERROR>';
UndefineResult: '<!UNDEFINED>';
Overflow: 'OVERFLOW';
Rounding: 'ROUNDING';
Error: 'ERROR';
Saturate: 'SATURATE';
Silent: 'SILENT';
TieToEven: 'TIE_TO_EVEN';
NaN: 'NAN';
AcceptNulls: 'ACCEPT_NULLS';
IgnoreNulls: 'IGNORE_NULLS';
NullHandling: 'NULL_HANDLING';
SpacesOnly: 'SPACES_ONLY';
Truncate: 'TRUNCATE';

IntegerLiteral
    : [+-]? Int
    ;

DecimalLiteral
    : [+-]? [0-9]+ ('.' [0-9]+)?
    ;

FloatLiteral
    : [+-]? [0-9]+ ('.' [0-9]*)? ( 'E' [+-]? [0-9]+ )?
    | [+-]? 'inf'
    | 'snan'
    ;

BooleanLiteral
    : 'true' | 'false'
    ;

fragment FourDigits: [0-9][0-9][0-9][0-9];
fragment TwoDigits: [0-9][0-9];

TimestampTzLiteral
    : '\'' FourDigits '-' TwoDigits '-' TwoDigits 'T' TwoDigits ':' TwoDigits ':' TwoDigits ( '.' [0-9]+ )?
        [+-] TwoDigits ':' TwoDigits '\''
    ;

TimestampLiteral
    : '\'' FourDigits '-' TwoDigits '-' TwoDigits 'T' TwoDigits ':' TwoDigits ':' TwoDigits ( '.' [0-9]+ )? '\''
    ;

TimeLiteral
    : '\'' TwoDigits ':' TwoDigits ':' TwoDigits ( '.' [0-9]+ )? '\''
    ;

DateLiteral
    : '\'' FourDigits '-' TwoDigits '-' TwoDigits '\''
    ;

PeriodPrefix: 'P';
TimePrefix: 'T';
YearPrefix: 'Y';
MSuffix: 'M';  // used for both months and minutes
DaySuffix: 'D';
HourSuffix: 'H';
SecondSuffix: 'S';
FractionalSecondSuffix: 'F';
OAngleBracket: Lt;
CAngleBracket: Gt;

IntervalYearLiteral
    : '\'' PeriodPrefix IntegerLiteral YearPrefix (IntegerLiteral MSuffix)? '\''
    | '\'' PeriodPrefix IntegerLiteral MSuffix '\''
    ;

IntervalDayLiteral
    : '\'' PeriodPrefix IntegerLiteral DaySuffix (TimePrefix TimeInterval)? '\''
    | '\'' PeriodPrefix TimePrefix TimeInterval '\''
    ;

fragment TimeInterval
    : IntegerLiteral HourSuffix (IntegerLiteral MSuffix)? (DecimalLiteral SecondSuffix)?
    | IntegerLiteral MSuffix (DecimalLiteral SecondSuffix)?
    | DecimalLiteral SecondSuffix
    ;

NullLiteral: 'null';

StringLiteral
    : '\'' ('\\' . | '\'\'' | ~['\\])* '\''
    ;

ColumnName
    : 'COL' Int
    ;
