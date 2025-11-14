# SPDX-License-Identifier: Apache-2.0
from __future__ import annotations

import os
import yaml
from dataclasses import dataclass
from typing import Union

from tests.coverage.antlr_parser.FuncTestCaseLexer import FuncTestCaseLexer
from tests.coverage.antlr_parser.FuncTestCaseParser import FuncTestCaseParser
from tests.coverage.nodes import SubstraitError

enable_debug = False


def error(msg):
    print(f"ERROR: {msg}")


def debug(msg):
    if enable_debug:
        print(f"DEBUG: {msg}")


def substrait_type_str(rule_num):
    return FuncTestCaseLexer.symbolicNames[rule_num].lower()


# Structured type representations
@dataclass(frozen=True)
class TypeVariable:
    """Represents a type variable like T, U, V"""
    name: str
    nullable: bool = False

    def to_string(self) -> str:
        """Convert to string representation"""
        result = self.name
        if self.nullable:
            result += "?"
        return result


@dataclass(frozen=True)
class SimpleType:
    """Represents a simple type like i32, str, boolean"""
    name: str
    nullable: bool = False

    def to_string(self) -> str:
        """Convert to string representation"""
        result = self.name
        if self.nullable:
            result += "?"
        return result


@dataclass(frozen=True)
class ListType:
    """Represents list<element_type>"""
    element_type: Type
    nullable: bool = False

    def to_string(self) -> str:
        """Convert to string representation"""
        elem_str = self.element_type.to_string()
        result = f"list<{elem_str}>"
        if self.nullable:
            result += "?"
        return result


@dataclass(frozen=True)
class StructType:
    """Represents struct<field1, field2, ...>"""
    field_types: list[Type]
    nullable: bool = False

    def to_string(self) -> str:
        """Convert to string representation"""
        fields_str = ",".join(f.to_string() for f in self.field_types)
        result = f"struct<{fields_str}>"
        if self.nullable:
            result += "?"
        return result


@dataclass(frozen=True)
class ParameterizedType:
    """Represents other parameterized types like decimal<P,S>, varchar<N>"""
    base: str
    params: list[Type | int]
    nullable: bool = False

    def to_string(self) -> str:
        """Convert to string representation"""
        params_str = ",".join(
            str(p) if isinstance(p, int) else p.to_string()
            for p in self.params
        )
        result = f"{self.base}<{params_str}>"
        if self.nullable:
            result += "?"
        return result


@dataclass(frozen=True)
class LambdaType:
    """Represents lambda<param -> return>"""
    param_type: Type
    return_type: Type
    nullable: bool = False

    def to_string(self) -> str:
        """Convert to string representation"""
        param_str = self.param_type.to_string()
        return_str = self.return_type.to_string()
        result = f"lambda<{param_str}->{return_str}>"
        if self.nullable:
            result += "?"
        return result


# Union type for all type representations
Type = Union[TypeVariable, SimpleType, ListType, StructType, ParameterizedType, LambdaType]


def build_type_from_context(ctx) -> Type:
    """Convert ANTLR type context to structured Type"""
    type_text = ctx.getText()

    # Extract nullable flag if present
    nullable = hasattr(ctx, 'isnull') and ctx.isnull is not None

    # Check for type variable - this is now handled by the grammar's #typeVariable rule
    # The context will have an Identifier() method if it's a type variable
    if hasattr(ctx, 'Identifier') and callable(ctx.Identifier) and ctx.Identifier():
        identifier = ctx.Identifier().getText()
        return TypeVariable(identifier, nullable=nullable)

    # Check for lambda type
    if hasattr(ctx, 'Lambda') and ctx.Lambda():
        return build_lambda_type(ctx, nullable=nullable)

    # Check if this is a dataType that wraps a parameterizedType
    if hasattr(ctx, 'parameterizedType') and ctx.parameterizedType():
        return build_type_from_context(ctx.parameterizedType())

    # Check for list type (has elemType, not dataTypeList)
    if hasattr(ctx, 'elemType') and ctx.elemType:
        elem_type = build_type_from_context(ctx.elemType)
        return ListType(element_type=elem_type, nullable=nullable)

    # Check for struct type
    if hasattr(ctx, 'Struct') and ctx.Struct():
        return build_struct_type(ctx, nullable=nullable)

    # Check for other parameterized types (decimal, varchar, etc.)
    if hasattr(ctx, 'dataTypeList') and ctx.dataTypeList():
        return build_parameterized_type(ctx, nullable=nullable)

    # Simple type (just get the base name, remove any trailing ?)
    base_name = type_text.split("<")[0].rstrip("?").lower()
    return SimpleType(base_name, nullable=nullable)


