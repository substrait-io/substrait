# SPDX-License-Identifier: Apache-2.0
import os
import yaml

from tests.coverage.antlr_parser.FuncTestCaseLexer import FuncTestCaseLexer
from tests.coverage.nodes import SubstraitError, type_str_is_outer_nullable

enable_debug = False


def error(msg):
    print(f"ERROR: {msg}")


def debug(msg):
    if enable_debug:
        print(f"DEBUG: {msg}")


def substrait_type_str(rule_num):
    return FuncTestCaseLexer.symbolicNames[rule_num].lower()


def find_extension_files(dir_path: str):
    """Paths of the function extension YAML files under dir_path, sorted."""
    extensions = []
    for root, _dirs, files in os.walk(dir_path):
        for file in files:
            if file.endswith(".yaml") and file.startswith("functions_"):
                extensions.append(os.path.join(root, file))

    extensions.sort()
    return extensions


FUNCTION_SECTIONS = ("scalar_functions", "aggregate_functions", "window_functions")


def declared_type(type_str):
    """The concrete type of a declaration, dropping any derivation expression.

    A ``return`` may be a multi-line derivation expression whose leading lines
    assign intermediate values.  Those lines can contain a ``?`` that is a
    ternary operator rather than a nullability marker (e.g.
    ``scale = init_prec > 38 ? scale_after_borrow : init_scale``), so only the
    final line names the declared type.
    """
    return str(type_str).strip().split("\n")[-1].strip()


def validate_impl_nullability_markers(impl, loc):
    """Validate that one impl carries no meaningless nullability markers.

    Under ``MIRROR`` the output nullability is derived from the arguments, so a
    ``?`` on an argument or on the return type is ignored.  Under
    ``DECLARED_OUTPUT`` the return marker is authoritative, but the outermost
    argument nullability is still stripped before binding.  A marker that is
    ignored is misleading, so it must not appear.  ``DISCRETE`` is exempt: there
    markers are meaningful on both sides.

    Only the outermost nullability is checked; nested markers are meaningful
    (e.g. ``func<any1 -> boolean?>``, ``list<i32?>``).

    Returns a list of error strings (empty if everything is valid).
    """
    nullability = impl.get("nullability", "MIRROR")  # MIRROR is the spec default
    if nullability == "DISCRETE":
        return []

    errors = []
    for arg in impl.get("args") or []:
        value = arg.get("value")
        if value is None:
            continue  # option (enum) argument, not a value
        if type_str_is_outer_nullable(declared_type(value)):
            errors.append(
                f"{loc}: argument '{arg.get('name', value)}' is declared '{value}' "
                f"under {nullability} nullability, but the outermost nullability of "
                f"an argument is stripped before binding; remove the '?'"
            )

    return_type = impl.get("return")
    if (
        nullability == "MIRROR"
        and return_type is not None
        and type_str_is_outer_nullable(declared_type(return_type))
    ):
        errors.append(
            f"{loc}: return type '{declared_type(return_type)}' carries a nullability "
            f"marker under MIRROR nullability, where output nullability is derived "
            f"from the arguments; remove the '?'"
        )
    return errors


