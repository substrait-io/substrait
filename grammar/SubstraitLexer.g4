lexer grammar SubstraitLexer;

options {
    caseInsensitive = true;
}

// Whitespace and comment handling
LineComment   : '//' ~[\r\n]* -> channel(HIDDEN) ;
BlockComment  : ( '/*' ( ~'*' | '*'+ ~[*/] ) '*'* '*/' ) -> channel(HIDDEN) ;
Whitespace    : [ \t\r]+ -> channel(HIDDEN) ;

// Substrait is case-insensitive, ANTLR is not. So, in order to define our
// keywords in a somewhat readable way, we have to define these shortcuts.

fragment DIGIT: [0-9];

// Syntactic keywords.
If       : 'IF';
Then     : 'THEN';
Else     : 'ELSE';

// TYPES
Boolean  : 'BOOLEAN';
I8       : 'I8';
I16      : 'I16';
I32      : 'I32';
I64      : 'I64';
FP32     : 'FP32';
FP64     : 'FP64';
String   : 'STRING';
Binary   : 'BINARY';
Timestamp: 'TIMESTAMP';
Timestamp_TZ: 'TIMESTAMP_TZ';
Date     : 'DATE';
Time     : 'TIME';
Interval_Year: 'INTERVAL_YEAR';
Interval_Day: 'INTERVAL_DAY';
UUID     : 'UUID';
Decimal  : 'DECIMAL';
Precision_Timestamp: 'PRECISION_TIMESTAMP';
Precision_Timestamp_TZ: 'PRECISION_TIMESTAMP_TZ';
FixedChar: 'FIXEDCHAR';
VarChar  : 'VARCHAR';
FixedBinary: 'FIXEDBINARY';
Struct   : 'STRUCT';
NStruct  : 'NSTRUCT';
List     : 'LIST';
Map      : 'MAP';
UserDefined: 'U!';

// short names for types
Bool: 'BOOL';
Str: 'STR';
VBin: 'VBIN';
Ts: 'TS';
TsTZ: 'TSTZ';
IYear: 'IYEAR';
IDay: 'IDAY';
Dec: 'DEC';
PTs: 'PTS';
PTsTZ: 'PTSTZ';
FChar: 'FCHAR';
VChar: 'VCHAR';
FBin: 'FBIN';

Any: 'ANY';
AnyVar: Any [0-9];

DoubleColon: '::';

// MATH
Plus            : '+';
Minus           : '-';
Asterisk        : '*';
ForwardSlash    : '/';
Percent         : '%';

// COMPARE
Eq       : '=';
Ne       : '!=';
Gte      : '>=';
Lte      : '<=';
Gt       : '>';
Lt       : '<';
Bang     : '!';

// ORGANIZE
OAngleBracket: Lt;
CAngleBracket: Gt;
OParen: '(';
CParen: ')';
OBracket: '[';
CBracket: ']';
Comma: ',';
Colon: ':';
QMark: '?';
Hash: '#';
Dot: '.';


// OPERATIONS
And      : 'AND';
Or       : 'OR';
Assign   : ':=';



fragment Int
  : '1'..'9' Digit*
  | '0'
  ;

fragment Digit
  : '0'..'9'
  ;

Number
  : '-'? Int
  ;

Identifier
  : ('A'..'Z' | '_' | '$') ('A'..'Z' | '_' | '$' | Digit)*
  ;

Newline
  : ( '\r' '\n'?
    | '\n'
    )
  ;
