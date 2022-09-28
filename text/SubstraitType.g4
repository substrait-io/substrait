grammar SubstraitType;

// Note: this grammar is intentionally written to avoid ANTLR-specific features
// that someone who hasn't used ANTLR before might not know about, including
// explicitly avoiding left recursion, such that it can easily be ported to
// other parser generators if necessary. In this way, it hopefully doubles as a
// human-readable specification for this DSL.
//
// This comes at the cost of not generating very nice parse trees. You can use
// this grammar with ANTLR directly if you want, but you might want to rewrite
// it if you intend to use the listener or generated AST directly.
//
// Some things that you will need to know if you've never seen ANTLR before:
//  - ANTLR distinguishes between tokenizer rules and parser rules by
//    capitalization of the rule name: if the first letter is uppercase, the
//    rule is a token rule; if it is lowercase, it is a parser rule. Yuck.
//  - When multiple token rules match:
//     - choose the token that matches the most text;
//     - if same length, use the one defined earlier.
//    (ANTLR supports implicit tokens as well, but we don't use them)
//  - Parse conflicts are solved using PEG rules. That is, for alternations,
//    the first alternative that matches the input is used. For ?, *, and +,
//    matching is greedy.
//  - The ~ symbol is used to negate character sets, as opposed to the [^...]
//    syntax from regular expressions.


//=============================================================================
// Whitespace and comment tokens
//=============================================================================

// Whitespace and comment handling. You can use C-style line and block
// comments.
LineComment   : '//' ~[\r\n]* -> channel(HIDDEN) ;
BlockComment  : ( '/*' ( ~'*' | '*'+ ~[*/] ) '*'* '*/' ) -> channel(HIDDEN) ;
Whitespace    : [ \t]+ -> channel(HIDDEN) ;

// Type derivations are newline-sensitive, so they're not ignored.
Newline       : [\r\n]+ ;

// Newlines can be embedded by escaping the newline character itself with a
// backslash.
EscNewline    : '\\' [\r\n]+ -> channel(HIDDEN) ;


//=============================================================================
// Keyword tokens
//=============================================================================

// Substrait is case-insensitive, ANTLR is not. So, in order to define our
// keywords in a somewhat readable way, we have to define these shortcuts.
// If you've never seen ANTLR before, fragment rules are pretty much just
// glorified preprocessor/search-and-replace macros.
fragment A : [aA]; fragment B : [bB]; fragment C : [cC]; fragment D : [dD];
fragment E : [eE]; fragment F : [fF]; fragment G : [gG]; fragment H : [hH];
fragment I : [iI]; fragment J : [jJ]; fragment K : [kK]; fragment L : [lL];
fragment M : [mM]; fragment N : [nN]; fragment O : [oO]; fragment P : [pP];
fragment Q : [qQ]; fragment R : [rR]; fragment S : [sS]; fragment T : [tT];
fragment U : [uU]; fragment V : [vV]; fragment W : [wW]; fragment X : [xX];
fragment Y : [yY]; fragment Z : [zZ];

// Syntactic keywords.
Assert    : A S S E R T ;
Matches   : M A T C H E S ;
If        : I F ;
Then      : T H E N ;
Else      : E L S E ;

// Named literal values.
Null      : N U L L ;
True      : T R U E ;
False     : F A L S E ;

// Metatype identification keywords.
Metabool  : M E T A B O O L ;
Metaint   : M E T A I N T ;
Metaenum  : M E T A E N U M ;
Metastr   : M E T A S T R ;
Typename  : T Y P E N A M E ;

// Note that data type classes are not keywords. We support user-defined type
// classes anyway, so name resolution has to be done after parsing anyway.


//=============================================================================
// Symbol tokens
//=============================================================================