def validate_nullability_markers(dir_path: str):
    """Validate every function declaration under dir_path.

    See :func:`validate_impl_nullability_markers` for the rules applied.

    Returns a list of error strings (empty if everything is valid).
    """
    errors = []
    for path in find_extension_files(dir_path):
        with open(path, "r") as fh:
            data = yaml.load(fh, Loader=yaml.FullLoader)
        for section in FUNCTION_SECTIONS:
            for func in data.get(section) or []:
                loc = f"{os.path.basename(path)}: {func['name']}"
                for impl in func.get("impls") or []:
                    errors.extend(validate_impl_nullability_markers(impl, loc))
    return errors


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
        FuncTestCaseLexer.Date: FuncTestCaseLexer.Date,
        FuncTestCaseLexer.Interval_Year: FuncTestCaseLexer.IYear,
        FuncTestCaseLexer.Interval_Day: FuncTestCaseLexer.IDay,
        FuncTestCaseLexer.Interval_Compound: FuncTestCaseLexer.ICompound,
        FuncTestCaseLexer.UUID: FuncTestCaseLexer.UUID,
        FuncTestCaseLexer.FixedChar: FuncTestCaseLexer.FChar,
        FuncTestCaseLexer.VarChar: FuncTestCaseLexer.VChar,
        FuncTestCaseLexer.FixedBinary: FuncTestCaseLexer.FBin,
        FuncTestCaseLexer.Decimal: FuncTestCaseLexer.Dec,
        FuncTestCaseLexer.Precision_Time: FuncTestCaseLexer.PT,
        FuncTestCaseLexer.Precision_Timestamp: FuncTestCaseLexer.PTs,
        FuncTestCaseLexer.Precision_Timestamp_TZ: FuncTestCaseLexer.PTsTZ,
        FuncTestCaseLexer.Struct: FuncTestCaseLexer.Struct,
        FuncTestCaseLexer.List: FuncTestCaseLexer.List,
        FuncTestCaseLexer.Map: FuncTestCaseLexer.Map,
        FuncTestCaseLexer.Any: FuncTestCaseLexer.Any,
        FuncTestCaseLexer.Func: FuncTestCaseLexer.Func,
    }
    to_short_type = {
        substrait_type_str(k): substrait_type_str(v) for k, v in rule_map.items()
    }
    any_type = substrait_type_str(FuncTestCaseLexer.Any)
    for i in range(1, 3):
        to_short_type[f"{any_type}{i}"] = f"{any_type}{i}"
    # Add enum type
    to_short_type["enum"] = "enum"
    return to_short_type


type_to_short_type = build_type_to_short_type()
short_type_to_type = {st: lt for lt, st in type_to_short_type.items()}


class Extension:
    @staticmethod
    def get_base_uri():
        return "https://github.com/substrait-io/substrait/blob/main/extensions/"

    @staticmethod
    def get_short_type(long_type):
        long_type = long_type.lower().rstrip("?")

        # Handle "enum" type specially
        if long_type == "enum":
            return "enum"

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
                        args.append("enum")
            nullability = impl.get(
                "nullability", "MIRROR"
            )  # MIRROR is the spec default
            return_type_raw = str(impl["return"])
            is_return_nullable = type_str_is_outer_nullable(return_type_raw)
            overloads.append(
                FunctionOverload(
                    args,
                    Extension.get_short_type(impl["return"]),
                    "variadic" in impl,
                    nullability=nullability,
                    is_return_nullable=is_return_nullable,
                )
            )
        return overloads

    @staticmethod
    def add_functions_to_map(func_list, function_map, suffix, extension, urn):
        dup_idx = 0
        for func in func_list:
            name = func["name"]
            if name in function_map:
                debug(
                    f"Duplicate function name: {name} renaming to {name}_{suffix} extension: {extension}"
                )
                dup_idx += 1
                name = f"{name}_dup{dup_idx}_{suffix}"
                assert name not in function_map, (
                    f"Duplicate function name: {name} renaming to {name}_{suffix} extension: {extension}"
                )
            func["overloads"] = Extension.get_supported_kernels_from_impls(func)
            func["urn"] = urn
            func.pop("description", None)
            func.pop("impls", None)
            function_map[name] = func

    @staticmethod
    def read_substrait_extensions(dir_path: str):
        extensions = find_extension_files(dir_path)

        scalar_functions = {}
        aggregate_functions = {}
        window_functions = {}
        dependencies = {}
        registered_urns = set()
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
                urn = data.get("urn")
                if urn:
                    registered_urns.add(urn)
                if "scalar_functions" in data:
                    Extension.add_functions_to_map(
                        data["scalar_functions"],
                        scalar_functions,
                        suffix,
                        extension,
                        urn,
                    )
                if "aggregate_functions" in data:
                    Extension.add_functions_to_map(
                        data["aggregate_functions"],
                        aggregate_functions,
                        suffix,
                        extension,
                        urn,
                    )
                if "window_functions" in data:
                    Extension.add_functions_to_map(
                        data["window_functions"],
                        window_functions,
                        suffix,
                        extension,
                        urn,
                    )

        return FunctionRegistry(
            scalar_functions,
            aggregate_functions,
            window_functions,
            dependencies,
            registered_urns,
        )