def build_lambda_type(lambda_ctx, nullable: bool = False) -> LambdaType:
    """Build LambdaType from lambda context"""
    param_list_ctx = lambda_ctx.dataTypeList()
    return_ctx = lambda_ctx.dataType()

    # For lambda, param could be single type or struct
    if param_list_ctx:
        # Multiple params means struct
        param_types = [
            build_type_from_context(dt)
            for dt in param_list_ctx.dataType()
        ]
        if len(param_types) == 1:
            param_type = param_types[0]
        else:
            # Multiple params wrapped in struct
            param_type = StructType(field_types=param_types)
    else:
        # Single param (shouldn't happen in our grammar but handle it)
        param_type = SimpleType("unknown")

    return_type = build_type_from_context(return_ctx)
    return LambdaType(
        param_type=param_type,
        return_type=return_type,
        nullable=nullable
    )


def build_struct_type(ctx, nullable: bool = False) -> StructType:
    """Build StructType from context"""
    field_types = []
    if hasattr(ctx, 'dataTypeList') and ctx.dataTypeList():
        for dt_ctx in ctx.dataTypeList().dataType():
            field_types.append(build_type_from_context(dt_ctx))
    return StructType(field_types=field_types, nullable=nullable)


def build_parameterized_type(ctx, nullable: bool = False) -> ParameterizedType:
    """Build ParameterizedType for decimal, varchar, etc."""
    base_name = None

    # Determine base type name
    if hasattr(ctx, 'Map') and ctx.Map():
        base_name = "map"
    else:
        # Fallback: extract from text
        text = ctx.getText()
        base_name = text.split("<")[0].rstrip("?").lower()

    # Get type parameters (could be types or numeric params)
    param_types = []
    if hasattr(ctx, 'dataTypeList') and ctx.dataTypeList():
        for dt_ctx in ctx.dataTypeList().dataType():
            param_types.append(build_type_from_context(dt_ctx))
    elif hasattr(ctx, 'dataType') and callable(ctx.dataType):
        # Single parameter
        param_types.append(build_type_from_context(ctx.dataType()))

    return ParameterizedType(base_name, param_types, nullable=nullable)


def parse_type_string(type_str: str) -> Type:
    """Parse a type string using the SubstraitType.g4 grammar

    This uses SubstraitType grammar because FuncTestCaseParser lexer has
    conflicts (e.g., 'T' is tokenized as TimePrefix, not Identifier).
    The SubstraitType grammar is designed for parsing type expressions from
    YAML extension files, which is exactly what this function needs to do.

    For parsing types from test case ANTLR contexts, use
    build_type_from_context().
    """
    from antlr4 import InputStream, CommonTokenStream
    from tests.type.antlr_parser.SubstraitTypeLexer import (
        SubstraitTypeLexer
    )
    from tests.type.antlr_parser.SubstraitTypeParser import (
        SubstraitTypeParser
    )

    # Normalize short type names to long names for SubstraitType grammar
    # The grammar uses long names (STRING, BOOLEAN, etc.)
    type_str_normalized = _normalize_type_string(type_str).upper()

    # Create input stream and parse using startRule (expr EOF)
    input_stream = InputStream(type_str_normalized)
    lexer = SubstraitTypeLexer(input_stream)
    token_stream = CommonTokenStream(lexer)
    parser = SubstraitTypeParser(token_stream)

    # Use the startRule (expr EOF) which can handle identifiers
    tree = parser.startRule()

    # Convert expr to Type, preserving original case
    return _build_type_from_expr(tree.expr(), type_str)


