import os
from itertools import cycle
from pathlib import Path

import oyaml as yaml
from mdutils.mdutils import MdUtils


def write_markdown(file_obj: dict) -> None:
    for function_classification, value in file_obj.items():
        mdFile.new_header(level=1, title=f"{function_classification}")
        functions_list = yaml_file_object[function_classification]

        for function_spec in functions_list:
            function_name = function_spec["name"]
            mdFile.new_header(level=2, title=f"{function_name}")
            if "description" in function_spec:
                description = function_spec["description"]
                mdFile.new_paragraph("DESCRIPTION:")
                mdFile.new_line(f"{description}")

            """
            Write markdown for implementations.

            If names for the function arguments are provided, show function signature with
            the argument names.  Function signature will also include optional arguments.
            """

            EXAMPLE_IMPL = False
            mdFile.new_paragraph("<details><summary>IMPLEMENTATIONS:</summary>")
            mdFile.write("\n")
            implementations_list = function_spec["impls"]
            option_names_list = []
            document_option_names_list = []
            options_list = []

            for count, impls in enumerate(implementations_list):
                args_list = impls["args"]
                arg_string = []
                arg_names = []
                arg_descriptions = []

                # For each function implementation, collect details on the following:
                #
                # Argument values:
                #   values, value names, description
                # Options:
                #   options, option names, required
                for arg in args_list:
                    if "value" in arg:
                        arg_string.append(arg["value"])
                        if "name" in arg:
                            arg_names.append(arg["name"])
                        if "description" in arg:
                            arg_descriptions.append(arg["description"])
                    if "options" in arg:
                        options = str(arg["options"])

                        # Options with no defined name, will be named as `name_placeholder`
                        if "name" in arg:
                            option_name = str(arg["name"])
                        else:
                            option_name = "name_placeholder"
                        document_option_names_list.append(option_name)

                        # Required options will be prepended with `req_enum` inside the function
                        # implementation. Optional options will be prepended with `opt_enum`
                        # inside the function implementation.
                        if "required" in arg and arg["required"]:
                            option_name = f"req_enum:{option_name}"
                        else:
                            option_name = f"opt_enum:{option_name}"
                        option_names_list.append(option_name)
                        options_list.append(options)

                # If the implementation is variadic, the last argument will appear `min_args`,
                # number of times in the implementation.
                if "variadic" in impls:
                    min_args = impls["variadic"]["min"]
                    for count in range(min_args - 1):
                        arg_string.append(arg_string[-1])
                        if len(arg_names) > 0:
                            arg_names.append(arg_names[-1])

                document_option_names_list = list(
                    dict.fromkeys(document_option_names_list)
                )
                option_names_list = list(dict.fromkeys(option_names_list))
                options_list = list(dict.fromkeys(options_list))
                arg_values = [
                    f"{x.replace('<','&lt').replace('>','&gt')}"
                    for x in arg_string + option_names_list
                ]
                arg_names_and_options = [
                    f"{x}" for x in arg_names + option_names_list
                ]
                # reset the options names list for the next function implementation.
                option_names_list = []
                func_concat_arg_input_names = ", ".join(arg_names_and_options)
                func_concat_arg_input_values = ", ".join(arg_values)

                # Only provide an example implementation using the argument names if argument
                # names are provided and an example implementation doesn't already exist.
                if len(arg_names) > 0 and not EXAMPLE_IMPL:
                    mdFile.new_line(
                        f"{function_name}({func_concat_arg_input_names}): -> `return_type` "
                    )
                    for arg_name, arg_desc in zip(arg_names, arg_descriptions):
                        mdFile.new_line(f"<li>{arg_name}: {arg_desc}</li>")
                    EXAMPLE_IMPL = True

                # If the return value for the function implementation is multiple lines long,
                # print each line separately. This is the case for some functions in
                # functions_arithmetic_decimal.yaml
                if "\n" in impls["return"]:
                    mdFile.new_line(
                        f"{count}. {function_name}({func_concat_arg_input_values}): -> "
                    )
                    multiline_return_str = "\t" + impls["return"]
                    multiline_return_str = multiline_return_str.replace("\n", "\n\t")
                    mdFile.new_line("\t```")
                    mdFile.new_line(f"{multiline_return_str}")
                    mdFile.new_line("\t```")
                else:
                    mdFile.new_line(
                        f"<br> {count}. {function_name}({func_concat_arg_input_values}): -> "
                        f"{impls['return'].replace('<','&lt').replace('>','&gt')} </br> "
                    )
            mdFile.new_paragraph("</details>")
            mdFile.write("\n")

            """
            Write markdown for options.
            """
            if len(options_list) > 0:
                mdFile.new_paragraph("<details><summary>OPTIONS:</summary>")
                mdFile.write("\n")
                A = options_list
                B = document_option_names_list
                for options_list, option_name in (
                    zip(A, cycle(B)) if len(A) > len(B) else zip(cycle(A), B)
                ):
                    mdFile.new_line(f"<li>{option_name} {options_list} </li> ")

                mdFile.new_paragraph("</details>")
                mdFile.write("\n")


current_file = Path(__file__).name
cur_path = Path(__file__).resolve()
functions_folder = os.path.join(str(Path(cur_path).parents[3]), "extensions")

# Get a list of all the function yaml files
function_files = []
for file in os.listdir(functions_folder):
    if file.startswith("functions"):
        full_path = os.path.join(functions_folder, file)
        function_files.append(full_path)

for function_file in function_files:
    with open(function_file) as file:
        yaml_file_object = yaml.load(file, Loader=yaml.FullLoader)

    function_file_name = os.path.basename(function_file)
    function_file_no_extension = os.path.splitext(function_file_name)[0]
    function_category = function_file_no_extension.split("_")[-1]
    mdFile = MdUtils(
        file_name=f"{function_file_no_extension}",
        title=f"{function_category} functions",
    )
    mdFile.new_paragraph(
        "This document file is generated for "
        + mdFile.new_inline_link(
            link=f"https://github.com/substrait-io/substrait/tree/main/extensions/"
            f"{function_file_name}",
            text=f"{function_file_name}",
        )
    )

    mdFile.new_paragraph(
        "Updating this document with the latest yaml can be done by running: "
        + mdFile.new_inline_link(
            link=f"https://github.com/substrait-io/substrait/tree/main/site/docs/functions/"
            f"{current_file}",
            text="generate_function_docs.py",
        )
    )

    write_markdown(yaml_file_object)

    mdFile.new_table_of_contents(table_title="Table of Contents", depth=2)
    mdFile.create_md_file()