class FunctionType:
    SCALAR = 1
    AGGREGATE = 2
    WINDOW = 3


class FunctionVariant:
    def __init__(
        self,
        name,
        urn,
        description,
        args,
        return_type,
        variadic,
        func_type,
        nullability="MIRROR",
        is_return_nullable=False,
    ):
        self.name = name
        self.urn = urn
        self.description = description
        self.args = args
        self.return_type = return_type
        self.variadic = variadic
        self.func_type = func_type
        self.nullability = nullability
        self.is_return_nullable = is_return_nullable
        self.test_count = 0

    def __str__(self):
        return f"Function(name={self.name}, urn={self.urn}, description={self.description}, overloads={self.overload}, args={self.args}, result={self.result})"

    def increment_test_count(self, count=1):
        self.test_count += count


class FunctionOverload:
    def __init__(
        self,
        args,
        return_type,
        variadic,
        nullability="MIRROR",
        is_return_nullable=False,
    ):
        self.args = args
        self.return_type = return_type
        self.variadic = variadic
        self.nullability = nullability
        self.is_return_nullable = is_return_nullable

    def __str__(self):
        return f"FunctionOverload(args={self.args}, result={self.return_type}, variadic={self.variadic}, nullability={self.nullability}, is_return_nullable={self.is_return_nullable})"


# define function type enum


class FunctionRegistry:
    def __init__(
        self,
        scalar_functions,
        aggregate_functions,
        window_functions,
        dependencies,
        registered_urns=None,
    ):
        self.registry = {}
        self.dependencies = dependencies
        self.scalar_functions = scalar_functions
        self.aggregate_functions = aggregate_functions
        self.window_functions = window_functions
        self.registered_urns = registered_urns or set()
        self.add_functions(scalar_functions, FunctionType.SCALAR)
        self.add_functions(aggregate_functions, FunctionType.AGGREGATE)
        self.add_functions(window_functions, FunctionType.WINDOW)

    def add_functions(self, functions, func_type):
        for func in functions.values():
            f_name = func["name"]
            fun_arr = self.registry.get(f_name, [])
            for overload in func["overloads"]:
                function = FunctionVariant(
                    func["name"],
                    func["urn"],
                    "",
                    overload.args,
                    overload.return_type,
                    overload.variadic,
                    func_type,
                    nullability=overload.nullability,
                    is_return_nullable=overload.is_return_nullable,
                )
                fun_arr.append(function)
            self.registry[f_name] = fun_arr

    def validate_urn(self, urn):
        """Validate that an extension URN is registered.

        Args:
            urn: A URN (e.g., 'extension:io.substrait:functions_arithmetic')

        Raises:
            ValueError: If the URN is not registered
        """
        if urn not in self.registered_urns:
            raise ValueError(
                f"Unknown extension URN: {urn}. "
                f"Valid URNs are: {', '.join(sorted(self.registered_urns))}"
            )

    @staticmethod
    def is_type_any(func_arg_type):
        return func_arg_type[:3] == "any"

    @staticmethod
    def is_same_type(func_arg_type, arg_type):
        arg_type_base = arg_type.split("<")[0].rstrip("?")
        if func_arg_type == arg_type_base:
            return True
        return FunctionRegistry.is_type_any(func_arg_type)

    def get_function(
        self, name: str, urn: str, args: object, return_type
    ) -> [FunctionVariant]:
        functions = self.registry.get(name, None)
        if functions is None:
            return None
        for function in functions:
            if urn != function.urn:
                continue
            if not isinstance(return_type, SubstraitError) and not self.is_same_type(
                function.return_type, return_type
            ):
                continue
            if function.args == args:
                return function
            if len(function.args) != len(args) and not (
                function.variadic and len(args) >= len(function.args)
            ):
                continue
            is_match = True
            for i, arg in enumerate(args):
                j = i if i < len(function.args) else len(function.args) - 1
                if not self.is_same_type(function.args[j], arg):
                    is_match = False
                    break
            if is_match:
                return function
        return None

    def get_extension_list(self):
        return list(self.registered_urns)

    def fill_coverage(self, coverage):
        for func_name, functions in self.registry.items():
            for function in functions:
                coverage.update_coverage(
                    function.urn, func_name, function.args, function.test_count
                )
