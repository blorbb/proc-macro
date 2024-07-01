let macro_name = variable::get("macro_name");

if variable::get("macro_type") == "Derive" {
    // should be upper snake case
    if macro_name != to_upper_camel_case(macro_name) {
        print(`WARN: The macro name ${macro_name} is not in UpperCamelCase.`);
        print("Derive macros usually have an upper camel case name.");
    }
} else {
    // attribute or function like: snake case
    if macro_name != to_snake_case(macro_name) {
        print(`WARN: The macro name ${macro_name} is not in snake_case.`);
        print("Non-derive macros usually have a snake case name.");
    }
}
