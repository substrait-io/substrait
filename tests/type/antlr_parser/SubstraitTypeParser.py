# SPDX-License-Identifier: Apache-2.0
# Generated from SubstraitType.g4 by ANTLR 4.13.2
# encoding: utf-8
from antlr4 import *
from io import StringIO
import sys
if sys.version_info[1] > 5:
	from typing import TextIO
else:
	from typing.io import TextIO

def serializedATN():
    return [
        4,1,78,268,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,2,6,7,
        6,2,7,7,7,1,0,1,0,1,0,1,1,1,1,1,1,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,
        2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,3,2,38,8,2,1,3,1,3,3,3,42,8,3,1,3,
        1,3,1,3,1,3,1,3,1,3,3,3,50,8,3,1,3,1,3,1,3,1,3,1,3,1,3,3,3,58,8,
        3,1,3,1,3,1,3,1,3,1,3,1,3,3,3,66,8,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,
        1,3,3,3,76,8,3,1,3,1,3,1,3,1,3,1,3,1,3,3,3,84,8,3,1,3,1,3,1,3,1,
        3,1,3,1,3,3,3,92,8,3,1,3,1,3,1,3,1,3,1,3,1,3,3,3,100,8,3,1,3,1,3,
        1,3,1,3,5,3,106,8,3,10,3,12,3,109,9,3,1,3,1,3,1,3,1,3,3,3,115,8,
        3,1,3,1,3,1,3,1,3,1,3,1,3,5,3,123,8,3,10,3,12,3,126,9,3,1,3,1,3,
        1,3,1,3,3,3,132,8,3,1,3,1,3,1,3,1,3,1,3,1,3,3,3,140,8,3,1,3,1,3,
        1,3,1,3,1,3,1,3,1,3,1,3,1,3,3,3,151,8,3,1,3,1,3,1,3,1,3,5,3,157,
        8,3,10,3,12,3,160,9,3,1,3,1,3,3,3,164,8,3,3,3,166,8,3,1,4,1,4,1,
        4,3,4,171,8,4,1,5,1,5,3,5,175,8,5,1,5,1,5,3,5,179,8,5,3,5,181,8,
        5,1,6,1,6,3,6,185,8,6,1,6,1,6,3,6,189,8,6,1,7,1,7,1,7,1,7,1,7,1,
        7,1,7,1,7,1,7,4,7,200,8,7,11,7,12,7,201,1,7,1,7,1,7,1,7,4,7,208,
        8,7,11,7,12,7,209,5,7,212,8,7,10,7,12,7,215,9,7,1,7,1,7,5,7,219,
        8,7,10,7,12,7,222,9,7,1,7,1,7,1,7,1,7,3,7,228,8,7,1,7,1,7,1,7,1,
        7,1,7,5,7,235,8,7,10,7,12,7,238,9,7,3,7,240,8,7,1,7,1,7,1,7,1,7,
        1,7,1,7,1,7,1,7,1,7,1,7,3,7,252,8,7,1,7,1,7,1,7,1,7,1,7,1,7,1,7,
        1,7,1,7,5,7,263,8,7,10,7,12,7,266,9,7,1,7,0,1,14,8,0,2,4,6,8,10,
        12,14,0,1,3,0,50,53,55,60,73,74,324,0,16,1,0,0,0,2,19,1,0,0,0,4,
        37,1,0,0,0,6,165,1,0,0,0,8,170,1,0,0,0,10,180,1,0,0,0,12,188,1,0,
        0,0,14,251,1,0,0,0,16,17,3,14,7,0,17,18,5,0,0,1,18,1,1,0,0,0,19,
        20,3,12,6,0,20,21,5,0,0,1,21,3,1,0,0,0,22,38,5,7,0,0,23,38,5,8,0,
        0,24,38,5,9,0,0,25,38,5,10,0,0,26,38,5,11,0,0,27,38,5,12,0,0,28,
        38,5,13,0,0,29,38,5,14,0,0,30,38,5,15,0,0,31,38,5,16,0,0,32,38,5,
        17,0,0,33,38,5,18,0,0,34,38,5,19,0,0,35,38,5,20,0,0,36,38,5,22,0,
        0,37,22,1,0,0,0,37,23,1,0,0,0,37,24,1,0,0,0,37,25,1,0,0,0,37,26,
        1,0,0,0,37,27,1,0,0,0,37,28,1,0,0,0,37,29,1,0,0,0,37,30,1,0,0,0,
        37,31,1,0,0,0,37,32,1,0,0,0,37,33,1,0,0,0,37,34,1,0,0,0,37,35,1,
        0,0,0,37,36,1,0,0,0,38,5,1,0,0,0,39,41,5,26,0,0,40,42,5,70,0,0,41,
        40,1,0,0,0,41,42,1,0,0,0,42,43,1,0,0,0,43,44,5,60,0,0,44,45,3,8,
        4,0,45,46,5,59,0,0,46,166,1,0,0,0,47,49,5,27,0,0,48,50,5,70,0,0,
        49,48,1,0,0,0,49,50,1,0,0,0,50,51,1,0,0,0,51,52,5,60,0,0,52,53,3,
        8,4,0,53,54,5,59,0,0,54,166,1,0,0,0,55,57,5,28,0,0,56,58,5,70,0,
        0,57,56,1,0,0,0,57,58,1,0,0,0,58,59,1,0,0,0,59,60,5,60,0,0,60,61,
        3,8,4,0,61,62,5,59,0,0,62,166,1,0,0,0,63,65,5,23,0,0,64,66,5,70,
        0,0,65,64,1,0,0,0,65,66,1,0,0,0,66,67,1,0,0,0,67,68,5,60,0,0,68,
        69,3,8,4,0,69,70,5,68,0,0,70,71,3,8,4,0,71,72,5,59,0,0,72,166,1,
        0,0,0,73,75,5,21,0,0,74,76,5,70,0,0,75,74,1,0,0,0,75,76,1,0,0,0,
        76,77,1,0,0,0,77,78,5,60,0,0,78,79,3,8,4,0,79,80,5,59,0,0,80,166,
        1,0,0,0,81,83,5,24,0,0,82,84,5,70,0,0,83,82,1,0,0,0,83,84,1,0,0,
        0,84,85,1,0,0,0,85,86,5,60,0,0,86,87,3,8,4,0,87,88,5,59,0,0,88,166,
        1,0,0,0,89,91,5,25,0,0,90,92,5,70,0,0,91,90,1,0,0,0,91,92,1,0,0,
        0,92,93,1,0,0,0,93,94,5,60,0,0,94,95,3,8,4,0,95,96,5,59,0,0,96,166,
        1,0,0,0,97,99,5,29,0,0,98,100,5,70,0,0,99,98,1,0,0,0,99,100,1,0,
        0,0,100,101,1,0,0,0,101,102,5,60,0,0,102,107,3,14,7,0,103,104,5,
        68,0,0,104,106,3,14,7,0,105,103,1,0,0,0,106,109,1,0,0,0,107,105,
        1,0,0,0,107,108,1,0,0,0,108,110,1,0,0,0,109,107,1,0,0,0,110,111,
        5,59,0,0,111,166,1,0,0,0,112,114,5,30,0,0,113,115,5,70,0,0,114,113,
        1,0,0,0,114,115,1,0,0,0,115,116,1,0,0,0,116,117,5,60,0,0,117,118,
        5,77,0,0,118,124,3,14,7,0,119,120,5,68,0,0,120,121,5,77,0,0,121,
        123,3,14,7,0,122,119,1,0,0,0,123,126,1,0,0,0,124,122,1,0,0,0,124,
        125,1,0,0,0,125,127,1,0,0,0,126,124,1,0,0,0,127,128,5,59,0,0,128,
        166,1,0,0,0,129,131,5,31,0,0,130,132,5,70,0,0,131,130,1,0,0,0,131,
        132,1,0,0,0,132,133,1,0,0,0,133,134,5,60,0,0,134,135,3,14,7,0,135,
        136,5,59,0,0,136,166,1,0,0,0,137,139,5,32,0,0,138,140,5,70,0,0,139,
        138,1,0,0,0,139,140,1,0,0,0,140,141,1,0,0,0,141,142,5,60,0,0,142,
        143,3,14,7,0,143,144,5,68,0,0,144,145,3,14,7,0,145,146,5,59,0,0,
        146,166,1,0,0,0,147,148,5,33,0,0,148,150,5,77,0,0,149,151,5,70,0,
        0,150,149,1,0,0,0,150,151,1,0,0,0,151,163,1,0,0,0,152,153,5,60,0,
        0,153,158,3,14,7,0,154,155,5,68,0,0,155,157,3,14,7,0,156,154,1,0,
        0,0,157,160,1,0,0,0,158,156,1,0,0,0,158,159,1,0,0,0,159,161,1,0,
        0,0,160,158,1,0,0,0,161,162,5,59,0,0,162,164,1,0,0,0,163,152,1,0,
        0,0,163,164,1,0,0,0,164,166,1,0,0,0,165,39,1,0,0,0,165,47,1,0,0,
        0,165,55,1,0,0,0,165,63,1,0,0,0,165,73,1,0,0,0,165,81,1,0,0,0,165,
        89,1,0,0,0,165,97,1,0,0,0,165,112,1,0,0,0,165,129,1,0,0,0,165,137,
        1,0,0,0,165,147,1,0,0,0,166,7,1,0,0,0,167,171,5,76,0,0,168,171,5,
        77,0,0,169,171,3,14,7,0,170,167,1,0,0,0,170,168,1,0,0,0,170,169,
        1,0,0,0,171,9,1,0,0,0,172,174,5,47,0,0,173,175,5,70,0,0,174,173,
        1,0,0,0,174,175,1,0,0,0,175,181,1,0,0,0,176,178,5,48,0,0,177,179,
        5,70,0,0,178,177,1,0,0,0,178,179,1,0,0,0,179,181,1,0,0,0,180,172,
        1,0,0,0,180,176,1,0,0,0,181,11,1,0,0,0,182,184,3,4,2,0,183,185,5,
        70,0,0,184,183,1,0,0,0,184,185,1,0,0,0,185,189,1,0,0,0,186,189,3,
        6,3,0,187,189,3,10,5,0,188,182,1,0,0,0,188,186,1,0,0,0,188,187,1,
        0,0,0,189,13,1,0,0,0,190,191,6,7,-1,0,191,192,5,64,0,0,192,193,3,
        14,7,0,193,194,5,65,0,0,194,252,1,0,0,0,195,196,5,77,0,0,196,197,
        5,55,0,0,197,199,3,14,7,0,198,200,5,78,0,0,199,198,1,0,0,0,200,201,
        1,0,0,0,201,199,1,0,0,0,201,202,1,0,0,0,202,213,1,0,0,0,203,204,
        5,77,0,0,204,205,5,55,0,0,205,207,3,14,7,0,206,208,5,78,0,0,207,
        206,1,0,0,0,208,209,1,0,0,0,209,207,1,0,0,0,209,210,1,0,0,0,210,
        212,1,0,0,0,211,203,1,0,0,0,212,215,1,0,0,0,213,211,1,0,0,0,213,
        214,1,0,0,0,214,216,1,0,0,0,215,213,1,0,0,0,216,220,3,12,6,0,217,
        219,5,78,0,0,218,217,1,0,0,0,219,222,1,0,0,0,220,218,1,0,0,0,220,
        221,1,0,0,0,221,252,1,0,0,0,222,220,1,0,0,0,223,252,3,12,6,0,224,
        252,5,76,0,0,225,227,5,77,0,0,226,228,5,70,0,0,227,226,1,0,0,0,227,
        228,1,0,0,0,228,252,1,0,0,0,229,230,5,77,0,0,230,239,5,64,0,0,231,
        236,3,14,7,0,232,233,5,68,0,0,233,235,3,14,7,0,234,232,1,0,0,0,235,
        238,1,0,0,0,236,234,1,0,0,0,236,237,1,0,0,0,237,240,1,0,0,0,238,
        236,1,0,0,0,239,231,1,0,0,0,239,240,1,0,0,0,240,241,1,0,0,0,241,
        252,5,65,0,0,242,243,5,4,0,0,243,244,3,14,7,0,244,245,5,5,0,0,245,
        246,3,14,7,0,246,247,5,6,0,0,247,248,3,14,7,3,248,252,1,0,0,0,249,
        250,5,61,0,0,250,252,3,14,7,2,251,190,1,0,0,0,251,195,1,0,0,0,251,
        223,1,0,0,0,251,224,1,0,0,0,251,225,1,0,0,0,251,229,1,0,0,0,251,
        242,1,0,0,0,251,249,1,0,0,0,252,264,1,0,0,0,253,254,10,4,0,0,254,
        255,7,0,0,0,255,263,3,14,7,5,256,257,10,1,0,0,257,258,5,70,0,0,258,
        259,3,14,7,0,259,260,5,69,0,0,260,261,3,14,7,2,261,263,1,0,0,0,262,
        253,1,0,0,0,262,256,1,0,0,0,263,266,1,0,0,0,264,262,1,0,0,0,264,
        265,1,0,0,0,265,15,1,0,0,0,266,264,1,0,0,0,34,37,41,49,57,65,75,
        83,91,99,107,114,124,131,139,150,158,163,165,170,174,178,180,184,
        188,201,209,213,220,227,236,239,251,262,264
    ]