def _normalize_type_string(type_str: str) -> str:
    """Normalize short type names to long names for SubstraitType grammar

    The SubstraitType grammar uses long names (STRING, BOOLEAN, etc.)
    but YAML files may contain short names (str, bool, etc.).
    """
    # Build mapping of short to long names
    short_to_long = {
        'bool': 'boolean',
        'str': 'string',
        'vbin': 'binary',
        'ts': 'timestamp',
        'tstz': 'timestamp_tz',
        'iyear': 'interval_year',
        'iday': 'interval_day',
        'dec': 'decimal',
        'pt': 'precision_time',
        'pts': 'precision_timestamp',
        'ptstz': 'precision_timestamp_tz',
        'fchar': 'fixedchar',
        'vchar': 'varchar',
        'fbin': 'fixedbinary',
    }

    # Replace short names with long names
    # Need to be careful with word boundaries
    import re
    result = type_str
    for short, long in short_to_long.items():
        # Match word boundaries to avoid partial replacements
        pattern = r'\b' + re.escape(short) + r'\b'
        result = re.sub(pattern, long, result, flags=re.IGNORECASE)

    return result


def _build_type_from_expr(ctx, original_str: str) -> Type:
    """Build Type from SubstraitType grammar expr context

    original_str is the original (possibly lowercase) type string for
    preserving case in identifiers.
    """
    from tests.type.antlr_parser.SubstraitTypeParser import (
        SubstraitTypeParser
    )

    # TypeLiteral -> typeDef (scalar, parameterized, any)
    if isinstance(ctx, SubstraitTypeParser.TypeLiteralContext):
        return _build_type_from_typedef(ctx.typeDef(), original_str)

    # ParameterName -> Identifier with optional nullable (type variables)
    if isinstance(ctx, SubstraitTypeParser.ParameterNameContext):
        nullable = ctx.isnull is not None
        # Preserve original case for type variables
        name = original_str.rstrip('?')
        return TypeVariable(name, nullable=nullable)

    raise ValueError(f"Unexpected expr type: {type(ctx).__name__}")


def _build_type_from_typedef(ctx, original_str: str) -> Type:
    """Build Type from SubstraitType grammar typeDef context"""
    # Check nullable
    nullable = hasattr(ctx, 'isnull') and ctx.isnull is not None

    # anyType (ANY, ANY1, etc.)
    if ctx.anyType():
        # For ANY types, preserve original case
        return TypeVariable(original_str.rstrip('?'), nullable=nullable)

    # parameterizedType
    if ctx.parameterizedType():
        return _build_parameterized_from_substrait(
            ctx.parameterizedType(), original_str, nullable
        )

    # scalarType
    if ctx.scalarType():
        # Get the name and convert back to short form
        name = ctx.scalarType().getText().lower()
        # Normalize to short form (our internal representation)
        name = Extension.get_short_type(name)
        return SimpleType(name=name, nullable=nullable)

    raise ValueError(f"Unknown typeDef: {ctx.getText()}")