// Symbols used.
Period          : '.' ;   // identifier paths
Comma           : ',' ;   // separator for pattern lists
Colon           : ':' ;   // separator for named parameters
Semicolon       : ';' ;   // separator for statements
Question        : '?' ;   // any, inconsistent bindings & nullable type suffix
Bang            : '!' ;   // boolean NOT & explicitly non-nullable type suffix
OpenParen       : '(' ;   // precedence override & function call args (open)
CloseParen      : ')' ;   // precedence override & function call args (close)
OpenCurly       : '{' ;   // enum set patterns (open)
CloseCurly      : '}' ;   // enum set patterns (close)
OpenSquare      : '[' ;   // data type variation suffix (open)
CloseSquare     : ']' ;   // data type variation suffix (close)
Assign          : '=' ;   // assignment statements
BooleanOr       : '||' ;  // boolean OR expression
BooleanAnd      : '&&' ;  // boolean AND expression
Equal           : '==' ;  // equality expression
NotEqual        : '!=' ;  // not-equals expression
LessThan        : '<' ;   // less-than expression & data type parameter pack
LessEqual       : '<=' ;  // less-equal expression
GreaterThan     : '>' ;   // greater-than expression & data type parameter pack
GreaterEqual    : '>=' ;  // greater-equal expression
Plus            : '+' ;   // additions and integer literal sign
Minus           : '-' ;   // subtractions, negation, and integer literal sign
Multiply        : '*' ;   // multiplication expression
Divide          : '/' ;   // division expression
Range           : '..' ;  // integer set patterns


//=============================================================================
// Procedurally-matched tokens
//=============================================================================

// Tokens for integer literals.
Nonzero         : [1-9] [0-9]* ;
Zero            : '0' ;