class SubstraitTypeParser ( Parser ):

    grammarFileName = "SubstraitType.g4"

    atn = ATNDeserializer().deserialize(serializedATN())

    decisionsToDFA = [ DFA(ds, i) for i, ds in enumerate(atn.decisionToState) ]

    sharedContextCache = PredictionContextCache()

    literalNames = [ "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                     "'IF'", "'THEN'", "'ELSE'", "'BOOLEAN'", "'I8'", "'I16'", 
                     "'I32'", "'I64'", "'FP32'", "'FP64'", "'STRING'", "'BINARY'", 
                     "'TIMESTAMP'", "'TIMESTAMP_TZ'", "'DATE'", "'TIME'", 
                     "'INTERVAL_YEAR'", "'INTERVAL_DAY'", "'UUID'", "'DECIMAL'", 
                     "'PRECISION_TIMESTAMP'", "'PRECISION_TIMESTAMP_TZ'", 
                     "'FIXEDCHAR'", "'VARCHAR'", "'FIXEDBINARY'", "'STRUCT'", 
                     "'NSTRUCT'", "'LIST'", "'MAP'", "'U!'", "'BOOL'", "'STR'", 
                     "'VBIN'", "'TS'", "'TSTZ'", "'IYEAR'", "'IDAY'", "'DEC'", 
                     "'PTS'", "'PTSTZ'", "'FCHAR'", "'VCHAR'", "'FBIN'", 
                     "'ANY'", "<INVALID>", "'::'", "'+'", "'-'", "'*'", 
                     "'/'", "'%'", "'='", "'!='", "'>='", "'<='", "'>'", 
                     "'<'", "'!'", "<INVALID>", "<INVALID>", "'('", "')'", 
                     "'['", "']'", "','", "':'", "'?'", "'#'", "'.'", "'AND'", 
                     "'OR'", "':='" ]

    symbolicNames = [ "<INVALID>", "LineComment", "BlockComment", "Whitespace", 
                      "If", "Then", "Else", "Boolean", "I8", "I16", "I32", 
                      "I64", "FP32", "FP64", "String", "Binary", "Timestamp", 
                      "Timestamp_TZ", "Date", "Time", "Interval_Year", "Interval_Day", 
                      "UUID", "Decimal", "Precision_Timestamp", "Precision_Timestamp_TZ", 
                      "FixedChar", "VarChar", "FixedBinary", "Struct", "NStruct", 
                      "List", "Map", "UserDefined", "Bool", "Str", "VBin", 
                      "Ts", "TsTZ", "IYear", "IDay", "Dec", "PTs", "PTsTZ", 
                      "FChar", "VChar", "FBin", "Any", "AnyVar", "DoubleColon", 
                      "Plus", "Minus", "Asterisk", "ForwardSlash", "Percent", 
                      "Eq", "Ne", "Gte", "Lte", "Gt", "Lt", "Bang", "OAngleBracket", 
                      "CAngleBracket", "OParen", "CParen", "OBracket", "CBracket", 
                      "Comma", "Colon", "QMark", "Hash", "Dot", "And", "Or", 
                      "Assign", "Number", "Identifier", "Newline" ]

    RULE_startRule = 0
    RULE_typeStatement = 1
    RULE_scalarType = 2
    RULE_parameterizedType = 3
    RULE_numericParameter = 4
    RULE_anyType = 5
    RULE_typeDef = 6
    RULE_expr = 7

    ruleNames =  [ "startRule", "typeStatement", "scalarType", "parameterizedType", 
                   "numericParameter", "anyType", "typeDef", "expr" ]

    EOF = Token.EOF
    LineComment=1
    BlockComment=2
    Whitespace=3
    If=4
    Then=5
    Else=6
    Boolean=7
    I8=8
    I16=9
    I32=10
    I64=11
    FP32=12
    FP64=13
    String=14
    Binary=15
    Timestamp=16
    Timestamp_TZ=17
    Date=18
    Time=19
    Interval_Year=20
    Interval_Day=21
    UUID=22
    Decimal=23
    Precision_Timestamp=24
    Precision_Timestamp_TZ=25
    FixedChar=26
    VarChar=27
    FixedBinary=28
    Struct=29
    NStruct=30
    List=31
    Map=32
    UserDefined=33
    Bool=34
    Str=35
    VBin=36
    Ts=37
    TsTZ=38
    IYear=39
    IDay=40
    Dec=41
    PTs=42
    PTsTZ=43
    FChar=44
    VChar=45
    FBin=46
    Any=47
    AnyVar=48
    DoubleColon=49
    Plus=50
    Minus=51
    Asterisk=52
    ForwardSlash=53
    Percent=54
    Eq=55
    Ne=56
    Gte=57
    Lte=58
    Gt=59
    Lt=60
    Bang=61
    OAngleBracket=62
    CAngleBracket=63
    OParen=64
    CParen=65
    OBracket=66
    CBracket=67
    Comma=68
    Colon=69
    QMark=70
    Hash=71
    Dot=72
    And=73
    Or=74
    Assign=75
    Number=76
    Identifier=77
    Newline=78

    def __init__(self, input:TokenStream, output:TextIO = sys.stdout):
        super().__init__(input, output)
        self.checkVersion("4.13.2")
        self._interp = ParserATNSimulator(self, self.atn, self.decisionsToDFA, self.sharedContextCache)
        self._predicates = None




    class StartRuleContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def expr(self):
            return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,0)


        def EOF(self):
            return self.getToken(SubstraitTypeParser.EOF, 0)

        def getRuleIndex(self):
            return SubstraitTypeParser.RULE_startRule

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterStartRule" ):
                listener.enterStartRule(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitStartRule" ):
                listener.exitStartRule(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitStartRule" ):
                return visitor.visitStartRule(self)
            else:
                return visitor.visitChildren(self)




    def startRule(self):

        localctx = SubstraitTypeParser.StartRuleContext(self, self._ctx, self.state)
        self.enterRule(localctx, 0, self.RULE_startRule)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 16
            self.expr(0)
            self.state = 17
            self.match(SubstraitTypeParser.EOF)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TypeStatementContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def typeDef(self):
            return self.getTypedRuleContext(SubstraitTypeParser.TypeDefContext,0)


        def EOF(self):
            return self.getToken(SubstraitTypeParser.EOF, 0)

        def getRuleIndex(self):
            return SubstraitTypeParser.RULE_typeStatement

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTypeStatement" ):
                listener.enterTypeStatement(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTypeStatement" ):
                listener.exitTypeStatement(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTypeStatement" ):
                return visitor.visitTypeStatement(self)
            else:
                return visitor.visitChildren(self)




    def typeStatement(self):

        localctx = SubstraitTypeParser.TypeStatementContext(self, self._ctx, self.state)
        self.enterRule(localctx, 2, self.RULE_typeStatement)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 19
            self.typeDef()
            self.state = 20
            self.match(SubstraitTypeParser.EOF)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ScalarTypeContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return SubstraitTypeParser.RULE_scalarType

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)



    class DateContext(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Date(self):
            return self.getToken(SubstraitTypeParser.Date, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDate" ):
                listener.enterDate(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDate" ):
                listener.exitDate(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDate" ):
                return visitor.visitDate(self)
            else:
                return visitor.visitChildren(self)


    class StringContext(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def String(self):
            return self.getToken(SubstraitTypeParser.String, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterString" ):
                listener.enterString(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitString" ):
                listener.exitString(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitString" ):
                return visitor.visitString(self)
            else:
                return visitor.visitChildren(self)


    class I64Context(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def I64(self):
            return self.getToken(SubstraitTypeParser.I64, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterI64" ):
                listener.enterI64(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitI64" ):
                listener.exitI64(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitI64" ):
                return visitor.visitI64(self)
            else:
                return visitor.visitChildren(self)


    class I32Context(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def I32(self):
            return self.getToken(SubstraitTypeParser.I32, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterI32" ):
                listener.enterI32(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitI32" ):
                listener.exitI32(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitI32" ):
                return visitor.visitI32(self)
            else:
                return visitor.visitChildren(self)


    class IntervalYearContext(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Interval_Year(self):
            return self.getToken(SubstraitTypeParser.Interval_Year, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterIntervalYear" ):
                listener.enterIntervalYear(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitIntervalYear" ):
                listener.exitIntervalYear(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitIntervalYear" ):
                return visitor.visitIntervalYear(self)
            else:
                return visitor.visitChildren(self)


    class UuidContext(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def UUID(self):
            return self.getToken(SubstraitTypeParser.UUID, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterUuid" ):
                listener.enterUuid(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitUuid" ):
                listener.exitUuid(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitUuid" ):
                return visitor.visitUuid(self)
            else:
                return visitor.visitChildren(self)


    class I8Context(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def I8(self):
            return self.getToken(SubstraitTypeParser.I8, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterI8" ):
                listener.enterI8(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitI8" ):
                listener.exitI8(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitI8" ):
                return visitor.visitI8(self)
            else:
                return visitor.visitChildren(self)


    class I16Context(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def I16(self):
            return self.getToken(SubstraitTypeParser.I16, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterI16" ):
                listener.enterI16(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitI16" ):
                listener.exitI16(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitI16" ):
                return visitor.visitI16(self)
            else:
                return visitor.visitChildren(self)


    class BooleanContext(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Boolean(self):
            return self.getToken(SubstraitTypeParser.Boolean, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterBoolean" ):
                listener.enterBoolean(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitBoolean" ):
                listener.exitBoolean(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBoolean" ):
                return visitor.visitBoolean(self)
            else:
                return visitor.visitChildren(self)


    class BinaryContext(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Binary(self):
            return self.getToken(SubstraitTypeParser.Binary, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterBinary" ):
                listener.enterBinary(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitBinary" ):
                listener.exitBinary(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBinary" ):
                return visitor.visitBinary(self)
            else:
                return visitor.visitChildren(self)


    class Fp64Context(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def FP64(self):
            return self.getToken(SubstraitTypeParser.FP64, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFp64" ):
                listener.enterFp64(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFp64" ):
                listener.exitFp64(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFp64" ):
                return visitor.visitFp64(self)
            else:
                return visitor.visitChildren(self)


    class Fp32Context(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def FP32(self):
            return self.getToken(SubstraitTypeParser.FP32, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFp32" ):
                listener.enterFp32(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFp32" ):
                listener.exitFp32(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFp32" ):
                return visitor.visitFp32(self)
            else:
                return visitor.visitChildren(self)


    class TimeContext(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Time(self):
            return self.getToken(SubstraitTypeParser.Time, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTime" ):
                listener.enterTime(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTime" ):
                listener.exitTime(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTime" ):
                return visitor.visitTime(self)
            else:
                return visitor.visitChildren(self)


    class TimestampContext(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Timestamp(self):
            return self.getToken(SubstraitTypeParser.Timestamp, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTimestamp" ):
                listener.enterTimestamp(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTimestamp" ):
                listener.exitTimestamp(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTimestamp" ):
                return visitor.visitTimestamp(self)
            else:
                return visitor.visitChildren(self)


    class TimestampTzContext(ScalarTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ScalarTypeContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Timestamp_TZ(self):
            return self.getToken(SubstraitTypeParser.Timestamp_TZ, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTimestampTz" ):
                listener.enterTimestampTz(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTimestampTz" ):
                listener.exitTimestampTz(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTimestampTz" ):
                return visitor.visitTimestampTz(self)
            else:
                return visitor.visitChildren(self)



    def scalarType(self):

        localctx = SubstraitTypeParser.ScalarTypeContext(self, self._ctx, self.state)
        self.enterRule(localctx, 4, self.RULE_scalarType)
        try:
            self.state = 37
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [7]:
                localctx = SubstraitTypeParser.BooleanContext(self, localctx)
                self.enterOuterAlt(localctx, 1)
                self.state = 22
                self.match(SubstraitTypeParser.Boolean)
                pass
            elif token in [8]:
                localctx = SubstraitTypeParser.I8Context(self, localctx)
                self.enterOuterAlt(localctx, 2)
                self.state = 23
                self.match(SubstraitTypeParser.I8)
                pass
            elif token in [9]:
                localctx = SubstraitTypeParser.I16Context(self, localctx)
                self.enterOuterAlt(localctx, 3)
                self.state = 24
                self.match(SubstraitTypeParser.I16)
                pass
            elif token in [10]:
                localctx = SubstraitTypeParser.I32Context(self, localctx)
                self.enterOuterAlt(localctx, 4)
                self.state = 25
                self.match(SubstraitTypeParser.I32)
                pass
            elif token in [11]:
                localctx = SubstraitTypeParser.I64Context(self, localctx)
                self.enterOuterAlt(localctx, 5)
                self.state = 26
                self.match(SubstraitTypeParser.I64)
                pass
            elif token in [12]:
                localctx = SubstraitTypeParser.Fp32Context(self, localctx)
                self.enterOuterAlt(localctx, 6)
                self.state = 27
                self.match(SubstraitTypeParser.FP32)
                pass
            elif token in [13]:
                localctx = SubstraitTypeParser.Fp64Context(self, localctx)
                self.enterOuterAlt(localctx, 7)
                self.state = 28
                self.match(SubstraitTypeParser.FP64)
                pass
            elif token in [14]:
                localctx = SubstraitTypeParser.StringContext(self, localctx)
                self.enterOuterAlt(localctx, 8)
                self.state = 29
                self.match(SubstraitTypeParser.String)
                pass
            elif token in [15]:
                localctx = SubstraitTypeParser.BinaryContext(self, localctx)
                self.enterOuterAlt(localctx, 9)
                self.state = 30
                self.match(SubstraitTypeParser.Binary)
                pass
            elif token in [16]:
                localctx = SubstraitTypeParser.TimestampContext(self, localctx)
                self.enterOuterAlt(localctx, 10)
                self.state = 31
                self.match(SubstraitTypeParser.Timestamp)
                pass
            elif token in [17]:
                localctx = SubstraitTypeParser.TimestampTzContext(self, localctx)
                self.enterOuterAlt(localctx, 11)
                self.state = 32
                self.match(SubstraitTypeParser.Timestamp_TZ)
                pass
            elif token in [18]:
                localctx = SubstraitTypeParser.DateContext(self, localctx)
                self.enterOuterAlt(localctx, 12)
                self.state = 33
                self.match(SubstraitTypeParser.Date)
                pass
            elif token in [19]:
                localctx = SubstraitTypeParser.TimeContext(self, localctx)
                self.enterOuterAlt(localctx, 13)
                self.state = 34
                self.match(SubstraitTypeParser.Time)
                pass
            elif token in [20]:
                localctx = SubstraitTypeParser.IntervalYearContext(self, localctx)
                self.enterOuterAlt(localctx, 14)
                self.state = 35
                self.match(SubstraitTypeParser.Interval_Year)
                pass
            elif token in [22]:
                localctx = SubstraitTypeParser.UuidContext(self, localctx)
                self.enterOuterAlt(localctx, 15)
                self.state = 36
                self.match(SubstraitTypeParser.UUID)
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ParameterizedTypeContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return SubstraitTypeParser.RULE_parameterizedType

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)



    class StructContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.copyFrom(ctx)

        def Struct(self):
            return self.getToken(SubstraitTypeParser.Struct, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.ExprContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,i)

        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def Comma(self, i:int=None):
            if i is None:
                return self.getTokens(SubstraitTypeParser.Comma)
            else:
                return self.getToken(SubstraitTypeParser.Comma, i)
        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterStruct" ):
                listener.enterStruct(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitStruct" ):
                listener.exitStruct(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitStruct" ):
                return visitor.visitStruct(self)
            else:
                return visitor.visitChildren(self)


    class PrecisionTimestampTZContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.precision = None # NumericParameterContext
            self.copyFrom(ctx)

        def Precision_Timestamp_TZ(self):
            return self.getToken(SubstraitTypeParser.Precision_Timestamp_TZ, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def numericParameter(self):
            return self.getTypedRuleContext(SubstraitTypeParser.NumericParameterContext,0)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterPrecisionTimestampTZ" ):
                listener.enterPrecisionTimestampTZ(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitPrecisionTimestampTZ" ):
                listener.exitPrecisionTimestampTZ(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitPrecisionTimestampTZ" ):
                return visitor.visitPrecisionTimestampTZ(self)
            else:
                return visitor.visitChildren(self)


    class NStructContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.copyFrom(ctx)

        def NStruct(self):
            return self.getToken(SubstraitTypeParser.NStruct, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Identifier(self, i:int=None):
            if i is None:
                return self.getTokens(SubstraitTypeParser.Identifier)
            else:
                return self.getToken(SubstraitTypeParser.Identifier, i)
        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.ExprContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,i)

        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def Comma(self, i:int=None):
            if i is None:
                return self.getTokens(SubstraitTypeParser.Comma)
            else:
                return self.getToken(SubstraitTypeParser.Comma, i)
        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterNStruct" ):
                listener.enterNStruct(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitNStruct" ):
                listener.exitNStruct(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitNStruct" ):
                return visitor.visitNStruct(self)
            else:
                return visitor.visitChildren(self)


    class VarCharContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.length = None # NumericParameterContext
            self.copyFrom(ctx)

        def VarChar(self):
            return self.getToken(SubstraitTypeParser.VarChar, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def numericParameter(self):
            return self.getTypedRuleContext(SubstraitTypeParser.NumericParameterContext,0)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterVarChar" ):
                listener.enterVarChar(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitVarChar" ):
                listener.exitVarChar(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitVarChar" ):
                return visitor.visitVarChar(self)
            else:
                return visitor.visitChildren(self)


    class FixedBinaryContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.length = None # NumericParameterContext
            self.copyFrom(ctx)

        def FixedBinary(self):
            return self.getToken(SubstraitTypeParser.FixedBinary, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def numericParameter(self):
            return self.getTypedRuleContext(SubstraitTypeParser.NumericParameterContext,0)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFixedBinary" ):
                listener.enterFixedBinary(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFixedBinary" ):
                listener.exitFixedBinary(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFixedBinary" ):
                return visitor.visitFixedBinary(self)
            else:
                return visitor.visitChildren(self)


    class UserDefinedContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.copyFrom(ctx)

        def UserDefined(self):
            return self.getToken(SubstraitTypeParser.UserDefined, 0)
        def Identifier(self):
            return self.getToken(SubstraitTypeParser.Identifier, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.ExprContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,i)

        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)
        def Comma(self, i:int=None):
            if i is None:
                return self.getTokens(SubstraitTypeParser.Comma)
            else:
                return self.getToken(SubstraitTypeParser.Comma, i)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterUserDefined" ):
                listener.enterUserDefined(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitUserDefined" ):
                listener.exitUserDefined(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitUserDefined" ):
                return visitor.visitUserDefined(self)
            else:
                return visitor.visitChildren(self)


    class PrecisionTimestampContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.precision = None # NumericParameterContext
            self.copyFrom(ctx)

        def Precision_Timestamp(self):
            return self.getToken(SubstraitTypeParser.Precision_Timestamp, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def numericParameter(self):
            return self.getTypedRuleContext(SubstraitTypeParser.NumericParameterContext,0)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterPrecisionTimestamp" ):
                listener.enterPrecisionTimestamp(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitPrecisionTimestamp" ):
                listener.exitPrecisionTimestamp(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitPrecisionTimestamp" ):
                return visitor.visitPrecisionTimestamp(self)
            else:
                return visitor.visitChildren(self)


    class FixedCharContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.length = None # NumericParameterContext
            self.copyFrom(ctx)

        def FixedChar(self):
            return self.getToken(SubstraitTypeParser.FixedChar, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def numericParameter(self):
            return self.getTypedRuleContext(SubstraitTypeParser.NumericParameterContext,0)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFixedChar" ):
                listener.enterFixedChar(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFixedChar" ):
                listener.exitFixedChar(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFixedChar" ):
                return visitor.visitFixedChar(self)
            else:
                return visitor.visitChildren(self)


    class DecimalContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.precision = None # NumericParameterContext
            self.scale = None # NumericParameterContext
            self.copyFrom(ctx)

        def Decimal(self):
            return self.getToken(SubstraitTypeParser.Decimal, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Comma(self):
            return self.getToken(SubstraitTypeParser.Comma, 0)
        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def numericParameter(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.NumericParameterContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.NumericParameterContext,i)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterDecimal" ):
                listener.enterDecimal(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitDecimal" ):
                listener.exitDecimal(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitDecimal" ):
                return visitor.visitDecimal(self)
            else:
                return visitor.visitChildren(self)


    class ListContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.copyFrom(ctx)

        def List(self):
            return self.getToken(SubstraitTypeParser.List, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def expr(self):
            return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,0)

        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterList" ):
                listener.enterList(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitList" ):
                listener.exitList(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitList" ):
                return visitor.visitList(self)
            else:
                return visitor.visitChildren(self)


    class MapContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.key = None # ExprContext
            self.value = None # ExprContext
            self.copyFrom(ctx)

        def Map(self):
            return self.getToken(SubstraitTypeParser.Map, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Comma(self):
            return self.getToken(SubstraitTypeParser.Comma, 0)
        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.ExprContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,i)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterMap" ):
                listener.enterMap(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitMap" ):
                listener.exitMap(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitMap" ):
                return visitor.visitMap(self)
            else:
                return visitor.visitChildren(self)


    class PrecisionIntervalDayContext(ParameterizedTypeContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ParameterizedTypeContext
            super().__init__(parser)
            self.isnull = None # Token
            self.precision = None # NumericParameterContext
            self.copyFrom(ctx)

        def Interval_Day(self):
            return self.getToken(SubstraitTypeParser.Interval_Day, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def numericParameter(self):
            return self.getTypedRuleContext(SubstraitTypeParser.NumericParameterContext,0)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterPrecisionIntervalDay" ):
                listener.enterPrecisionIntervalDay(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitPrecisionIntervalDay" ):
                listener.exitPrecisionIntervalDay(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitPrecisionIntervalDay" ):
                return visitor.visitPrecisionIntervalDay(self)
            else:
                return visitor.visitChildren(self)



    def parameterizedType(self):

        localctx = SubstraitTypeParser.ParameterizedTypeContext(self, self._ctx, self.state)
        self.enterRule(localctx, 6, self.RULE_parameterizedType)
        self._la = 0 # Token type
        try:
            self.state = 165
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [26]:
                localctx = SubstraitTypeParser.FixedCharContext(self, localctx)
                self.enterOuterAlt(localctx, 1)
                self.state = 39
                self.match(SubstraitTypeParser.FixedChar)
                self.state = 41
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 40
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 43
                self.match(SubstraitTypeParser.Lt)
                self.state = 44
                localctx.length = self.numericParameter()
                self.state = 45
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [27]:
                localctx = SubstraitTypeParser.VarCharContext(self, localctx)
                self.enterOuterAlt(localctx, 2)
                self.state = 47
                self.match(SubstraitTypeParser.VarChar)
                self.state = 49
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 48
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 51
                self.match(SubstraitTypeParser.Lt)
                self.state = 52
                localctx.length = self.numericParameter()
                self.state = 53
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [28]:
                localctx = SubstraitTypeParser.FixedBinaryContext(self, localctx)
                self.enterOuterAlt(localctx, 3)
                self.state = 55
                self.match(SubstraitTypeParser.FixedBinary)
                self.state = 57
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 56
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 59
                self.match(SubstraitTypeParser.Lt)
                self.state = 60
                localctx.length = self.numericParameter()
                self.state = 61
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [23]:
                localctx = SubstraitTypeParser.DecimalContext(self, localctx)
                self.enterOuterAlt(localctx, 4)
                self.state = 63
                self.match(SubstraitTypeParser.Decimal)
                self.state = 65
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 64
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 67
                self.match(SubstraitTypeParser.Lt)
                self.state = 68
                localctx.precision = self.numericParameter()
                self.state = 69
                self.match(SubstraitTypeParser.Comma)
                self.state = 70
                localctx.scale = self.numericParameter()
                self.state = 71
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [21]:
                localctx = SubstraitTypeParser.PrecisionIntervalDayContext(self, localctx)
                self.enterOuterAlt(localctx, 5)
                self.state = 73
                self.match(SubstraitTypeParser.Interval_Day)
                self.state = 75
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 74
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 77
                self.match(SubstraitTypeParser.Lt)
                self.state = 78
                localctx.precision = self.numericParameter()
                self.state = 79
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [24]:
                localctx = SubstraitTypeParser.PrecisionTimestampContext(self, localctx)
                self.enterOuterAlt(localctx, 6)
                self.state = 81
                self.match(SubstraitTypeParser.Precision_Timestamp)
                self.state = 83
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 82
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 85
                self.match(SubstraitTypeParser.Lt)
                self.state = 86
                localctx.precision = self.numericParameter()
                self.state = 87
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [25]:
                localctx = SubstraitTypeParser.PrecisionTimestampTZContext(self, localctx)
                self.enterOuterAlt(localctx, 7)
                self.state = 89
                self.match(SubstraitTypeParser.Precision_Timestamp_TZ)
                self.state = 91
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 90
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 93
                self.match(SubstraitTypeParser.Lt)
                self.state = 94
                localctx.precision = self.numericParameter()
                self.state = 95
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [29]:
                localctx = SubstraitTypeParser.StructContext(self, localctx)
                self.enterOuterAlt(localctx, 8)
                self.state = 97
                self.match(SubstraitTypeParser.Struct)
                self.state = 99
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 98
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 101
                self.match(SubstraitTypeParser.Lt)
                self.state = 102
                self.expr(0)
                self.state = 107
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                while _la==68:
                    self.state = 103
                    self.match(SubstraitTypeParser.Comma)
                    self.state = 104
                    self.expr(0)
                    self.state = 109
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)

                self.state = 110
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [30]:
                localctx = SubstraitTypeParser.NStructContext(self, localctx)
                self.enterOuterAlt(localctx, 9)
                self.state = 112
                self.match(SubstraitTypeParser.NStruct)
                self.state = 114
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 113
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 116
                self.match(SubstraitTypeParser.Lt)
                self.state = 117
                self.match(SubstraitTypeParser.Identifier)
                self.state = 118
                self.expr(0)
                self.state = 124
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                while _la==68:
                    self.state = 119
                    self.match(SubstraitTypeParser.Comma)
                    self.state = 120
                    self.match(SubstraitTypeParser.Identifier)
                    self.state = 121
                    self.expr(0)
                    self.state = 126
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)

                self.state = 127
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [31]:
                localctx = SubstraitTypeParser.ListContext(self, localctx)
                self.enterOuterAlt(localctx, 10)
                self.state = 129
                self.match(SubstraitTypeParser.List)
                self.state = 131
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 130
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 133
                self.match(SubstraitTypeParser.Lt)
                self.state = 134
                self.expr(0)
                self.state = 135
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [32]:
                localctx = SubstraitTypeParser.MapContext(self, localctx)
                self.enterOuterAlt(localctx, 11)
                self.state = 137
                self.match(SubstraitTypeParser.Map)
                self.state = 139
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==70:
                    self.state = 138
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 141
                self.match(SubstraitTypeParser.Lt)
                self.state = 142
                localctx.key = self.expr(0)
                self.state = 143
                self.match(SubstraitTypeParser.Comma)
                self.state = 144
                localctx.value = self.expr(0)
                self.state = 145
                self.match(SubstraitTypeParser.Gt)
                pass
            elif token in [33]:
                localctx = SubstraitTypeParser.UserDefinedContext(self, localctx)
                self.enterOuterAlt(localctx, 12)
                self.state = 147
                self.match(SubstraitTypeParser.UserDefined)
                self.state = 148
                self.match(SubstraitTypeParser.Identifier)
                self.state = 150
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,14,self._ctx)
                if la_ == 1:
                    self.state = 149
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                self.state = 163
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,16,self._ctx)
                if la_ == 1:
                    self.state = 152
                    self.match(SubstraitTypeParser.Lt)
                    self.state = 153
                    self.expr(0)
                    self.state = 158
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)
                    while _la==68:
                        self.state = 154
                        self.match(SubstraitTypeParser.Comma)
                        self.state = 155
                        self.expr(0)
                        self.state = 160
                        self._errHandler.sync(self)
                        _la = self._input.LA(1)

                    self.state = 161
                    self.match(SubstraitTypeParser.Gt)


                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class NumericParameterContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return SubstraitTypeParser.RULE_numericParameter

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)



    class NumericParameterNameContext(NumericParameterContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.NumericParameterContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Identifier(self):
            return self.getToken(SubstraitTypeParser.Identifier, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterNumericParameterName" ):
                listener.enterNumericParameterName(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitNumericParameterName" ):
                listener.exitNumericParameterName(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitNumericParameterName" ):
                return visitor.visitNumericParameterName(self)
            else:
                return visitor.visitChildren(self)


    class NumericLiteralContext(NumericParameterContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.NumericParameterContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Number(self):
            return self.getToken(SubstraitTypeParser.Number, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterNumericLiteral" ):
                listener.enterNumericLiteral(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitNumericLiteral" ):
                listener.exitNumericLiteral(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitNumericLiteral" ):
                return visitor.visitNumericLiteral(self)
            else:
                return visitor.visitChildren(self)


    class NumericExpressionContext(NumericParameterContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.NumericParameterContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def expr(self):
            return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,0)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterNumericExpression" ):
                listener.enterNumericExpression(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitNumericExpression" ):
                listener.exitNumericExpression(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitNumericExpression" ):
                return visitor.visitNumericExpression(self)
            else:
                return visitor.visitChildren(self)



    def numericParameter(self):

        localctx = SubstraitTypeParser.NumericParameterContext(self, self._ctx, self.state)
        self.enterRule(localctx, 8, self.RULE_numericParameter)
        try:
            self.state = 170
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,18,self._ctx)
            if la_ == 1:
                localctx = SubstraitTypeParser.NumericLiteralContext(self, localctx)
                self.enterOuterAlt(localctx, 1)
                self.state = 167
                self.match(SubstraitTypeParser.Number)
                pass

            elif la_ == 2:
                localctx = SubstraitTypeParser.NumericParameterNameContext(self, localctx)
                self.enterOuterAlt(localctx, 2)
                self.state = 168
                self.match(SubstraitTypeParser.Identifier)
                pass

            elif la_ == 3:
                localctx = SubstraitTypeParser.NumericExpressionContext(self, localctx)
                self.enterOuterAlt(localctx, 3)
                self.state = 169
                self.expr(0)
                pass


        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class AnyTypeContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.isnull = None # Token

        def Any(self):
            return self.getToken(SubstraitTypeParser.Any, 0)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def AnyVar(self):
            return self.getToken(SubstraitTypeParser.AnyVar, 0)

        def getRuleIndex(self):
            return SubstraitTypeParser.RULE_anyType

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterAnyType" ):
                listener.enterAnyType(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitAnyType" ):
                listener.exitAnyType(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitAnyType" ):
                return visitor.visitAnyType(self)
            else:
                return visitor.visitChildren(self)




    def anyType(self):

        localctx = SubstraitTypeParser.AnyTypeContext(self, self._ctx, self.state)
        self.enterRule(localctx, 10, self.RULE_anyType)
        try:
            self.state = 180
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [47]:
                self.enterOuterAlt(localctx, 1)
                self.state = 172
                self.match(SubstraitTypeParser.Any)
                self.state = 174
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,19,self._ctx)
                if la_ == 1:
                    self.state = 173
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                pass
            elif token in [48]:
                self.enterOuterAlt(localctx, 2)
                self.state = 176
                self.match(SubstraitTypeParser.AnyVar)
                self.state = 178
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,20,self._ctx)
                if la_ == 1:
                    self.state = 177
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class TypeDefContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser
            self.isnull = None # Token

        def scalarType(self):
            return self.getTypedRuleContext(SubstraitTypeParser.ScalarTypeContext,0)


        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def parameterizedType(self):
            return self.getTypedRuleContext(SubstraitTypeParser.ParameterizedTypeContext,0)


        def anyType(self):
            return self.getTypedRuleContext(SubstraitTypeParser.AnyTypeContext,0)


        def getRuleIndex(self):
            return SubstraitTypeParser.RULE_typeDef

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTypeDef" ):
                listener.enterTypeDef(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTypeDef" ):
                listener.exitTypeDef(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTypeDef" ):
                return visitor.visitTypeDef(self)
            else:
                return visitor.visitChildren(self)




    def typeDef(self):

        localctx = SubstraitTypeParser.TypeDefContext(self, self._ctx, self.state)
        self.enterRule(localctx, 12, self.RULE_typeDef)
        try:
            self.state = 188
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22]:
                self.enterOuterAlt(localctx, 1)
                self.state = 182
                self.scalarType()
                self.state = 184
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,22,self._ctx)
                if la_ == 1:
                    self.state = 183
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                pass
            elif token in [21, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33]:
                self.enterOuterAlt(localctx, 2)
                self.state = 186
                self.parameterizedType()
                pass
            elif token in [47, 48]:
                self.enterOuterAlt(localctx, 3)
                self.state = 187
                self.anyType()
                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class ExprContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser


        def getRuleIndex(self):
            return SubstraitTypeParser.RULE_expr

     
        def copyFrom(self, ctx:ParserRuleContext):
            super().copyFrom(ctx)


    class IfExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.ifExpr = None # ExprContext
            self.thenExpr = None # ExprContext
            self.elseExpr = None # ExprContext
            self.copyFrom(ctx)

        def If(self):
            return self.getToken(SubstraitTypeParser.If, 0)
        def Then(self):
            return self.getToken(SubstraitTypeParser.Then, 0)
        def Else(self):
            return self.getToken(SubstraitTypeParser.Else, 0)
        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.ExprContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,i)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterIfExpr" ):
                listener.enterIfExpr(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitIfExpr" ):
                listener.exitIfExpr(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitIfExpr" ):
                return visitor.visitIfExpr(self)
            else:
                return visitor.visitChildren(self)


    class TypeLiteralContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def typeDef(self):
            return self.getTypedRuleContext(SubstraitTypeParser.TypeDefContext,0)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTypeLiteral" ):
                listener.enterTypeLiteral(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTypeLiteral" ):
                listener.exitTypeLiteral(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTypeLiteral" ):
                return visitor.visitTypeLiteral(self)
            else:
                return visitor.visitChildren(self)


    class MultilineDefinitionContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.finalType = None # TypeDefContext
            self.copyFrom(ctx)

        def Identifier(self, i:int=None):
            if i is None:
                return self.getTokens(SubstraitTypeParser.Identifier)
            else:
                return self.getToken(SubstraitTypeParser.Identifier, i)
        def Eq(self, i:int=None):
            if i is None:
                return self.getTokens(SubstraitTypeParser.Eq)
            else:
                return self.getToken(SubstraitTypeParser.Eq, i)
        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.ExprContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,i)

        def typeDef(self):
            return self.getTypedRuleContext(SubstraitTypeParser.TypeDefContext,0)

        def Newline(self, i:int=None):
            if i is None:
                return self.getTokens(SubstraitTypeParser.Newline)
            else:
                return self.getToken(SubstraitTypeParser.Newline, i)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterMultilineDefinition" ):
                listener.enterMultilineDefinition(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitMultilineDefinition" ):
                listener.exitMultilineDefinition(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitMultilineDefinition" ):
                return visitor.visitMultilineDefinition(self)
            else:
                return visitor.visitChildren(self)


    class TernaryContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.ifExpr = None # ExprContext
            self.thenExpr = None # ExprContext
            self.elseExpr = None # ExprContext
            self.copyFrom(ctx)

        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)
        def Colon(self):
            return self.getToken(SubstraitTypeParser.Colon, 0)
        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.ExprContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,i)


        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterTernary" ):
                listener.enterTernary(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitTernary" ):
                listener.exitTernary(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitTernary" ):
                return visitor.visitTernary(self)
            else:
                return visitor.visitChildren(self)


    class BinaryExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.left = None # ExprContext
            self.op = None # Token
            self.right = None # ExprContext
            self.copyFrom(ctx)

        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.ExprContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,i)

        def And(self):
            return self.getToken(SubstraitTypeParser.And, 0)
        def Or(self):
            return self.getToken(SubstraitTypeParser.Or, 0)
        def Plus(self):
            return self.getToken(SubstraitTypeParser.Plus, 0)
        def Minus(self):
            return self.getToken(SubstraitTypeParser.Minus, 0)
        def Lt(self):
            return self.getToken(SubstraitTypeParser.Lt, 0)
        def Gt(self):
            return self.getToken(SubstraitTypeParser.Gt, 0)
        def Eq(self):
            return self.getToken(SubstraitTypeParser.Eq, 0)
        def Ne(self):
            return self.getToken(SubstraitTypeParser.Ne, 0)
        def Lte(self):
            return self.getToken(SubstraitTypeParser.Lte, 0)
        def Gte(self):
            return self.getToken(SubstraitTypeParser.Gte, 0)
        def Asterisk(self):
            return self.getToken(SubstraitTypeParser.Asterisk, 0)
        def ForwardSlash(self):
            return self.getToken(SubstraitTypeParser.ForwardSlash, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterBinaryExpr" ):
                listener.enterBinaryExpr(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitBinaryExpr" ):
                listener.exitBinaryExpr(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitBinaryExpr" ):
                return visitor.visitBinaryExpr(self)
            else:
                return visitor.visitChildren(self)


    class ParenExpressionContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def OParen(self):
            return self.getToken(SubstraitTypeParser.OParen, 0)
        def expr(self):
            return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,0)

        def CParen(self):
            return self.getToken(SubstraitTypeParser.CParen, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterParenExpression" ):
                listener.enterParenExpression(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitParenExpression" ):
                listener.exitParenExpression(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitParenExpression" ):
                return visitor.visitParenExpression(self)
            else:
                return visitor.visitChildren(self)


    class ParameterNameContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.isnull = None # Token
            self.copyFrom(ctx)

        def Identifier(self):
            return self.getToken(SubstraitTypeParser.Identifier, 0)
        def QMark(self):
            return self.getToken(SubstraitTypeParser.QMark, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterParameterName" ):
                listener.enterParameterName(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitParameterName" ):
                listener.exitParameterName(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitParameterName" ):
                return visitor.visitParameterName(self)
            else:
                return visitor.visitChildren(self)


    class FunctionCallContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Identifier(self):
            return self.getToken(SubstraitTypeParser.Identifier, 0)
        def OParen(self):
            return self.getToken(SubstraitTypeParser.OParen, 0)
        def CParen(self):
            return self.getToken(SubstraitTypeParser.CParen, 0)
        def expr(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(SubstraitTypeParser.ExprContext)
            else:
                return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,i)

        def Comma(self, i:int=None):
            if i is None:
                return self.getTokens(SubstraitTypeParser.Comma)
            else:
                return self.getToken(SubstraitTypeParser.Comma, i)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterFunctionCall" ):
                listener.enterFunctionCall(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitFunctionCall" ):
                listener.exitFunctionCall(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitFunctionCall" ):
                return visitor.visitFunctionCall(self)
            else:
                return visitor.visitChildren(self)


    class NotExprContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def expr(self):
            return self.getTypedRuleContext(SubstraitTypeParser.ExprContext,0)

        def Bang(self):
            return self.getToken(SubstraitTypeParser.Bang, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterNotExpr" ):
                listener.enterNotExpr(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitNotExpr" ):
                listener.exitNotExpr(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitNotExpr" ):
                return visitor.visitNotExpr(self)
            else:
                return visitor.visitChildren(self)


    class LiteralNumberContext(ExprContext):

        def __init__(self, parser, ctx:ParserRuleContext): # actually a SubstraitTypeParser.ExprContext
            super().__init__(parser)
            self.copyFrom(ctx)

        def Number(self):
            return self.getToken(SubstraitTypeParser.Number, 0)

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterLiteralNumber" ):
                listener.enterLiteralNumber(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitLiteralNumber" ):
                listener.exitLiteralNumber(self)

        def accept(self, visitor:ParseTreeVisitor):
            if hasattr( visitor, "visitLiteralNumber" ):
                return visitor.visitLiteralNumber(self)
            else:
                return visitor.visitChildren(self)



    def expr(self, _p:int=0):
        _parentctx = self._ctx
        _parentState = self.state
        localctx = SubstraitTypeParser.ExprContext(self, self._ctx, _parentState)
        _prevctx = localctx
        _startState = 14
        self.enterRecursionRule(localctx, 14, self.RULE_expr, _p)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 251
            self._errHandler.sync(self)
            la_ = self._interp.adaptivePredict(self._input,31,self._ctx)
            if la_ == 1:
                localctx = SubstraitTypeParser.ParenExpressionContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx

                self.state = 191
                self.match(SubstraitTypeParser.OParen)
                self.state = 192
                self.expr(0)
                self.state = 193
                self.match(SubstraitTypeParser.CParen)
                pass

            elif la_ == 2:
                localctx = SubstraitTypeParser.MultilineDefinitionContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 195
                self.match(SubstraitTypeParser.Identifier)
                self.state = 196
                self.match(SubstraitTypeParser.Eq)
                self.state = 197
                self.expr(0)
                self.state = 199 
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                while True:
                    self.state = 198
                    self.match(SubstraitTypeParser.Newline)
                    self.state = 201 
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)
                    if not (_la==78):
                        break

                self.state = 213
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                while _la==77:
                    self.state = 203
                    self.match(SubstraitTypeParser.Identifier)
                    self.state = 204
                    self.match(SubstraitTypeParser.Eq)
                    self.state = 205
                    self.expr(0)
                    self.state = 207 
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)
                    while True:
                        self.state = 206
                        self.match(SubstraitTypeParser.Newline)
                        self.state = 209 
                        self._errHandler.sync(self)
                        _la = self._input.LA(1)
                        if not (_la==78):
                            break

                    self.state = 215
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)

                self.state = 216
                localctx.finalType = self.typeDef()
                self.state = 220
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,27,self._ctx)
                while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                    if _alt==1:
                        self.state = 217
                        self.match(SubstraitTypeParser.Newline) 
                    self.state = 222
                    self._errHandler.sync(self)
                    _alt = self._interp.adaptivePredict(self._input,27,self._ctx)

                pass

            elif la_ == 3:
                localctx = SubstraitTypeParser.TypeLiteralContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 223
                self.typeDef()
                pass

            elif la_ == 4:
                localctx = SubstraitTypeParser.LiteralNumberContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 224
                self.match(SubstraitTypeParser.Number)
                pass

            elif la_ == 5:
                localctx = SubstraitTypeParser.ParameterNameContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 225
                self.match(SubstraitTypeParser.Identifier)
                self.state = 227
                self._errHandler.sync(self)
                la_ = self._interp.adaptivePredict(self._input,28,self._ctx)
                if la_ == 1:
                    self.state = 226
                    localctx.isnull = self.match(SubstraitTypeParser.QMark)


                pass

            elif la_ == 6:
                localctx = SubstraitTypeParser.FunctionCallContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 229
                self.match(SubstraitTypeParser.Identifier)
                self.state = 230
                self.match(SubstraitTypeParser.OParen)
                self.state = 239
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if (((_la) & ~0x3f) == 0 and ((1 << _la) & 2306265238858629008) != 0) or ((((_la - 64)) & ~0x3f) == 0 and ((1 << (_la - 64)) & 12289) != 0):
                    self.state = 231
                    self.expr(0)
                    self.state = 236
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)
                    while _la==68:
                        self.state = 232
                        self.match(SubstraitTypeParser.Comma)
                        self.state = 233
                        self.expr(0)
                        self.state = 238
                        self._errHandler.sync(self)
                        _la = self._input.LA(1)



                self.state = 241
                self.match(SubstraitTypeParser.CParen)
                pass

            elif la_ == 7:
                localctx = SubstraitTypeParser.IfExprContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx
                self.state = 242
                self.match(SubstraitTypeParser.If)
                self.state = 243
                localctx.ifExpr = self.expr(0)
                self.state = 244
                self.match(SubstraitTypeParser.Then)
                self.state = 245
                localctx.thenExpr = self.expr(0)
                self.state = 246
                self.match(SubstraitTypeParser.Else)
                self.state = 247
                localctx.elseExpr = self.expr(3)
                pass

            elif la_ == 8:
                localctx = SubstraitTypeParser.NotExprContext(self, localctx)
                self._ctx = localctx
                _prevctx = localctx

                self.state = 249
                self.match(SubstraitTypeParser.Bang)
                self.state = 250
                self.expr(2)
                pass


            self._ctx.stop = self._input.LT(-1)
            self.state = 264
            self._errHandler.sync(self)
            _alt = self._interp.adaptivePredict(self._input,33,self._ctx)
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt==1:
                    if self._parseListeners is not None:
                        self.triggerExitRuleEvent()
                    _prevctx = localctx
                    self.state = 262
                    self._errHandler.sync(self)
                    la_ = self._interp.adaptivePredict(self._input,32,self._ctx)
                    if la_ == 1:
                        localctx = SubstraitTypeParser.BinaryExprContext(self, SubstraitTypeParser.ExprContext(self, _parentctx, _parentState))
                        localctx.left = _prevctx
                        self.pushNewRecursionContext(localctx, _startState, self.RULE_expr)
                        self.state = 253
                        if not self.precpred(self._ctx, 4):
                            from antlr4.error.Errors import FailedPredicateException
                            raise FailedPredicateException(self, "self.precpred(self._ctx, 4)")
                        self.state = 254
                        localctx.op = self._input.LT(1)
                        _la = self._input.LA(1)
                        if not(((((_la - 50)) & ~0x3f) == 0 and ((1 << (_la - 50)) & 25167855) != 0)):
                            localctx.op = self._errHandler.recoverInline(self)
                        else:
                            self._errHandler.reportMatch(self)
                            self.consume()
                        self.state = 255
                        localctx.right = self.expr(5)
                        pass

                    elif la_ == 2:
                        localctx = SubstraitTypeParser.TernaryContext(self, SubstraitTypeParser.ExprContext(self, _parentctx, _parentState))
                        localctx.ifExpr = _prevctx
                        self.pushNewRecursionContext(localctx, _startState, self.RULE_expr)
                        self.state = 256
                        if not self.precpred(self._ctx, 1):
                            from antlr4.error.Errors import FailedPredicateException
                            raise FailedPredicateException(self, "self.precpred(self._ctx, 1)")
                        self.state = 257
                        self.match(SubstraitTypeParser.QMark)
                        self.state = 258
                        localctx.thenExpr = self.expr(0)
                        self.state = 259
                        self.match(SubstraitTypeParser.Colon)
                        self.state = 260
                        localctx.elseExpr = self.expr(2)
                        pass

             
                self.state = 266
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,33,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.unrollRecursionContexts(_parentctx)
        return localctx



    def sempred(self, localctx:RuleContext, ruleIndex:int, predIndex:int):
        if self._predicates == None:
            self._predicates = dict()
        self._predicates[7] = self.expr_sempred
        pred = self._predicates.get(ruleIndex, None)
        if pred is None:
            raise Exception("No predicate with index:" + str(ruleIndex))
        else:
            return pred(localctx, predIndex)

    def expr_sempred(self, localctx:ExprContext, predIndex:int):
            if predIndex == 0:
                return self.precpred(self._ctx, 4)
         

            if predIndex == 1:
                return self.precpred(self._ctx, 1)
         