def _build_parameterized_from_substrait(
    ctx, original_str: str, nullable: bool
) -> Type:
    """Build parameterized type from SubstraitType context

    Uses ANTLR parse tree to extract child expr() contexts,
    then recursively converts them to Types.
    """
    from tests.type.antlr_parser.SubstraitTypeParser import (
        SubstraitTypeParser
    )

    # Lambda type: lambda<param->return> or just lambda (without params)
    if isinstance(ctx, SubstraitTypeParser.LambdaContext):
        # Grammar: Lambda ... Lt expr Arrow expr Gt
        exprs = ctx.expr()
        if exprs is None or len(exprs) == 0:
            # Lambda without parameters - use generic type variables
            param_type = TypeVariable("T")
            return_type = TypeVariable("U")
        elif len(exprs) != 2:
            raise ValueError(f"Lambda must have 2 exprs: {ctx.getText()}")
        else:
            # Recursively build param and return types from expr contexts
            param_type = _build_type_from_expr(
                exprs[0], _extract_original_part(original_str, exprs[0])
            )
            return_type = _build_type_from_expr(
                exprs[1], _extract_original_part(original_str, exprs[1])
            )
        return LambdaType(
            param_type=param_type,
            return_type=return_type,
            nullable=nullable
        )

    # List type: list<elem> or just list (without element type)
    if isinstance(ctx, SubstraitTypeParser.ListContext):
        # Grammar: List ... Lt expr Gt
        elem_expr = ctx.expr()
        if elem_expr is None:
            # List without element type - use generic "any" element
            elem_type = TypeVariable("T")
        else:
            elem_type = _build_type_from_expr(
                elem_expr, _extract_original_part(original_str, elem_expr)
            )
        return ListType(element_type=elem_type, nullable=nullable)

    # Struct type: struct<field1, field2, ...>
    if isinstance(ctx, SubstraitTypeParser.StructContext):
        # Grammar: Struct ... Lt expr (Comma expr)* Gt
        field_exprs = ctx.expr()
        field_types = [
            _build_type_from_expr(
                e, _extract_original_part(original_str, e)
            )
            for e in field_exprs
        ]
        return StructType(field_types=field_types, nullable=nullable)

    # For other parameterized types (decimal, fixedchar, etc.),
    # extract the base name and handle parameters
    type_name = ctx.getChild(0).getText().lower()

    # Decimal: decimal<precision, scale> or just decimal
    if isinstance(ctx, SubstraitTypeParser.DecimalContext):
        if ctx.precision and ctx.scale:
            precision = int(ctx.precision.getText())
            scale = int(ctx.scale.getText())
            return ParameterizedType(
                base=type_name, params=[precision, scale], nullable=nullable
            )
        else:
            # decimal without parameters - treat as simple type
            return SimpleType(name=type_name, nullable=nullable)

    # FixedChar, VarChar, FixedBinary: <length> or without length
    if isinstance(
        ctx,
        (
            SubstraitTypeParser.FixedCharContext,
            SubstraitTypeParser.VarCharContext,
            SubstraitTypeParser.FixedBinaryContext
        )
    ):
        if ctx.length:
            length = int(ctx.length.getText())
            return ParameterizedType(
                base=type_name, params=[length], nullable=nullable
            )
        else:
            # Without parameters - treat as simple type
            return SimpleType(name=type_name, nullable=nullable)

    # Precision types: <precision> or without parameters
    if isinstance(
        ctx,
        (
            SubstraitTypeParser.PrecisionTimeContext,
            SubstraitTypeParser.PrecisionTimestampContext,
            SubstraitTypeParser.PrecisionTimestampTZContext,
            SubstraitTypeParser.PrecisionIntervalDayContext
        )
    ):
        if ctx.precision:
            precision = int(ctx.precision.getText())
            return ParameterizedType(
                base=type_name, params=[precision], nullable=nullable
            )
        else:
            # Without parameters - treat as simple type
            return SimpleType(name=type_name, nullable=nullable)

    raise ValueError(f"Unknown parameterized type: {type(ctx).__name__}")


def _extract_original_part(original_str: str, expr_ctx) -> str:
    """Extract the substring from original_str that corresponds to expr_ctx

    This preserves the original case from the input string by using
    the token positions from the parse tree.
    """
    # Handle None context (shouldn't happen but be defensive)
    if expr_ctx is None:
        return original_str

    # Get the text from the context
    text = expr_ctx.getText()

    # For identifiers (type variables), preserve uppercase
    # Check if it's a single uppercase letter (type variable)
    if len(text) == 1 and text.isupper():
        return text

    # For other cases, lowercase it for re-parsing
    # This handles types like I32, STRING, etc.
    return text.lower()


def types_match(pattern: Type, concrete: Type, bindings: dict) -> bool:
    """
    Match pattern type (may contain type variables) against concrete type.
    Updates bindings dict with type variable assignments.
    Returns True if match succeeds with consistent bindings.
    """
    # Type variable matching
    if isinstance(pattern, TypeVariable):
        if pattern.name in bindings:
            # Type variable already bound, must match
            return bindings[pattern.name] == concrete
        else:
            # Bind the type variable
            bindings[pattern.name] = concrete
            return True

    # Simple type matching
    if isinstance(pattern, SimpleType):
        return isinstance(concrete, SimpleType) and pattern.name == concrete.name

    # List type matching
    if isinstance(pattern, ListType):
        if not isinstance(concrete, ListType):
            return False
        return types_match(pattern.element_type, concrete.element_type, bindings)

    # Struct type matching
    if isinstance(pattern, StructType):
        if not isinstance(concrete, StructType):
            return False
        if len(pattern.field_types) != len(concrete.field_types):
            return False
        return all(
            types_match(p_field, c_field, bindings)
            for p_field, c_field in zip(pattern.field_types, concrete.field_types)
        )

    # Lambda type matching
    if isinstance(pattern, LambdaType):
        if not isinstance(concrete, LambdaType):
            return False
        return types_match(
            pattern.param_type, concrete.param_type, bindings
        ) and types_match(pattern.return_type, concrete.return_type, bindings)

    # Parameterized type matching
    if isinstance(pattern, ParameterizedType):
        if not isinstance(concrete, ParameterizedType):
            return False
        if pattern.base != concrete.base:
            return False
        if len(pattern.params) != len(concrete.params):
            return False
        return all(
            types_match(p, c, bindings)
            for p, c in zip(pattern.params, concrete.params)
        )

    return False


