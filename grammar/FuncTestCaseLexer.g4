lexer grammar FuncTestCaseLexer;

import SubstraitLexer;

SUBSTRAIT_SCALAR_TEST
    : '### SUBSTRAIT_SCALAR_TEST:'
    ;

FORMAT_VERSION
    : 'v' DIGIT+ ('.' DIGIT+)?
    ;

SUBSTRAIT_INCLUDE
    : '### SUBSTRAIT_INCLUDE:'
    ;

DESCRIPTION_LINE
    : '# ' ~[\r\n]* '\r'? '\n'
    ;

ERROR_RESULT
    : '<!ERROR>'
    ;

UNDEFINED_RESULT
    : '<!UNDEFINED>'
    ;


OVERFLOW: 'overlfow';
ROUNDING: 'rounding';
ERROR: 'ERROR';
SATURATE: 'SATURATE';
SILENT: 'SILENT';
TIE_TO_EVEN: 'TIE_TO_EVEN';
NAN: 'NAN';


INTEGER_LITERAL
    : [+-]? INTEGER
    ;

DECIMAL_LITERAL
    : [+-]? [0-9]+ ('.' [0-9]+)?
    ;

FLOAT_LITERAL
    : [+-]? [0-9]+ ('.' [0-9]*)? ( [eE] [+-]? [0-9]+ )?
    | [+-]? 'inf'
    | 'nan' | 'NaN'
    | 'snan'
    ;

BOOLEAN_LITERAL
    : 'true' | 'false'
    ;

fragment FourDigits: [0-9][0-9][0-9][0-9];
fragment TwoDigits: [0-9][0-9];

TIMESTAMP_TZ_LITERAL
    : '\'' FourDigits '-' TwoDigits '-' TwoDigits 'T' TwoDigits ':' TwoDigits ':' TwoDigits ( '.' [0-9]+ )?
        [+-] TwoDigits ':' TwoDigits '\''
    ;

TIMESTAMP_LITERAL
    : '\'' FourDigits '-' TwoDigits '-' TwoDigits 'T' TwoDigits ':' TwoDigits ':' TwoDigits ( '.' [0-9]+ )? '\''
    ;

TIME_LITERAL
    : '\'' TwoDigits ':' TwoDigits ':' TwoDigits ( '.' [0-9]+ )? '\''
    ;

DATE_LITERAL
    : '\'' FourDigits '-' TwoDigits '-' TwoDigits '\''
    ;

PERIOD_PREFIX: 'P';
TIME_PREFIX: 'T';
YEAR_SUFFIX: 'Y';
M_SUFFIX: 'M';  // used for both months and minutes
DAY_SUFFIX: 'D';
HOUR_SUFFIX: 'H';
SECOND_SUFFIX: 'S';
FRACTIONAL_SECOND_SUFFIX: 'F';

INTERVAL_YEAR_LITERAL
    : '\'' PERIOD_PREFIX INTEGER_LITERAL YEAR_SUFFIX (INTEGER_LITERAL M_SUFFIX)? '\''
    | '\'' PERIOD_PREFIX INTEGER_LITERAL M_SUFFIX '\''
    ;

INTERVAL_DAY_LITERAL
    : '\'' PERIOD_PREFIX INTEGER_LITERAL DAY_SUFFIX (TIME_PREFIX TIME_INTERVAL)? '\''
    | '\'' PERIOD_PREFIX TIME_PREFIX TIME_INTERVAL '\''
    ;

fragment TIME_INTERVAL
    : INTEGER_LITERAL HOUR_SUFFIX (INTEGER_LITERAL M_SUFFIX)? (INTEGER_LITERAL SECOND_SUFFIX)?
        (INTEGER_LITERAL FRACTIONAL_SECOND_SUFFIX)?
    | INTEGER_LITERAL M_SUFFIX (INTEGER_LITERAL SECOND_SUFFIX)? (INTEGER_LITERAL FRACTIONAL_SECOND_SUFFIX)?
    | INTEGER_LITERAL SECOND_SUFFIX (INTEGER_LITERAL FRACTIONAL_SECOND_SUFFIX)?
    | INTEGER_LITERAL FRACTIONAL_SECOND_SUFFIX
    ;

NULL_LITERAL: 'null';

STRING_LITERAL
    : '\'' ('\\' . | '\'\'' | ~['\\])* '\''
    ;
