lexer grammar SubstraitLexer;

// Whitespace and comment handling
LineComment   : '//' ~[\r\n]* -> channel(HIDDEN) ;
BlockComment  : ( '/*' ( ~'*' | '*'+ ~[*/] ) '*'* '*/' ) -> channel(HIDDEN) ;
Whitespace    : [ \t\r\n]+ -> channel(HIDDEN) ;

// Substrait is case-insensitive, ANTLR is not. So, in order to define our
// keywords in a somewhat readable way, we have to define these shortcuts.

fragment A : [aA];
fragment B : [bB];
fragment C : [cC];
fragment D : [dD];
fragment E : [eE];
fragment F : [fF];
fragment G : [gG];
fragment H : [hH];
fragment I : [iI];
fragment J : [jJ];
fragment K : [kK];
fragment L : [lL];
fragment M : [mM];
fragment N : [nN];
fragment O : [oO];
fragment P : [pP];
fragment Q : [qQ];
fragment R : [rR];
fragment S : [sS];
fragment T : [tT];
fragment U : [uU];
fragment V : [vV];
fragment W : [wW];
fragment X : [xX];
fragment Y : [yY];
fragment Z : [zZ];

fragment DIGIT: [0-9];

fragment INTEGER
  : '0'
  | [1-9] [0-9]*
  ;

// Syntactic keywords.
If       : I F;
Then     : T H E N;
Else     : E L S E;

// TYPES
Boolean  : B O O L E A N;
I8       : I '8';
I16      : I '16';
I32      : I '32';
I64      : I '64';
FP32     : F P '32';
FP64     : F P '64';
String   : S T R I N G;
Binary   : B I N A R Y;
Timestamp: T I M E S T A M P;
Timestamp_TZ: T I M E S T A M P '_' T Z;
Date     : D A T E;
Time     : T I M E;
Interval_Year: I N T E R V A L '_' Y E A R;
Interval_Day: I N T E R V A L '_' D A Y;
UUID     : U U I D;
Decimal  : D E C I M A L;
Precision_Timestamp: P R E C I S I O N '_' T I M E S T A M P;
Precision_Timestamp_TZ: P R E C I S I O N '_' T I M E S T A M P '_' T Z;
FixedChar: F I X E D C H A R;
VarChar  : V A R C H A R;
FixedBinary: F I X E D B I N A R Y;
Struct   : S T R U C T;
NStruct  : N S T R U C T;
List     : L I S T;
Map      : M A P;
ANY      : A N Y;
UserDefined: U '!';
Geometry: G E O M E T R Y;

// short names for types
Bool: B O O L;
Str: S T R;
VBin: V B I N;
Ts: T S;
TsTZ: T S T Z;
IYear: I Y E A R;
IDay: I D A Y;
Dec: D E C;
PTs: P T S;
PTsTZ: P T S T Z;
FChar: F C H A R;
VChar: V C H A R;
FBin: F B I N;

DOUBLE_COLON: '::';

IDENTIFIER
  : [a-zA-Z_] [a-zA-Z0-9_]*
  ;

// ORGANIZE
O_ANGLE_BRACKET: '<';
C_ANGLE_BRACKET: '>';
OPAREN: '(';
CPAREN: ')';
OBRACKET: '[';
CBRACKET: ']';
COMMA: ',';
EQ: '=';
COLON: ':';
QMARK: '?';
HASH: '#';
DOT: '.';

//STRING
//    : '\'' ('\\' . | '\'\'' | ~['\\])* '\''
//    ;