def type_to_string(t: Type) -> str:
    """Convert structured Type back to string representation"""
    return t.to_string()


def build_type_to_short_type():
    rule_map = {
        FuncTestCaseLexer.I8: FuncTestCaseLexer.I8,
        FuncTestCaseLexer.I16: FuncTestCaseLexer.I16,
        FuncTestCaseLexer.I32: FuncTestCaseLexer.I32,
        FuncTestCaseLexer.I64: FuncTestCaseLexer.I64,
        FuncTestCaseLexer.FP32: FuncTestCaseLexer.FP32,
        FuncTestCaseLexer.FP64: FuncTestCaseLexer.FP64,
        FuncTestCaseLexer.String: FuncTestCaseLexer.Str,
        FuncTestCaseLexer.Binary: FuncTestCaseLexer.VBin,
        FuncTestCaseLexer.Boolean: FuncTestCaseLexer.Bool,
        FuncTestCaseLexer.Timestamp: FuncTestCaseLexer.Ts,
        FuncTestCaseLexer.Timestamp_TZ: FuncTestCaseLexer.TsTZ,
        FuncTestCaseLexer.Date: FuncTestCaseLexer.Date,
        FuncTestCaseLexer.Time: FuncTestCaseLexer.Time,
        FuncTestCaseLexer.Interval_Year: FuncTestCaseLexer.IYear,
        FuncTestCaseLexer.Interval_Day: FuncTestCaseLexer.IDay,
        FuncTestCaseLexer.UUID: FuncTestCaseLexer.UUID,
        FuncTestCaseLexer.FixedChar: FuncTestCaseLexer.FChar,
        FuncTestCaseLexer.VarChar: FuncTestCaseLexer.VChar,
        FuncTestCaseLexer.FixedBinary: FuncTestCaseLexer.FBin,
        FuncTestCaseLexer.Decimal: FuncTestCaseLexer.Dec,
        FuncTestCaseLexer.Precision_Timestamp: FuncTestCaseLexer.PTs,
        FuncTestCaseLexer.Precision_Timestamp_TZ: FuncTestCaseLexer.PTsTZ,
        FuncTestCaseLexer.Struct: FuncTestCaseLexer.Struct,
        FuncTestCaseLexer.List: FuncTestCaseLexer.List,
        FuncTestCaseLexer.Map: FuncTestCaseLexer.Map,
        FuncTestCaseLexer.Lambda: FuncTestCaseLexer.Lambda,
        FuncTestCaseLexer.Any: FuncTestCaseLexer.Any,
    }
    to_short_type = {
        substrait_type_str(k): substrait_type_str(v) for k, v in rule_map.items()
    }
    any_type = substrait_type_str(FuncTestCaseLexer.Any)
    for i in range(1, 3):
        to_short_type[f"{any_type}{i}"] = f"{any_type}{i}"
    # Note: Type variables (single uppercase letters like T, U, V) are preserved
    # and handled dynamically in get_short_type() and is_same_type()
    return to_short_type


type_to_short_type = build_type_to_short_type()
short_type_to_type = {st: lt for lt, st in type_to_short_type.items()}


