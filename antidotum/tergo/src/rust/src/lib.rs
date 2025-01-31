use extendr_api::prelude::*;
use tergo_lib::{Config, FunctionLineBreaks};

/// Format code
///
/// @param source_code (`character`) the R code to format
///
/// @return (`character`) the formatted code
/// @keywords internal
#[extendr]
#[allow(clippy::too_many_arguments)]
fn format_code(source_code: &str, configuration: extendr_api::List) -> extendr_api::List {
    let configuration = configuration.into_hashmap();
    let default_config = Config::default();
    let config = Config::new(
        configuration
            .get("indent")
            .map(|x| x.as_integer().expect("The indent must be an integer"))
            .unwrap_or(default_config.indent.0),
        configuration
            .get("line_length")
            .map(|x| x.as_integer().expect("The line_length must be an integer"))
            .unwrap_or(default_config.line_length.0),
        configuration
            .get("embracing_op_no_nl")
            .map(|x| {
                x.as_bool()
                    .expect("The embracing_op_no_nl must be a boolean")
            })
            .unwrap_or(default_config.embracing_op_no_nl.0),
        configuration
            .get("allow_nl_after_assignment")
            .map(|x| {
                x.as_bool()
                    .expect("The allow_nl_after_assignment must be a boolean")
            })
            .unwrap_or(default_config.allow_nl_after_assignment.0),
        configuration
            .get("space_before_complex_rhs_in_formula")
            .map(|x| {
                x.as_bool()
                    .expect("The space_before_complex_rhs_in_formula must be a boolean")
            })
            .unwrap_or(default_config.space_before_complex_rhs_in_formula.0),
        configuration
            .get("strip_suffix_whitespace_in_function_defs")
            .map(|x| {
                x.as_bool()
                    .expect("The strip_suffix_whitespace_in_function_defs must be a boolean")
            })
            .unwrap_or(default_config.strip_suffix_whitespace_in_function_defs.0),
        configuration
            .get("function_line_breaks")
            .map(|x| {
                match x
                    .as_str()
                    .expect("The function_line_breaks must be character")
                {
                    "single" => FunctionLineBreaks::Single,
                    "double" => FunctionLineBreaks::Double,
                    "hanging" => FunctionLineBreaks::Hanging,
                    _ => panic!("Unknown function line breaks. Allowed: single, double, hanging."),
                }
            })
            .unwrap_or(default_config.function_line_breaks),
        configuration
            .get("insert_newline_in_quote_call")
            .map(|x| {
                x.as_bool()
                    .expect("The insert_newline_in_quote_call must be a boolean")
            })
            .unwrap_or(default_config.insert_newline_in_quote_call.0),
    );

    // Check for special inline comments to skip formatting
    let mut skip_formatting = false;
    let mut formatted_code = String::new();
    for line in source_code.lines() {
        if line.trim().starts_with("# tergo-off") {
            skip_formatting = true;
        } else if line.trim().starts_with("# tergo-on") {
            skip_formatting = false;
        }

        if skip_formatting {
            formatted_code.push_str(line);
            formatted_code.push('\n');
        } else {
            match tergo_lib::tergo_format(line, Some(&config)) {
                Ok(formatted_line) => {
                    formatted_code.push_str(&formatted_line);
                    formatted_code.push('\n');
                }
                Err(error) => {
                    return list!("error", error);
                }
            }
        }
    }

    list!("success", formatted_code)
}

/// Parse the config file and return the configuration
///
/// @param path (`character(1)`) the file path of the configuration
/// file
///
/// @return (`list`)
/// @keywords internal
#[extendr]
fn get_config(path: &str) -> extendr_api::List {
    match std::fs::read_to_string(path) {
        Ok(config_file) => {
            let config: Config = toml::from_str(&config_file).unwrap_or_else(|_| Config::default());
            list!(
                indent = config.indent.0,
                line_length = config.line_length.0,
                embracing_op_no_nl = config.embracing_op_no_nl.0,
                allow_nl_after_assignment = config.allow_nl_after_assignment.0,
                space_before_complex_rhs_in_formula = config.space_before_complex_rhs_in_formula.0,
                strip_suffix_whitespace_in_function_defs =
                    config.strip_suffix_whitespace_in_function_defs.0,
                function_line_breaks = match config.function_line_breaks {
                    FunctionLineBreaks::Hanging => "hanging",
                    FunctionLineBreaks::Double => "double",
                    FunctionLineBreaks::Single => "single",
                },
                insert_newline_in_quote_call = config.insert_newline_in_quote_call.0
            )
        }
        Err(_) => {
            let config = Config::default();
            list!(
                indent = config.indent.0,
                line_length = config.line_length.0,
                embracing_op_no_nl = config.embracing_op_no_nl.0,
                allow_nl_after_assignment = config.allow_nl_after_assignment.0,
                space_before_complex_rhs_in_formula = config.space_before_complex_rhs_in_formula.0,
                strip_suffix_whitespace_in_function_defs =
                    config.strip_suffix_whitespace_in_function_defs.0,
                function_line_breaks = match config.function_line_breaks {
                    FunctionLineBreaks::Hanging => "hanging",
                    FunctionLineBreaks::Double => "double",
                    FunctionLineBreaks::Single => "single",
                },
                insert_newline_in_quote_call = config.insert_newline_in_quote_call.0
            )
        }
    }
}

/// Get the default configuration
///
/// This configuration is used by the styling functions
/// if no value is provided for the configuration keys.
///
/// @return `list` with the default configuration
/// @export
/// @examples
/// get_default_config()
#[extendr]
fn get_default_config() -> extendr_api::List {
    let config = Config::default();
    list!(
        indent = config.indent.0,
        line_length = config.line_length.0,
        embracing_op_no_nl = config.embracing_op_no_nl.0,
        allow_nl_after_assignment = config.allow_nl_after_assignment.0,
        space_before_complex_rhs_in_formula = config.space_before_complex_rhs_in_formula.0,
        strip_suffix_whitespace_in_function_defs =
            config.strip_suffix_whitespace_in_function_defs.0,
        function_line_breaks = match config.function_line_breaks {
            FunctionLineBreaks::Hanging => "hanging",
            FunctionLineBreaks::Double => "double",
            FunctionLineBreaks::Single => "single",
        },
        insert_newline_in_quote_call = config.insert_newline_in_quote_call.0
    )
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod tergo;
    fn format_code;
    fn get_config;
    fn get_default_config;
}