// String literal token.
String          : '"' ~["] '"' ;

// Identifier token. Note that $ signs are legal in identifiers, and note that
// all identifier matching is case-insensitive. Note also that keywords take
// precedence.
Identifier      : [a-zA-Z_$] [a-zA-Z0-9_$]* ;


//=============================================================================
// Grammar rules
//=============================================================================

// Most things in the simple extension YAMLs that refer to a type are parsed
// using patterns; patterns can both matched and evaluated (not ALL patterns
// can do both, but there is considerable overlap between the two classes,
// so they were conceptually merged). When a type needs to be derived based on
// a number of given metavalues, such as the data types of arguments passed to
// a function, a derivation program is used. Syntactically, the only difference
// is that programs can include a set of statements before the final pattern.
// Newlines can optionally go before or after a type derivation pattern or
// program without affecting syntax.
startPattern : Whitespace* Newline* pattern Newline* EOF ;
startProgram : Whitespace* Newline* program Newline* EOF ;

// A type derivation program consists of zero or more statements followed by
// the final pattern that should evaluate to the derived data type.
program : ( statement statementSeparator )* pattern ;

// Statements are separated from each other and from the final derivation
// expression using newlines or a semicolon.
statementSeparator : Newline* ( Newline | Semicolon Newline* ) ;

// Statements manipulate the state of the type derivation interpreter before
// the final derivation expression is evaluated. They look like assignment
// statements at first glance, but act more like equality or set containment
// assertions: the right-hand side is evaluated like an expression as you
// might expect, but the left-hand side acts just like the patterns that are
// used to match function argument types. While this is perhaps not the most
// intuitive ruleset, it is extremely easy to implement (it only reuses
// features we already needed anyway), while also being a much more powerful
// primitive than a simple assignment statement, because it can also be used
// for bounds checking and other assertions. For example, if we have a
// function like `fn(VARCHAR(a), VARCHAR(b))` and the implementation of the
// function requires that a + b equals 10, we can simply write "10 = a + b".
// This works, because the pattern "10" will only match the value 10, and
// a pattern mismatch at any point during the matching and evaluation process
// indicates that the implementation is incompatible with the given argument
// types. If you find this syntax confusing, you may also write
// "assert a + b matches 10" or "assert a + b == 10"; the former does the
// exact same thing, while the latter reduces to "true = a + b == 10", which is
// functionally the same thing.
//
// Note that when you use these statements like assignment statements, you can
// only ever reassign a binding to the same value. For example, "a = 10; a = 20"
// will always fail, because a cannot both be 10 and 20 at the same time (more
// accurately, a is bound to 10, so the second statement behaves like
// "10 = 20", and 20 does not match 10).
statement
  : pattern Assign pattern #Normal
  | Assert pattern Matches pattern #Match
  | Assert pattern #Assert
  ;

// Patterns are at the core of the type derivation interpreter; they are used
// both for matching and as expressions. However, note that not all types of
// patterns work in both contexts.
pattern : patternOr ;

// Lazily-evaluated boolean OR expression. Maps to builtin or() function if
// more than one pattern is parsed.
patternOr : patternAnd ( operatorOr patternAnd )* ;
operatorOr : BooleanOr #Or ;

// Lazily-evaluated boolean AND expression. Maps to builtin and() function if
// more than one pattern is parsed.
patternAnd : patternEqNeq ( operatorAnd patternEqNeq )* ;
operatorAnd : BooleanAnd #And ;

// Equality and not-equality expressions. These map to the builtin equal()
// and not_equal() functions in left-to-right order.
patternEqNeq : patternIneq ( operatorEqNeq patternIneq )* ;
operatorEqNeq : Equal #Eq | NotEqual #Neq ;

// Integer inequality expressions. These map to the builtin greater_than(),
// less_than(), greater_equal(), and less_equal() functions in left-to-right
// order.
patternIneq : patternAddSub ( operatorIneq patternAddSub )* ;
operatorIneq : LessThan #Lt | LessEqual #Le | GreaterThan #Gt | GreaterEqual #Ge ;

// Integer addition and subtraction. These map to the builtin add() and
// subtract() functions in left-to-right order.
patternAddSub : patternMulDiv ( operatorAddSub patternMulDiv )* ;
operatorAddSub : Plus #Add | Minus #Sub ;

// Integer multiplication and division. These map to the builtin multiply() and
// divide() functions in left-to-right order.
patternMulDiv : patternMisc ( operatorMulDiv patternMisc )* ;
operatorMulDiv : Multiply #Mul | Divide #Div ;

// Miscellaneous patterns that don't need special rules for precedence or
// avoiding left-recursion.
patternMisc

  // Parentheses for overriding operator precedence.
  : OpenParen pattern CloseParen #parentheses

  // If-then-else pattern. Can only be evaluated. The first pattern must
  // evaluate to a boolean. The second or third pattern is then evaluated
  // based on that boolean and returned. The branch that is not selected is
  // also not evaluated (i.e. evaluation is lazy).
  | If pattern Then pattern Else pattern #ifThenElse

  // Unary not function. Can only be evaluated and can only be applied to
  // booleans.
  | Bang pattern #unaryNot

  // The "anything" pattern. This matches everything, and cannot be evaluated.
  // It's primarily intended for matching (parts of) argument types, when you
  // don't need or want a binding. For example, `equals(?, ?) -> boolean` would
  // allow for any combination of argument types. This distinguishes it from
  // `equals(any1, any1) -> boolean`, which only accepts equal types; instead
  // it behaves like `equals(any1, any2) -> boolean`. `?` is especially useful
  // when you want this type of behavior for a variadic function; for example,
  // `serialize(?...) -> binary` will match any number and combination of
  // argument types, while `serialize(any1...) -> binary` would only accept any
  // number of any *one* data type.
  | Question #any

  // Matches any boolean value. Cannot be evaluated.
  | Metabool #boolAny

  // Matches and evaluates to the boolean value "true".
  | True #boolTrue

  // Matches and evaluates to the boolean value "false".
  | False #boolFalse

  // Matches any integer value. Cannot be evaluated.
  | Metaint #intAny

  // Matches any integer value within the specified inclusive range. Can only
  // be evaluated if the two bounds are equal, in which case it reduces to just
  // a single integer.
  | integer Range integer #intRange

  // Matches any integer value that equals at least the given number. Cannot be
  // evaluated.
  | integer Range #intAtLeast

  // Matches any integer value that equals at most the given number. Cannot be
  // evaluated.
  | Range integer #intAtMost

  // Matches and evaluates to exactly the given integer.
  | integer #intExactly

  // Matches any enumeration constant.
  | Metaenum #enumAny

  // Matches an enumeration constant in the given set. If only a single
  // constant is specified, the pattern evaluates to that constant, otherwise
  // it cannot be evaluated.
  | OpenCurly Identifier (Comma Identifier)* CloseCurly #enumSet

  // Matches any string.
  | Metastr #strAny

  // Matches and evaluates to exactly the given string.
  | String #strExactly

  // Matches any typename for which the nullability matches the nullability
  // suffix. Use `typename??` for either nullability.
  | Typename nullability? #dtAny

  // Evaluates a function. When a function is used in match context, the
  // function (and its arguments) will be *evaluated* instead, and the incoming
  // value is matched against the result. This means that it is legal to define
  // a function like f(VARCHAR(x), VARCHAR(y), VARCHAR(x + y)) because the x
  // and y bindings are captured before x + y is evaluated, but it is NOT legal
  // to define it like f(VARCHAR(x + y), VARCHAR(x), VARCHAR(y)) because x and
  // y are not yet bound when x + y is evaluated.
  // f(VARCHAR(x), VARCHAR(x + y), VARCHAR(y)) is also NOT legal, again because
  // some of the function bindings have not yet been captured, even though
  // mathematically this could be rewritten from x + y <- input to
  // y <= input - x (the evaluator is not smart enough for this, and this
  // rewriting cannot be generalized over all functions).
  //
  // The following functions are currently available:
  //
  //  - "not(metabool) -> metabool": boolean NOT.
  //  - "and(metabool*) -> metabool": boolean AND. Evaluated lazily from left
  //    to right.
  //  - "or(metabool*) -> metabool": boolean OR. Evaluated lazily from left to
  //    right.
  //  - "negate(metaint) -> metaint": integer negation. 64-bit two's complement
  //    overflow must be detected, and implies that the function implementation
  //    that the program belongs to does not match the given argument types.
  //  - "add(metaint*) -> metaint": integer sum. Overflow handled as above.
  //  - "subtract(metaint, metaint) -> metaint": subtracts an integer from
  //    another. Overflow handled as above.
  //  - "multiply(metaint*) -> metaint": integer product. Overflow handled as
  //    above.
  //  - "divide(metaint, metaint) -> metaint": divides an integer over
  //    another. Overflow and division by zero handled as above.
  //  - "min(metaint+) -> metaint": return the minimum integer value.
  //  - "max(metaint+) -> metaint": return the maximum integer value.
  //  - "equal(T, T) -> metabool": return whether the two values are equal.
  //  - "not_equal(T, T) -> metabool": return whether the two values are not
  //    equal.
  //  - "greater_than(metaint, metaint) -> metabool": return whether the left
  //    integer is greater than the right.
  //  - "less_than(metaint, metaint) -> metabool": return whether the left
  //    integer is less than the right.
  //  - "greater_equal(metaint, metaint) -> metabool": return whether the left
  //    integer is greater than or equal to the right.
  //  - "less_equal(metaint, metaint) -> metabool": return whether the left
  //    integer is less than or equal to the right.
  //  - "covers(value, pattern) -> metabool": return whether the left value
  //    matches the pattern. The pattern may make use of bindings that were
  //    previously defined. New bindings are captured if and only if covers
  //    returns true. This allows for patterns like
  //      assert if covers(x, struct<a>) then a < 10 \
  //        else if covers(x, struct<a, b>) then a + b < 10 \
  //        else false;
  //    to be written and work as expected.
  //  - "if_then_else(metabool, T, T) -> T": if-then-else expression. Evaluated
  //    lazily.
  //
  // Note that many of the functions also have corresponding expressions. These
  // expressions are simply syntactic sugar for calling the functions directly.
  | Identifier OpenParen ( pattern (Comma pattern)* )? CloseParen #function

  // This pattern matches one of three things, which are too context-sensitive
  // to distinguish at this time:
  //
  //  - a data type pattern;
  //  - an enum constant;
  //  - a normal binding; or
  //  - a binding with nullability override.
  //
  // The type depends on the identifier path, and must be disambiguated as
  // follows during name resolution:
  //
  //  - Keep track of a case-insensitive mapping from name to binding, enum
  //    constant, or type class while analyzing the parse tree. It will be
  //    empty initially.
  //  - Whenever this pattern appears, resolve the name using this mapping:
  //     - If resolution fails, resolve the name as a type class instead (it
  //       could be the name of a builtin type class, a type class defined
  //       in the current extension, or a type class defined in a dependency
  //       if appropriately prefixed with the dependency namespace):
  //        - If this succeeds, add an entry to the name mapping, mapping the
  //          incoming identifier path to the type class. If the type class is
  //          user-defined, and the type class has enum parameter slots, also
  //          add entries to the name mapping for all the enum variants; if a
  //          name was already defined, do NOT update the mapping. Finally,
  //          disambiguate the pattern as a data type pattern.
  //        - If this fails and the identifier path consists of only a single
  //          element, map the incoming identifier path to a binding, and
  //          disambiguate the pattern as a normal binding or a binding
  //          with nullability override, depending on the presence of the
  //          nullability field.
  //        - If the above fails and the identifier path consists of multiple
  //          elements, analysis should fail.
  //     - If resolution yields a binding, disambiguate the pattern as a
  //       normal binding or a binding with nullability override, depending on
  //       the presence of the nullability field.
  //     - If resolution yields an enum constant, disambiguate the pattern as
  //       an enum constant.
  //     - If resolution yields a type class, disambiguate the pattern as a
  //       data type pattern.
  //
  // If the optional nullability, variation, or parameters fields are non-empty
  // when they can't be according to the rules of the disambiguated pattern
  // type, analysis should fail.
  //
  // Note that the `!` suffix disambiguates between a normal binding and a
  // binding with a non-nullable nullability override. For a data type pattern,
  // non-nullable is the default, so something like `i32` is exactly the same
  // as `i32!`.
  //
  // The behavior for the resolved pattern types is:
  //  - Data type pattern:
  //     - Matches a metavalue if and only if:
  //        - the metavalue is a typename;
  //        - the type class matches the identified class;
  //        - the nullability of the type matches the rules detailed in the
  //          comments of the nullability rule;
  //        - the variation of the type matches the rules detailed in the
  //          comments of the variation rule; and
  //        - the parameter pack matches the rules detailed in the comments
  //          of the parameters rule.
  //     - Evaluates to a data type with the specified type class and the
  //       evaluation result of the nullability, variation, and parameters
  //       fields. If any of those things cannot be evaluated, the data type
  //       pattern cannot be evaluated. If any parameter pack constraint
  //       violations result from this, they are treated as pattern match
  //       failures (i.e., if this happens in a return type derivation of
  //       a function, the function is said to not match the given arguments).
  //
  //  - Enum constant:
  //     - Matches a metavalue if and only if it is exactly the specified enum
  //       variant.
  //     - Evaluates to the specified enum variant.
  //     - The nullability, variation, and parameters fields are illegal and
  //       must be blank.
  //
  //  - Consistent binding without nullability suffix:
  //     - If this is the first use of the name, matches non-typename
  //       metavalues and non-nullable typenames. The incoming metavalue is
  //       bound to the name as a side effect.
  //     - If the name was previously bound, matches only if the incoming
  //       metavalue is exactly equal to the previous binding.
  //     - Can only be evaluated if the name was previously bound, in which
  //       case it yields the bound value exactly.
  //     - The variation and parameter pack fields are illegal and must be
  //       blank.
  //
  //  - Consistent binding with nullability suffix:
  //     - If this is the first use of the name, matches if and only if:
  //        - the incoming metavalue is a typename; and
  //        - the nullability of the incoming type matches the nullability
  //          suffix.
  //       If the above rules match, the incoming typename, with its
  //       nullability overridden to non-nullable, is bound to the name as a
  //       side effect.
  //     - If the name was previously bound, matches if and only if:
  //        - the incoming metavalue is a typename;
  //        - the nullability of the incoming type matches the nullability
  //          suffix;
  //        - the previously bound metavalue is a typename; and
  //        - the incoming type matches the previously bound type, ignoring
  //          nullability and parameter names.
  //     - Can only be evaluated if the name was previously bound. If the
  //       previously bound metavalue is not a typename, evaluation fails. The
  //       returned type is the previously bound type, with its nullability
  //       adjusted according to the nullability suffix evaluation rules.
  //     - The variation and parameters fields are illegal and must be blank.
  | identifierPath nullability? variation? parameters? #datatypeBindingOrConstant

  // Pattern for inconsistent bindings. Inconsistent bindings are variations
  // of normal bindings and bindings with nullability override with looser
  // matching and extended evaluation rules. These rules are designed
  // specifically for matching inconsistent variadic arguments and for
  // modelling MIRROR nullability behavior. Specifically:
  //
  //  - Use `?T` instead of `T` for a variadic argument slot to capture the
  //    value of the first argument and ignore the rest, thus rendering it
  //    inconsistent.
  //  - Use `type??nullable` instead of `type` for argument slots and the
  //    return type to match both nullable and non-nullable data types for
  //    the argument, and yield a nullable return type only if any of the
  //    bound arguments are nullable.
  //
  // The exacty behavior for the pattern types is as follows. Rules that differ
  // from the consistent binding rules are highlighted with (!).
  //
  //  - Inconsistent binding without nullability suffix:
  //     - If this is the first use of the name, matches non-typename
  //       metavalues and non-nullable typenames. The incoming metavalue is
  //       bound to the name as a side effect.
  //     - (!) If the name was previously bound, still matches all
  //       non-typename metavalues and non-nullable typenames. If the
  //       incoming metavalue is boolean `true`, and the currently bound
  //       metavalue is boolean `false`, rebind the name to `true` as a side
  //       effect. Otherwise, leave it unchanged.
  //     - (!) If this is the first use of the name, evaluation yields
  //       the metabool `false` (for the nullability of the return type in
  //       a MIRROR function).
  //     - If the name was previously bound, evaluation yields the bound
  //       value exactly.
  //
  //  - Inconsistent binding with nullability override:
  //     - If this is the first use of the name, matches if and only if:
  //        - the incoming metavalue is a typename; and
  //        - the nullability of the incoming type matches the nullability
  //          field.
  //       If the above rules match, the incoming typename, with its
  //       nullability overridden to non-nullable, is bound to the name as a
  //       side effect.
  //     - (!) If the binding was previously bound, matches if and only if:
  //        - the incoming metavalue is a typename; and
  //        - the nullability of the incoming type matches the nullability
  //          field.
  //       There are no side effects in this case.
  //     - Can only be evaluated if the name was previously bound. If the
  //       previously bound metavalue is not a typename, evaluation fails. The
  //       returned type is the previously bound type, with its nullability
  //       adjusted according to the nullability field evaluation rules.
  | Question Identifier nullability? #inconsistent

  // Unary negation function. Can only be evaluated and can only be applied to
  // integers. Note that this is all the way at the back because signed integer
  // literals should be preferred, since those can also be matched, and can
  // deal with -2^63 without overflow.
  | Minus pattern #unaryNegate
  ;

// Nullability suffix.
//
//  - If there is no such suffix, or the suffix is "!", the pattern matches
//    only non-nullable types, and also evaluates to a non-nullable type if
//    applicable. The "!" suffix changes the semantics of bindings slightly
//    compared to no suffix (specifically, `x!` only matches non-nullable
//    typenames, but `x` also matches non-typename metavalues), but is
//    otherwise optional and customarily not written.
//  - If this suffix is just "?", the pattern matches only nullable types,
//    and also evaluates to a nullable type if applicable.
//  - If this suffix is a "?" followed by a pattern, the pattern is matched
//    against false for non-nullable and true for nullable types. Likewise for
//    evaluation; if the pattern evaluates to false the type will be
//    non-nullable, if it evaluates to true it will be nullable.
nullability
  : Bang #nonNullable
  | Question #nullable
  | Question pattern #nullableIf
  ;

// Type variation suffix.
//
//  - If there is no such suffix, the pattern matches any variation that is
//    marked as compatible with the system-preferred variation via the function
//    behavior option of the variation, as well as the system-preferred
//    variation itself. It will evaluate to the system-preferred variation.
//  - If the suffix is [?], the pattern matches any variation, and cannot be
//    evaluated.
//  - If the suffix is [0], the pattern matches and evaluates to the
//    system-preferred variation exactly.
//  - If the suffix is [ident], the pattern matches and evaluates to the named
//    variation exactly. The variation must be in scope.
variation : OpenSquare variationBody CloseSquare ;
variationBody
  : Question #varAny
  | Zero #varSystemPreferred
  | identifierPath #varUserDefined
  ;

// Type parameter pack suffix.
//
//  - If there is no such suffix, the pattern accepts any number of parameters
//    for the type (assuming that the type class accepts this as well), and
//    will attempt to evaluate to a type with no parameters.
//  - If there is a "<>" suffix, the pattern accepts only types with zero
//    parameters, and will attempt to evaluate to a type with no parameters.
//  - If parameters are specified, the pattern accepts only types with exactly
//    the specified number of parameters, and will attempt to evaluate to a
//    type with exactly those parameters.
parameters : LessThan ( parameter (Comma parameter)* )? GreaterThan ;

// Type parameter pattern. The name prefix is only used when evaluated (it is
// never matched), and is currently only accepted by the NSTRUCT (pseudo)type.
parameter : ( identifierOrString Colon )? parameterValue ;

// A pattern for matching potentially-optional parameter values. "null" may be
// used to match or evaluate to explicitly-skipped optional parameters;
// otherwise, the given pattern is used for the parameter value. The "?" (any)
// pattern is special-cased to also match explicitly-skipped parameter slots.
parameterValue : Null #Null | pattern #Specified;

// Integer literals.
integer : ( Plus | Minus )? ( Zero | Nonzero ) ;

// When identifying user-defined types and variations, period-separated
// namespace paths are supported.
identifierPath : ( Identifier Period )* Identifier ;

// The names of parameters (i.e. NSTRUCT field names) can be specified using
// both identifiers and strings. The latter is idiomatic only when the field
// name is not a valid Substrait identifier.
identifierOrString : String #Str | Identifier #Ident ;