class Extension:
    @staticmethod
    def get_base_uri():
        return "https://github.com/substrait-io/substrait/blob/main/extensions/"

    @staticmethod
    def is_type_variable(type_str):
        """Check if a type string is a type variable (single uppercase letter)"""
        return len(type_str) == 1 and type_str.isupper()

    @staticmethod
    def get_short_type(long_type):
        long_type = long_type.rstrip("?")

        # Preserve single uppercase letters as type variables (T, U, V, etc.)
        # Don't lowercase them like other types
        if Extension.is_type_variable(long_type):
            return long_type

        long_type = long_type.lower()
        short_type = type_to_short_type.get(long_type, None)

        if short_type is None:
            # remove the type parameters and try again
            if "<" in long_type:
                long_type = long_type[: long_type.find("<")].rstrip("?")
                short_type = type_to_short_type.get(long_type, None)
            if short_type is None:
                if "\n" in long_type:
                    long_type = long_type.split("\n")[-1]
                    short_type = type_to_short_type.get(long_type, None)
            if short_type is None:
                if "!" not in long_type:
                    error(f"Type not found in the mapping: {long_type}")
                return long_type
        return short_type

    @staticmethod
    def get_long_type(short_type):
        # Type variables map to themselves
        if Extension.is_type_variable(short_type):
            return short_type

        if short_type.endswith("?"):
            short_type = short_type[:-1]
        long_type = short_type_to_type.get(short_type, None)
        if long_type is None:
            error(f"Type not found in the mapping: {short_type}")
            return short_type
        return long_type

    @staticmethod
    def get_supported_kernels_from_impls(func):
        overloads = []
        for impl in func["impls"]:
            args = []
            if "args" in impl:
                for arg in impl["args"]:
                    if "value" in arg:
                        arg_type = arg["value"]
                        if arg_type.endswith("?"):
                            arg_type = arg_type[:-1]
                        args.append(Extension.get_short_type(arg_type))
                    else:
                        debug(
                            f"arg is not a value type for function: {func['name']} arg must be enum options {arg['options']}"
                        )
                        args.append("str")
            overloads.append(
                FunctionOverload(
                    args, Extension.get_short_type(impl["return"]), "variadic" in impl
                )
            )
        return overloads

    @staticmethod
    def add_functions_to_map(func_list, function_map, suffix, extension, uri):
        dup_idx = 0
        for func in func_list:
            name = func["name"]
            if name in function_map:
                debug(
                    f"Duplicate function name: {name} renaming to {name}_{suffix} extension: {extension}"
                )
                dup_idx += 1
                name = f"{name}_dup{dup_idx}_{suffix}"
                assert (
                    name not in function_map
                ), f"Duplicate function name: {name} renaming to {name}_{suffix} extension: {extension}"
            func["overloads"] = Extension.get_supported_kernels_from_impls(func)
            func["uri"] = uri
            func.pop("description", None)
            func.pop("impls", None)
            function_map[name] = func

    @staticmethod
    def read_substrait_extensions(dir_path: str):
        # read files from extensions directory
        extensions = []
        for root, dirs, files in os.walk(dir_path):
            for file in files:
                if file.endswith(".yaml") and file.startswith("functions_"):
                    extensions.append(os.path.join(root, file))

        extensions.sort()

        scalar_functions = {}
        aggregate_functions = {}
        window_functions = {}
        dependencies = {}
        # convert yaml file to a python dictionary
        for extension in extensions:
            suffix = extension[:-5]  # strip .yaml at the end of the extension
            suffix = suffix[
                suffix.rfind("/") + 1 :
            ]  # strip the path and get the name of the extension
            uri = f"/extensions/{suffix}.yaml"
            suffix = suffix[suffix.find("_") + 1 :]  # get the suffix after the last _

            dependencies[suffix] = Extension.get_base_uri() + uri
            with open(extension, "r") as fh:
                data = yaml.load(fh, Loader=yaml.FullLoader)
                if "scalar_functions" in data:
                    Extension.add_functions_to_map(
                        data["scalar_functions"],
                        scalar_functions,
                        suffix,
                        extension,
                        uri,
                    )
                if "aggregate_functions" in data:
                    Extension.add_functions_to_map(
                        data["aggregate_functions"],
                        aggregate_functions,
                        suffix,
                        extension,
                        uri,
                    )
                if "window_functions" in data:
                    Extension.add_functions_to_map(
                        data["window_functions"],
                        window_functions,
                        suffix,
                        extension,
                        uri,
                    )

        return FunctionRegistry(
            scalar_functions, aggregate_functions, window_functions, dependencies
        )


class FunctionType:
    SCALAR = 1
    AGGREGATE = 2
    WINDOW = 3


class FunctionVariant:
    def __init__(self, name, uri, description, args, return_type, variadic, func_type):
        self.name = name
        self.uri = uri
        self.description = description
        self.args = args
        self.return_type = return_type
        self.variadic = variadic
        self.func_type = func_type
        self.test_count = 0

    def __str__(self):
        return f"Function(name={self.name}, uri={self.uri}, description={self.description}, overloads={self.overload}, args={self.args}, result={self.result})"

    def increment_test_count(self, count=1):
        self.test_count += count


class FunctionOverload:
    def __init__(self, args, return_type, variadic):
        self.args = args
        self.return_type = return_type
        self.variadic = variadic

    def __str__(self):
        return f"FunctionOverload(args={self.args}, result={self.return_type}, variadic={self.variadic})"


# define function type enum


class FunctionRegistry:
    registry = dict()
    dependencies = dict()
    scalar_functions = dict()
    aggregate_functions = dict()
    window_functions = dict()
    extensions = set()

    def __init__(
        self, scalar_functions, aggregate_functions, window_functions, dependencies
    ):
        self.dependencies = dependencies
        self.scalar_functions = scalar_functions
        self.aggregate_functions = aggregate_functions
        self.window_functions = window_functions
        self.add_functions(scalar_functions, FunctionType.SCALAR)
        self.add_functions(aggregate_functions, FunctionType.AGGREGATE)
        self.add_functions(window_functions, FunctionType.WINDOW)

    def add_functions(self, functions, func_type):
        for func in functions.values():
            self.extensions.add(func["uri"])
            f_name = func["name"]
            fun_arr = self.registry.get(f_name, [])
            for overload in func["overloads"]:
                function = FunctionVariant(
                    func["name"],
                    func["uri"],
                    "",
                    overload.args,
                    overload.return_type,
                    overload.variadic,
                    func_type,
                )
                fun_arr.append(function)
            self.registry[f_name] = fun_arr

    @staticmethod
    def is_type_any(func_arg_type):
        return func_arg_type[:3] == "any"

    @staticmethod
    def is_same_type(func_arg_type, arg_type):
        """Check if two types match, with support for type variables.

        Args:
            func_arg_type: Type object or string
            arg_type: Type object or string

        Returns:
            bool: True if types match

        Note:
            Strings are automatically converted to Type objects for comparison.
        """
        # Convert strings to Type objects for comparison
        if isinstance(func_arg_type, str):
            func_arg_type = parse_type_string(func_arg_type)
        if isinstance(arg_type, str):
            arg_type = parse_type_string(arg_type)

        # Type variables match any concrete type
        if isinstance(func_arg_type, TypeVariable):
            return True

        # Use structured type matching with type variable binding
        bindings = {}
        return types_match(func_arg_type, arg_type, bindings)

    def get_function(
        self, name: str, uri: str, args: object, return_type
    ) -> [FunctionVariant]:
        """Get function variant matching name, uri, args, and return type.

        Both args and return_type can be either strings or Type objects.
        """
        functions = self.registry.get(name, None)
        if functions is None:
            return None

        for function in functions:
            if uri != function.uri:
                continue

            # is_same_type handles both strings and Type objects
            if not isinstance(
                return_type, SubstraitError
            ) and not self.is_same_type(function.return_type, return_type):
                continue
            if function.args == args:
                return function
            if len(function.args) != len(args) and not (
                function.variadic and len(args) >= len(function.args)
            ):
                continue
            is_match = True
            for i, arg in enumerate(args):
                j = (
                    i
                    if i < len(function.args)
                    else len(function.args) - 1
                )
                # is_same_type handles both strings and Type objects
                if not self.is_same_type(function.args[j], arg):
                    is_match = False
                    break
            if is_match:
                return function
        return None

    def get_extension_list(self):
        return list(self.extensions)

    def fill_coverage(self, coverage):
        for func_name, functions in self.registry.items():
            for function in functions:
                coverage.update_coverage(
                    function.uri, func_name, function.args, function.test_count
                )
