#![allow(non_upper_case_globals)]
use sloe_compile::{self as sloe, WithStartPosition, syntax_name_range};

struct State<Expressions, Patterns, Types> {
    projects: std::collections::HashMap<lsp_types::Uri, ProjectState<Expressions, Patterns, Types>>,
    syntax_expressions:
        sloe::core::Vec<Expressions, sloe::SyntaxExpression<Expressions, Patterns, Types>>,
    syntax_patterns: sloe::core::Vec<Patterns, sloe::SyntaxPattern<Patterns, Types>>,
    syntax_types: sloe::core::Vec<Types, sloe::SyntaxType<Types>>,
}
struct ProjectState<Expressions, Patterns, Types> {
    source: String,
    syntax: sloe::SyntaxProject<Expressions, Patterns, Types>,
    type_aliases: std::collections::HashMap<sloe::Name, sloe::CompiledTypeAliasInfo>,
    choice_types: std::collections::HashMap<sloe::Name, sloe::CompiledChoiceTypeInfo>,
    fns: std::collections::HashMap<sloe::Name, sloe::CompiledProjectFnInfo>,
    records: std::collections::HashSet<Vec<sloe::Name>>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut full_command = std::env::args().skip(1);
    match full_command.next() {
        None => {
            // consider a help message instead
            lsp_main()
        }
        Some(command) => match command.as_str() {
            "lsp" | "language-server" | "ls" => lsp_main(),
            "help" | "-h" | "--help" | "elp" | "h" | "pad" | "?" | "-?" | "--h" | "--?" => {
                println!("{command_help}");
                Ok(())
            }
            "build" | "make" | "compile" | "transpile" | "b" | "m" | "c" => {
                let maybe_input_file_path: Option<String> = full_command.next();
                let maybe_output_file_path: Option<String> = full_command.next();
                build_main(
                    maybe_input_file_path.as_ref().map(std::path::Path::new),
                    maybe_output_file_path.as_ref().map(std::path::Path::new),
                );
                Ok(())
            }
            "doc" | "docs" | "documentation" | "core" | "stdlib" | "core-doc" | "core-docs"
            | "core-documentation" | "core-types" | "d" => {
                println!("Here are all core declarations:\n");
                // TODO instead print core_fns and core_choice_types
                print_core_docs();
                Ok(())
            }
            "init" | "initialize" | "new" | "create" | "setup" | "boilerplate" | "template"
            | "hello" | "hello-world" => {
                println!(
                    "Each project has one .sloe file. For applications, a rust project is also needed. Both will be initialized now."
                );
                if full_command.next().is_some() {
                    println!(
                        "Nothing was created. If you want to initialize a sloe project in a directory, please create that directory yourself and run sloe init from inside there."
                    );
                    return Ok(());
                }
                initialize_new_sloe_hello_world_project();
                Ok(())
            }
            _ => {
                println!("Unknown command name.\n{command_help}");
                Ok(())
            }
        },
    }
}
const command_help: &str = "\
To compile to a rust file: sloe build [input-file.sloe [output-file.rs]]
To copy the hello-world project setup into the current directory: sloe init
To start the language server: sloe lsp
To print core declaration documentation: sloe core-docs
To run a rust project: cargo run
To compile a rust project into an executable: cargo build --release
To print this help message: sloe help
See the source code, see the full documentation, report bugs or leave any kind of feedback at https://codeberg.org/lue-bird/sloe";

fn print_core_docs() {
    for (core_choice_type_name, core_choice_type) in sloe::core_choice_types.iter() {
        println!(
            "{}",
            present_choice_type_markdown(core_choice_type_name, core_choice_type)
        );
    }
    for (core_fn_name, core_fn) in sloe::core_fns.iter() {
        println!(
            "{}",
            present_project_fn_with_complete_type_markdown(core_fn_name, core_fn)
        );
    }
}
fn documentation_comment_to_markdown(documentation: &str) -> String {
    let markdown_source: &str = documentation.trim();
    let mut markdown: String = String::new();
    markdown_convert_code_blocks_to_sloe_into(&mut markdown, markdown_source);
    markdown
}

fn markdown_convert_code_blocks_to_sloe_into(builder: &mut String, markdown_source: &str) {
    // because I don't want to introduce a full markdown parser for just this tiny
    // improvement, the code below only approximates where code blocks are.
    let mut with_fenced_code_blocks_converted: String = String::new();
    markdown_convert_unspecific_fenced_code_blocks_to_sloe_into(
        &mut with_fenced_code_blocks_converted,
        markdown_source,
    );
    markdown_convert_indented_code_blocks_to_sloe(builder, &with_fenced_code_blocks_converted);
}
/// replace fenced no-language-specified code blocks by `sloe...`
fn markdown_convert_unspecific_fenced_code_blocks_to_sloe_into(
    result_builder: &mut String,
    markdown_source: &str,
) {
    let mut current_source_index: usize = 0;
    'converting_fenced: while current_source_index < markdown_source.len() {
        match markdown_source[current_source_index..]
            .find("```")
            .map(|i| i + current_source_index)
        {
            None => {
                result_builder.push_str(&markdown_source[current_source_index..]);
                break 'converting_fenced;
            }
            Some(index_at_opening_fence) => {
                let index_after_opening_fence = index_at_opening_fence + 3;
                match markdown_source[index_after_opening_fence..]
                    .find("```")
                    .map(|i| i + index_after_opening_fence)
                {
                    None => {
                        result_builder.push_str(&markdown_source[current_source_index..]);
                        break 'converting_fenced;
                    }
                    Some(index_at_closing_fence) => {
                        match markdown_source[index_after_opening_fence..].chars().next() {
                            // fenced block without a specific language
                            Some('\n') => {
                                result_builder.push_str(
                                    &markdown_source[current_source_index..index_at_opening_fence],
                                );
                                result_builder.push_str("```sloe");
                                result_builder.push_str(
                                    &markdown_source
                                        [index_after_opening_fence..index_at_closing_fence],
                                );
                                result_builder.push_str("```");
                                current_source_index = index_at_closing_fence + 3;
                            }
                            // fenced block with a specific language
                            _ => {
                                result_builder.push_str(
                                    &markdown_source
                                        [current_source_index..(index_at_closing_fence + 3)],
                                );
                                current_source_index = index_at_closing_fence + 3;
                            }
                        }
                    }
                }
            }
        }
    }
}
fn markdown_convert_indented_code_blocks_to_sloe(builder: &mut String, markdown_source: &str) {
    let mut current_indent: usize = 0;
    let mut is_in_code_block: bool = false;
    let mut previous_line_was_blank: bool = false;
    for source_line in markdown_source.lines() {
        if source_line.is_empty() {
            builder.push('\n');
            previous_line_was_blank = true;
        } else {
            let current_line_indent: usize = source_line
                .chars()
                .take_while(char::is_ascii_whitespace)
                .count();
            if current_line_indent == source_line.len() {
                // ignore blank line
                builder.push_str(source_line);
                builder.push('\n');
                previous_line_was_blank = true;
            } else {
                if is_in_code_block {
                    if current_line_indent <= current_indent - 1 {
                        is_in_code_block = false;
                        current_indent = current_line_indent;
                        builder.push_str("```\n");
                        builder.push_str(source_line);
                        builder.push('\n');
                    } else {
                        builder.push_str(&source_line[current_indent..]);
                        builder.push('\n');
                    }
                } else if previous_line_was_blank && (current_line_indent >= current_indent + 4) {
                    is_in_code_block = true;
                    current_indent = current_line_indent;
                    builder.push_str("```sloe\n");
                    builder.push_str(&source_line[current_line_indent..]);
                    builder.push('\n');
                } else {
                    current_indent = current_line_indent;
                    builder.push_str(source_line);
                    builder.push('\n');
                }
                previous_line_was_blank = false;
            }
        }
    }
    if is_in_code_block {
        builder.push_str("```\n");
    }
}
fn present_project_fn_with_complete_type_markdown(
    name: &sloe::Name,
    fn_info: &sloe::CompiledProjectFnInfo,
) -> String {
    let mut type_parameters_string = String::new();
    angled_type_parameters_format(&mut type_parameters_string, &fn_info.type_parameters);
    let mut parameter_type_string: String = String::new();
    let mut result_type_string: String = String::new();
    if let Some(fn_parameter_type) = &fn_info.parameter_type {
        sloe::type_into(&mut parameter_type_string, 8, fn_parameter_type);
    }
    if let Some(fn_result_type) = &fn_info.result_type {
        sloe::type_into(&mut result_type_string, 8, fn_result_type);
    }
    format!(
        "```sloe
fn {}{} ({}) ({})
```
{}
",
        name,
        type_parameters_string,
        parameter_type_string,
        result_type_string,
        documentation_comment_to_markdown(fn_info.documentation.as_deref().unwrap_or(""))
    )
}
fn present_type_alias_markdown(
    name: &sloe::Name,
    type_alias_info: &sloe::CompiledTypeAliasInfo,
) -> String {
    let mut type_string: String = String::new();
    if let Some(type_) = &type_alias_info.type_ {
        sloe::type_into(&mut type_string, 4, type_);
    }
    let description = format!(
        "```sloe\ntype {} {} {}\n```\n",
        name,
        type_alias_info.parameters.join(" "),
        type_string
    );
    match &type_alias_info.documentation {
        None => description,
        Some(documentation) => {
            description + documentation_comment_to_markdown(documentation).as_str()
        }
    }
}
fn angled_type_parameters_format(formatted: &mut String, type_parameters: &[sloe::Name]) {
    if let Some((type_parameter0, type_parameter1_up)) = type_parameters.split_first() {
        formatted.push('<');
        formatted.push_str(type_parameter0);
        for type_parameter in type_parameter1_up {
            formatted.push(' ');
            formatted.push_str(type_parameter);
        }
        formatted.push('>');
    }
}
fn present_choice_type_markdown(maybe_name: &str, info: &sloe::CompiledChoiceTypeInfo) -> String {
    let mut variants_string: String = String::new();
    for variant in &info.variants {
        variants_string.push_str("\n    (");
        variants_string.push_str(&variant.name);
        angled_type_parameters_format(&mut variants_string, &variant.type_parameters);
        if let Some(variant_value) = &variant.value {
            variants_string.push(' ');
            sloe::type_into(&mut variants_string, 8, variant_value);
        }
        variants_string.push_str(")");
    }
    format!(
        "```sloe\nchoice {} {}{}\n```\n{}",
        maybe_name,
        info.parameters.join(" "),
        variants_string,
        documentation_comment_to_markdown(info.documentation.as_deref().unwrap_or(""))
    )
}

fn initialize_new_sloe_hello_world_project() {
    try_generate_file(
        "sloe.sloe",
        "this is where all your sloe code goes",
        r#"

greet \:str:name >
    strs-flatten [ "Hello, ", name, "\n" ]

"#,
    );
    try_generate_file(
        "main.rs",
        "the actual program entrypoint, written in rust.",
        r#"mod sloe;

fn main() {
    print!("{}", sloe::greet(sloe::Str::Slice("world")));
}
"#,
    );
    try_generate_file(
        "Cargo.toml",
        "this tells cargo (the rust package manager) how to build the project",
        r#"[package]
name = "example"
edition = "2024"
[[bin]]
name = "example"
path = "main.rs"
"#,
    );
    try_generate_file(
        ".gitignore",
        "this tells git to not track the generated rust code",
        r"# Generated rust code
sloe/
",
    );
    match std::fs::exists("sloe") {
        Ok(true) => {
            println!("sloe/ directory already exists, skipping generating it.");
        }
        Ok(false) => {
            let write_result: Result<(), std::io::Error> = std::fs::create_dir("sloe");
            match write_result {
                Ok(()) => {
                    println!(
                        "created sloe/ directory, this will contain the generated rust file sloe/mod.rs."
                    );
                }
                Err(error) => {
                    println!("failed to generate sloe/ directory: {error}");
                }
            }
        }
        Err(error) => {
            println!("failed to check if sloe/ directory already exists: {error}");
        }
    }
}
fn try_generate_file(path: &str, purpose: &str, content: &str) {
    match std::fs::exists(path) {
        Ok(true) => {
            println!("{path} already exists, skipping generating it.");
        }
        Ok(false) => {
            let write_result: Result<(), std::io::Error> = std::fs::write(path, content);
            match write_result {
                Ok(()) => {
                    println!("created {path}, {purpose}.");
                }
                Err(error) => {
                    println!("failed to generate {path}: {error}");
                }
            }
        }
        Err(error) => {
            println!("failed to check if {path} already exists: {error}");
        }
    }
}
fn default_sloe_output_file_path_for_input_file_path(
    input_file_path: &std::path::Path,
) -> std::path::PathBuf {
    std::path::Path::join(&input_file_path.with_extension(""), "mod.rs")
}

fn build_main(
    maybe_input_file_path: Option<&std::path::Path>,
    maybe_output_file_path: Option<&std::path::Path>,
) {
    let input_file_path: &std::path::Path = match maybe_input_file_path {
        Some(input_file_path) => &input_file_path.with_extension("sloe"),
        None => std::path::Path::new("sloe.sloe"),
    };
    let output_file_path: &std::path::Path = match maybe_output_file_path {
        Some(output_file_path) => &output_file_path.with_extension(".rs"),
        None => &default_sloe_output_file_path_for_input_file_path(input_file_path),
    };
    println!("...compiling {input_file_path:?} into {output_file_path:?}.");
    match std::fs::read_to_string(input_file_path) {
        Err(read_error) => {
            eprintln!(
                "was looking for a file with the name {input_file_path:?} but failed: {read_error}"
            );
            std::process::exit(1)
        }
        Ok(project_source) => {
            sloe::core::origin_new!(expressions, Expressions);
            sloe::core::origin_new!(patterns, Patterns);
            sloe::core::origin_new!(types, Types);
            let mut syntax_expressions = sloe::core::vec_empty(expressions);
            let mut syntax_patterns = sloe::core::vec_empty(patterns);
            let mut syntax_types = sloe::core::vec_empty(types);
            let syntax_project = sloe::parse_project(
                &mut syntax_expressions,
                &mut syntax_patterns,
                &mut syntax_types,
                &project_source,
            );
            let mut output_errors: Vec<sloe::ErrorNode> = Vec::new();
            let compiled_project: sloe::CompiledProject = sloe::project_compile_to_rust(
                &mut output_errors,
                &syntax_project,
                &syntax_expressions,
                &syntax_patterns,
                &syntax_types,
            );
            for output_error in output_errors.iter().rev() {
                eprintln!(
                    "{input_file_path}:{span_start_line}:{span_start_column} {message}",
                    input_file_path = input_file_path.to_string_lossy(),
                    span_start_line = output_error.range.start.line + 1,
                    span_start_column = output_error.range.start.character + 1,
                    message = &output_error.message
                );
            }
            let output_rust_file_string: String =
                sloe::compiled_rust_to_file_content(&compiled_project.rust);
            if let Some(output_file_directory_path) = output_file_path.parent()
                && let Err(error) = std::fs::create_dir_all(output_file_directory_path)
            {
                eprintln!(
                    "tried to create the directory containing the output rust file {output_file_path:?} but failed: {}",
                    error
                );
                std::process::exit(1)
            }
            match std::fs::write(output_file_path, output_rust_file_string) {
                Err(write_error) => {
                    eprintln!(
                        "tried to write the output into the rust file {output_file_path:?} but failed: {}",
                        write_error
                    );
                    std::process::exit(1)
                }
                Ok(()) => {
                    if !output_errors.is_empty() {
                        std::process::exit(1)
                    }
                }
            }
        }
    }
}
fn lsp_main() -> Result<(), Box<dyn std::error::Error>> {
    let (connection, io_thread) = lsp_server::Connection::stdio();

    let (initialize_request_id, _initialize_arguments_json) = connection.initialize_start()?;
    connection.initialize_finish(
        initialize_request_id,
        serde_json::to_value(lsp_types::InitializeResult {
            capabilities: server_capabilities(),
            server_info: Some(lsp_types::ServerInfo {
                name: "sloe".to_string(),
                version: Some("0.0.1".to_string()),
            }),
        })?,
    )?;
    sloe::core::origin_new!(expressions, Expressions);
    sloe::core::origin_new!(patterns, Patterns);
    sloe::core::origin_new!(types, Types);
    let state = initial_state(expressions, patterns, types);
    server_loop(&connection, state)?;
    // shut down gracefully
    drop(connection);
    io_thread.join()?;
    Ok(())
}
fn initial_state<Expressions, Patterns, Types>(
    expressions: Expressions,
    patterns: Patterns,
    types: Types,
) -> State<Expressions, Patterns, Types> {
    State {
        projects: std::collections::HashMap::with_capacity(1),
        syntax_expressions: sloe::core::vec_empty(expressions),
        syntax_patterns: sloe::core::vec_empty(patterns),
        syntax_types: sloe::core::vec_empty(types),
    }
}
fn server_capabilities() -> lsp_types::ServerCapabilities {
    lsp_types::ServerCapabilities {
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        semantic_tokens_provider: Some(
            lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions(
                lsp_types::SemanticTokensOptions {
                    work_done_progress_options: lsp_types::WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                    legend: lsp_types::SemanticTokensLegend {
                        token_modifiers: vec![],
                        token_types: Vec::from(token_types),
                    },
                    range: None,
                    full: Some(lsp_types::SemanticTokensFullOptions::Bool(true)),
                },
            ),
        ),
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::INCREMENTAL,
        )),
        rename_provider: Some(lsp_types::OneOf::Right(lsp_types::RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: lsp_types::WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
        references_provider: Some(lsp_types::OneOf::Left(true)),
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![".".to_string()]),
            all_commit_characters: None,
            work_done_progress_options: lsp_types::WorkDoneProgressOptions {
                work_done_progress: None,
            },
            completion_item: Some(lsp_types::CompletionOptionsCompletionItem {
                label_details_support: None,
            }),
        }),
        document_formatting_provider: Some(lsp_types::OneOf::Left(true)),
        document_symbol_provider: Some(lsp_types::OneOf::Left(true)),
        ..lsp_types::ServerCapabilities::default()
    }
}

fn server_loop<Expressions, Patterns, Types>(
    connection: &lsp_server::Connection,
    mut state: State<Expressions, Patterns, Types>,
) -> Result<(), Box<dyn std::error::Error>> {
    for client_message in &connection.receiver {
        match client_message {
            lsp_server::Message::Request(request) => {
                if connection.handle_shutdown(&request)? {
                    break;
                }
                if let Err(error) = handle_request(
                    connection,
                    &state,
                    request.id,
                    &request.method,
                    request.params,
                ) {
                    eprintln!("request {} failed: {error}", &request.method);
                }
            }
            lsp_server::Message::Notification(notification) => {
                if let Err(err) = handle_notification(
                    connection,
                    &mut state,
                    &notification.method,
                    notification.params,
                ) {
                    eprintln!("notification {} failed: {err}", notification.method);
                }
            }
            lsp_server::Message::Response(_) => {}
        }
    }
    Ok(())
}
fn handle_notification<Expressions, Patterns, Types>(
    connection: &lsp_server::Connection,
    state: &mut State<Expressions, Patterns, Types>,
    notification_method: &str,
    notification_arguments_json: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    match notification_method {
        <lsp_types::notification::DidOpenTextDocument as lsp_types::notification::Notification>::METHOD => {
            let arguments: <lsp_types::notification::DidOpenTextDocument as lsp_types::notification::Notification>::Params =
                serde_json::from_value(notification_arguments_json)?;
            update_state_on_did_open_text_document(state, connection, arguments);
        }
        <lsp_types::notification::DidCloseTextDocument as lsp_types::notification::Notification>::METHOD => {
            let arguments: <lsp_types::notification::DidCloseTextDocument as lsp_types::notification::Notification>::Params =
                serde_json::from_value(notification_arguments_json)?;
            publish_diagnostics(
                connection,
                lsp_types::PublishDiagnosticsParams {
                    uri: arguments.text_document.uri,
                    diagnostics: vec![],
                    version: None,
                },
            );
        }
        <lsp_types::notification::DidChangeTextDocument as lsp_types::notification::Notification>::METHOD => {
            let arguments: <lsp_types::notification::DidChangeTextDocument as lsp_types::notification::Notification>::Params =
                serde_json::from_value(notification_arguments_json)?;
            update_state_on_did_change_text_document(state, connection, arguments);
        }
        <lsp_types::notification::Exit as lsp_types::notification::Notification>::METHOD => {}
        _ => {}
    }
    Ok(())
}
fn update_state_on_did_open_text_document<Expressions, Patterns, Types>(
    state: &mut State<Expressions, Patterns, Types>,
    connection: &lsp_server::Connection,
    arguments: lsp_types::DidOpenTextDocumentParams,
) {
    if arguments.text_document.language_id == "sloe"
        || lsp_uri_to_file_path(&arguments.text_document.uri)
            .is_some_and(|file_path| file_path.extension().is_some_and(|ext| ext == "sloe"))
    {
        match state.projects.get_mut(&arguments.text_document.uri) {
            None => {
                state.projects.insert(
                    arguments.text_document.uri.clone(),
                    initialize_project_state_from_source(
                        connection,
                        arguments.text_document.uri,
                        &mut state.syntax_expressions,
                        &mut state.syntax_patterns,
                        &mut state.syntax_types,
                        arguments.text_document.text,
                    ),
                );
            }
            Some(previous_project_state) => {
                *previous_project_state = initialize_project_state_from_source(
                    connection,
                    arguments.text_document.uri,
                    &mut state.syntax_expressions,
                    &mut state.syntax_patterns,
                    &mut state.syntax_types,
                    arguments.text_document.text,
                );
            }
        }
    }
}

fn handle_request<Expressions, Patterns, Types>(
    connection: &lsp_server::Connection,
    state: &State<Expressions, Patterns, Types>,
    request_id: lsp_server::RequestId,
    request_method: &str,
    request_arguments_json: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let response: Result<serde_json::Value, lsp_server::ResponseError> = match request_method {
        <lsp_types::request::HoverRequest as lsp_types::request::Request>::METHOD => {
            let arguments: <lsp_types::request::HoverRequest as lsp_types::request::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let maybe_hover_result: <lsp_types::request::HoverRequest as lsp_types::request::Request>::Result =
                respond_to_hover(state, &arguments);
            Ok(serde_json::to_value(maybe_hover_result)?)
        }
        <lsp_types::request::GotoDefinition as lsp_types::request::Request>::METHOD => {
            let arguments: <lsp_types::request::GotoDefinition as lsp_types::request::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let maybe_hover_result: <lsp_types::request::GotoDefinition as lsp_types::request::Request>::Result =
                respond_to_goto_definition(state, arguments);
            Ok(serde_json::to_value(maybe_hover_result)?)
        }
        <lsp_types::request::PrepareRenameRequest as lsp_types::request::Request>::METHOD => {
            let prepare_rename_arguments: <lsp_types::request::PrepareRenameRequest as lsp_types::request::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let prepared: Option<
                Result<lsp_types::PrepareRenameResponse, lsp_server::ResponseError>,
            > = respond_to_prepare_rename(state, &prepare_rename_arguments);
            let response_result: Result<
                <lsp_types::request::PrepareRenameRequest as lsp_types::request::Request>::Result,
                lsp_server::ResponseError,
            > = match prepared {
                None => Ok(None),
                Some(result) => result.map(Some),
            };
            match response_result {
                Err(error) => Err(error),
                Ok(maybe_response) => Ok(serde_json::to_value(maybe_response)?),
            }
        }
        <lsp_types::request::Rename as lsp_types::request::Request>::METHOD => {
            let arguments: <lsp_types::request::Rename as lsp_types::request::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let maybe_rename_edits: Option<Vec<lsp_types::TextDocumentEdit>> =
                respond_to_rename(state, arguments);
            let result: <lsp_types::request::Rename as lsp_types::request::Request>::Result =
                maybe_rename_edits.map(|rename_edits| lsp_types::WorkspaceEdit {
                    changes: None,
                    document_changes: Some(lsp_types::DocumentChanges::Edits(rename_edits)),
                    change_annotations: None,
                });
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::request::References as lsp_types::request::Request>::METHOD => {
            let arguments: <lsp_types::request::References as lsp_types::request::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::request::References as lsp_types::request::Request>::Result =
                respond_to_references(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::request::SemanticTokensFullRequest as lsp_types::request::Request>::METHOD => {
            let arguments: <lsp_types::request::SemanticTokensFullRequest as lsp_types::request::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::request::SemanticTokensFullRequest as lsp_types::request::Request>::Result =
                respond_to_semantic_tokens_full(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::request::Completion as lsp_types::request::Request>::METHOD => {
            let arguments: <lsp_types::request::Completion as lsp_types::request::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::request::Completion as lsp_types::request::Request>::Result =
                respond_to_completion(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::request::Formatting as lsp_types::request::Request>::METHOD => {
            let arguments: <lsp_types::request::Formatting as lsp_types::request::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::request::Formatting as lsp_types::request::Request>::Result =
                respond_to_document_formatting(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::request::DocumentSymbolRequest as lsp_types::request::Request>::METHOD => {
            let arguments: <lsp_types::request::DocumentSymbolRequest as lsp_types::request::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::request::DocumentSymbolRequest as lsp_types::request::Request>::Result =
                respond_to_document_symbols(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::request::Shutdown as lsp_types::request::Request>::METHOD => {
            let result: <lsp_types::request::Shutdown as lsp_types::request::Request>::Result = ();
            Ok(serde_json::to_value(result)?)
        }
        _ => Err(lsp_server::ResponseError {
            code: lsp_server::ErrorCode::MethodNotFound as i32,
            message: "unhandled method".to_string(),
            data: None,
        }),
    };
    match response {
        Ok(response_value) => send_response_ok(connection, request_id, response_value),
        Err(response_error) => send_response_error(connection, request_id, response_error),
    }
}

fn send_response_ok(
    connection: &lsp_server::Connection,
    id: lsp_server::RequestId,
    result: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let response: lsp_server::Response = lsp_server::Response {
        id,
        result: Some(result),
        error: None,
    };
    connection
        .sender
        .send(lsp_server::Message::Response(response))
        .map_err(|err| err.into())
}
fn send_response_error(
    connection: &lsp_server::Connection,
    id: lsp_server::RequestId,
    error: lsp_server::ResponseError,
) -> Result<(), Box<dyn std::error::Error>> {
    let response: lsp_server::Response = lsp_server::Response {
        id,
        result: None,
        error: Some(error),
    };
    connection
        .sender
        .send(lsp_server::Message::Response(response))
        .map_err(|err| err.into())
}
fn publish_diagnostics(
    connection: &lsp_server::Connection,
    diagnostics: <lsp_types::notification::PublishDiagnostics as lsp_types::notification::Notification>::Params,
) {
    let diagnostics_json: serde_json::Value = match serde_json::to_value(diagnostics) {
        Ok(diagnostics_json) => diagnostics_json,
        Err(err) => {
            eprintln!("failed to encode diagnostics {err}");
            return;
        }
    };
    connection.sender.send(lsp_server::Message::Notification(
        lsp_server::Notification {
            method: <lsp_types::notification::PublishDiagnostics as lsp_types::notification::Notification>::METHOD.to_string(),
            params: diagnostics_json,
        },
    )).unwrap_or_else(|err| {
        eprintln!("failed to send diagnostics {err}");
    });
}

fn update_state_on_did_change_text_document<Expressions, Patterns, Types>(
    state: &mut State<Expressions, Patterns, Types>,
    connection: &lsp_server::Connection,
    did_change_text_document: lsp_types::DidChangeTextDocumentParams,
) {
    if let Some(project_state) = state
        .projects
        .get_mut(&did_change_text_document.text_document.uri)
    {
        let mut updated_source: String = std::mem::take(&mut project_state.source);
        for change in did_change_text_document.content_changes {
            match (change.range, change.range_length) {
                // means full replacement
                (None, None) => {
                    updated_source = change.text;
                }
                // zed for example does not send a span length
                (Some(span), None) => {
                    string_replace_lsp_span(&mut updated_source, span, &change.text);
                }
                // sending a span is deprecated but e.g. vscode still sends it
                // which allows us to do a faster string replace
                (Some(span), Some(span_length)) => {
                    string_replace_lsp_span_for_length(
                        &mut updated_source,
                        span,
                        span_length as usize,
                        &change.text,
                    );
                }
                (None, Some(_)) => {}
            }
        }
        *project_state = initialize_project_state_from_source(
            connection,
            did_change_text_document.text_document.uri,
            &mut state.syntax_expressions,
            &mut state.syntax_patterns,
            &mut state.syntax_types,
            updated_source,
        );
    }
}

fn initialize_project_state_from_source<Expressions, Patterns, Types>(
    connection: &lsp_server::Connection,
    uri: lsp_types::Uri,
    expressions: &mut sloe::core::Vec<
        Expressions,
        sloe::SyntaxExpression<Expressions, Patterns, Types>,
    >,
    patterns: &mut sloe::core::Vec<Patterns, sloe::SyntaxPattern<Patterns, Types>>,
    types: &mut sloe::core::Vec<Types, sloe::SyntaxType<Types>>,
    source: String,
) -> ProjectState<Expressions, Patterns, Types> {
    let parsed_project = sloe::parse_project(expressions, patterns, types, &source);
    let mut errors: Vec<sloe::ErrorNode> = Vec::new();
    let compiled_project: sloe::CompiledProject =
        sloe::project_compile_to_rust(&mut errors, &parsed_project, expressions, patterns, types);
    if let Some(input_file_path) = lsp_uri_to_file_path(&uri)
        && std::fs::exists(input_file_path.with_extension("")).is_ok_and(|exists| exists)
    {
        let _: std::io::Result<()> = std::fs::write(
            default_sloe_output_file_path_for_input_file_path(&input_file_path),
            sloe::compiled_rust_to_file_content(&compiled_project.rust),
        );
    }
    publish_diagnostics(
        connection,
        lsp_types::PublishDiagnosticsParams {
            uri,
            diagnostics: errors
                .iter()
                .map(sloe_error_node_to_diagnostic)
                .collect::<Vec<_>>(),
            version: None,
        },
    );
    ProjectState {
        source: source,
        type_aliases: compiled_project.type_aliases,
        choice_types: compiled_project.choice_types,
        fns: compiled_project.fns,
        records: compiled_project.records,
        syntax: parsed_project,
    }
}
fn sloe_error_node_to_diagnostic(problem: &sloe::ErrorNode) -> lsp_types::Diagnostic {
    lsp_types::Diagnostic {
        range: problem.range,
        severity: Some(lsp_types::DiagnosticSeverity::WARNING),
        code: None,
        code_description: None,
        source: None,
        message: problem.message.to_string(),
        related_information: None,
        tags: None,
        data: None,
    }
}

fn respond_to_hover<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    hover_arguments: &lsp_types::HoverParams,
) -> Option<lsp_types::Hover> {
    let Some(project_state) = state.projects.get(
        &hover_arguments
            .text_document_position_params
            .text_document
            .uri,
    ) else {
        return None;
    };
    let Some(symbol) = sloe::syntax_project_symbol_at_position(
        &project_state.syntax,
        hover_arguments.text_document_position_params.position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    match symbol {
        sloe::SyntaxSymbol::ProjectTypeOrUnknown {
            name: symbol_name,
            origins: _,
        } => project_state
            .choice_types
            .iter()
            .find_map(|(choice_type_name, choice_type_info)| {
                choice_type_info.variants.iter().find_map(|variant| {
                    if &variant.name == &symbol_name.value {
                        let choice_type_formatted =
                            present_choice_type_markdown(choice_type_name, choice_type_info);
                        Some(lsp_types::Hover {
                            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                                kind: lsp_types::MarkupKind::Markdown,
                                value: format!("variant in\n{}", choice_type_formatted),
                            }),
                            range: Some(syntax_name_range(symbol_name)),
                        })
                    } else {
                        None
                    }
                })
            })
            .or_else(|| {
                project_state
                    .type_aliases
                    .iter()
                    .find_map(|(type_alias_name, type_alias_info)| {
                        if type_alias_name == &symbol_name.value {
                            Some(lsp_types::Hover {
                                contents: lsp_types::HoverContents::Markup(
                                    lsp_types::MarkupContent {
                                        kind: lsp_types::MarkupKind::Markdown,
                                        value: present_type_alias_markdown(
                                            type_alias_name,
                                            type_alias_info,
                                        ),
                                    },
                                ),
                                range: Some(syntax_name_range(symbol_name)),
                            })
                        } else {
                            None
                        }
                    })
            }),
        sloe::SyntaxSymbol::Origin {
            name,
            use_start,
            origin: _,
        } => Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: format!(
                    "```sloe
origin {}
```",
                    name
                ),
            }),
            range: Some(syntax_name_range(WithStartPosition {
                start: use_start,
                value: name,
            })),
        }),
        sloe::SyntaxSymbol::TypeVariable {
            name,
            use_start,
            scope: _,
        } => Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: format!("type variable"),
            }),
            range: Some(syntax_name_range(WithStartPosition {
                start: use_start,
                value: name,
            })),
        }),
        sloe::SyntaxSymbol::VariantOrUnknown(symbol_name) => project_state
            .choice_types
            .iter()
            .find_map(|(choice_type_name, choice_type_info)| {
                choice_type_info.variants.iter().find_map(|variant| {
                    if &variant.name == &symbol_name.value {
                        Some(lsp_types::Hover {
                            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                                kind: lsp_types::MarkupKind::Markdown,
                                value: format!(
                                    "variant in\n{}",
                                    present_choice_type_markdown(
                                        choice_type_name,
                                        choice_type_info
                                    )
                                ),
                            }),
                            range: Some(syntax_name_range(symbol_name)),
                        })
                    } else {
                        None
                    }
                })
            }),
        sloe::SyntaxSymbol::ProjectFnOrUnknown {
            name: symbol_name,
            pattern_variables: _,
            origins: _,
        } => project_state.fns.iter().find_map(|(fn_name, fn_info)| {
            if fn_name == &symbol_name.value {
                let choice_type_formatted =
                    present_project_fn_with_complete_type_markdown(fn_name, fn_info);
                Some(lsp_types::Hover {
                    contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                        kind: lsp_types::MarkupKind::Markdown,
                        value: format!("variant in\n{}", choice_type_formatted),
                    }),
                    range: Some(syntax_name_range(symbol_name)),
                })
            } else {
                None
            }
        }),
        sloe::SyntaxSymbol::PatternVariable {
            name,
            use_start,
            origin: _,
        } => {
            // possible improvement: infer type
            Some(lsp_types::Hover {
                contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: format!("pattern variable"),
                }),
                range: Some(syntax_name_range(WithStartPosition {
                    start: use_start,
                    value: name,
                })),
            })
        }
    }
}

fn respond_to_goto_definition<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    goto_definition_arguments: lsp_types::GotoDefinitionParams,
) -> Option<lsp_types::GotoDefinitionResponse> {
    let Some(project_state) = state.projects.get(
        &goto_definition_arguments
            .text_document_position_params
            .text_document
            .uri,
    ) else {
        return None;
    };
    let Some(symbol) = sloe::syntax_project_symbol_at_position(
        &project_state.syntax,
        goto_definition_arguments
            .text_document_position_params
            .position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    let origin_name_range = match symbol {
        sloe::SyntaxSymbol::VariantOrUnknown(symbol_name) => project_state
            .syntax
            .elements
            .iter()
            .find_map(|element| match element {
                sloe::SyntaxProjectElement::ChoiceType {
                    choice_keyword_start: _,
                    name: _,
                    parameters: _,
                    documentation: _,
                    variants,
                } => variants.iter().find_map(|variant| {
                    if let Some(variant_name) = &variant.name
                        && &variant_name.value == &symbol_name.value
                    {
                        Some(syntax_name_range(sloe::with_start_position_as_ref(
                            variant_name,
                        )))
                    } else {
                        None
                    }
                }),
                _ => None,
            }),
        sloe::SyntaxSymbol::ProjectFnOrUnknown {
            name: symbol_name,
            pattern_variables: _,
            origins: _,
        } => project_state
            .syntax
            .elements
            .iter()
            .find_map(|element| match element {
                sloe::SyntaxProjectElement::Fn {
                    name: Some(fn_name),
                    ..
                } if &fn_name.value == &symbol_name.value => {
                    Some(syntax_name_range(sloe::with_start_position_as_ref(fn_name)))
                }
                _ => None,
            }),
        sloe::SyntaxSymbol::ProjectTypeOrUnknown {
            name: symbol_name,
            origins: _,
        } => project_state
            .syntax
            .elements
            .iter()
            .find_map(|element| match element {
                sloe::SyntaxProjectElement::TypeAlias {
                    name: Some(type_alias_name),
                    ..
                } if type_alias_name.value == &symbol_name.value => Some(syntax_name_range(
                    sloe::with_start_position_as_ref(type_alias_name),
                )),
                sloe::SyntaxProjectElement::ChoiceType {
                    name: Some(choice_type_name),
                    ..
                } if choice_type_name.value == &symbol_name.value => Some(syntax_name_range(
                    sloe::with_start_position_as_ref(choice_type_name),
                )),
                _ => None,
            }),
        sloe::SyntaxSymbol::Origin {
            name,
            use_start: _,
            origin,
        } => Some(syntax_name_range(sloe::WithStartPosition {
            value: name,
            start: origin.start,
        })),
        sloe::SyntaxSymbol::TypeVariable { .. } => None,
        sloe::SyntaxSymbol::PatternVariable {
            name,
            use_start: _,
            origin,
        } => Some(syntax_name_range(sloe::WithStartPosition {
            value: name,
            start: origin.start,
        })),
    };
    origin_name_range.map(|origin_name_range| {
        lsp_types::GotoDefinitionResponse::Scalar(lsp_types::Location {
            uri: goto_definition_arguments
                .text_document_position_params
                .text_document
                .uri,
            range: origin_name_range,
        })
    })
}

fn respond_to_prepare_rename<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    prepare_rename_arguments: &lsp_types::TextDocumentPositionParams,
) -> Option<Result<lsp_types::PrepareRenameResponse, lsp_server::ResponseError>> {
    let Some(project_state) = state
        .projects
        .get(&prepare_rename_arguments.text_document.uri)
    else {
        return None;
    };
    let Some(symbol) = sloe::syntax_project_symbol_at_position(
        &project_state.syntax,
        prepare_rename_arguments.position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    todo!()
}

fn respond_to_rename<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    rename_arguments: lsp_types::RenameParams,
) -> Option<Vec<lsp_types::TextDocumentEdit>> {
    let Some(project_state) = state
        .projects
        .get(&rename_arguments.text_document_position.text_document.uri)
    else {
        return None;
    };
    let Some(symbol) = sloe::syntax_project_symbol_at_position(
        &project_state.syntax,
        rename_arguments.text_document_position.position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    todo!()
}

fn respond_to_references<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    references_arguments: &lsp_types::ReferenceParams,
) -> Option<Vec<lsp_types::Location>> {
    let Some(project_state) = state.projects.get(
        &references_arguments
            .text_document_position
            .text_document
            .uri,
    ) else {
        return None;
    };
    let Some(symbol) = sloe::syntax_project_symbol_at_position(
        &project_state.syntax,
        references_arguments.text_document_position.position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    todo!()
}

fn respond_to_semantic_tokens_full<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    semantic_tokens_arguments: &lsp_types::SemanticTokensParams,
) -> Option<lsp_types::SemanticTokensResult> {
    let Some(project_state) = state
        .projects
        .get(&semantic_tokens_arguments.text_document.uri)
    else {
        return None;
    };
    let mut highlight_state = HighlightState {
        tokens: Vec::with_capacity(project_state.source.len() / 16),
        previous_token_start: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    sloe_project_highlight(
        &mut highlight_state,
        &project_state.syntax,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    );
    Some(lsp_types::SemanticTokensResult::Tokens(
        lsp_types::SemanticTokens {
            result_id: None,
            data: highlight_state.tokens,
        },
    ))
}
struct HighlightState {
    tokens: Vec<lsp_types::SemanticToken>,
    previous_token_start: lsp_types::Position,
}
fn highlight_state_add_token_with_start_and_length(
    state: &mut HighlightState,
    new_token_kind: lsp_types::SemanticTokenType,
    new_token_start: lsp_types::Position,
    new_token_length: usize,
) {
    if new_token_length == 0 {
        return;
    }
    match lsp_position_positive_delta(state.previous_token_start, new_token_start) {
        Err(error) => {
            eprintln!("bad highlight token order {error}");
            return;
        }
        Ok(delta) => {
            let token = lsp_types::SemanticToken {
                delta_line: delta.line,
                delta_start: delta.character,
                length: new_token_length as u32,
                token_type: semantic_token_type_to_id(&new_token_kind),
                token_modifiers_bitset: 0_u32,
            };
            state.previous_token_start = new_token_start;
            state.tokens.push(token);
        }
    }
}
fn keyword_highlight(
    state: &mut HighlightState,
    keyword: &'static str,
    new_token_start: lsp_types::Position,
) {
    highlight_state_add_token_with_start_and_length(
        state,
        lsp_types::SemanticTokenType::KEYWORD,
        new_token_start,
        keyword.len(),
    );
}
fn sloe_project_highlight<Expressions, Patterns, Types>(
    state: &mut HighlightState,
    project: &sloe::SyntaxProject<Expressions, Patterns, Types>,
    expressions: &sloe::core::Vec<
        Expressions,
        sloe::SyntaxExpression<Expressions, Patterns, Types>,
    >,
    patterns: &sloe::core::Vec<Patterns, sloe::SyntaxPattern<Patterns, Types>>,
    types: &sloe::core::Vec<Types, sloe::SyntaxType<Types>>,
) {
    for element in &project.elements {
        match element {
            sloe::SyntaxProjectElement::Comments(comments) => {
                sloe_syntax_comments_highlight(state, comments);
            }
            sloe::SyntaxProjectElement::TypeAlias {
                type_keyword_start,
                name,
                parameters,
                documentation,
                type_,
            } => {
                keyword_highlight(state, "type", *type_keyword_start);
                if let Some(name) = name {
                    highlight_state_add_token_with_start_and_length(
                        state,
                        lsp_types::SemanticTokenType::TYPE,
                        name.start,
                        name.value.len(),
                    );
                }
                for parameter in parameters {
                    highlight_state_add_token_with_start_and_length(
                        state,
                        lsp_types::SemanticTokenType::TYPE_PARAMETER,
                        parameter.start,
                        parameter.value.len(),
                    );
                }
                if let Some(documentation) = documentation {
                    sloe_syntax_comments_highlight(state, documentation);
                }
                if let Some(type_) = type_ {
                    sloe_syntax_type_highlight(state, types, type_);
                }
            }
            sloe::SyntaxProjectElement::ChoiceType {
                choice_keyword_start,
                name,
                parameters,
                documentation,
                variants,
            } => {
                keyword_highlight(state, "choice", *choice_keyword_start);
                if let Some(name) = name {
                    highlight_state_add_token_with_start_and_length(
                        state,
                        lsp_types::SemanticTokenType::TYPE,
                        name.start,
                        name.value.len(),
                    );
                }
                for parameter in parameters {
                    highlight_state_add_token_with_start_and_length(
                        state,
                        lsp_types::SemanticTokenType::TYPE_PARAMETER,
                        parameter.start,
                        parameter.value.len(),
                    );
                }
                if let Some(documentation) = documentation {
                    sloe_syntax_comments_highlight(state, documentation);
                }
                for variant in variants {
                    sloe_syntax_variant_highlight(state, types, variant);
                }
            }
            sloe::SyntaxProjectElement::Fn {
                fn_keyword_start,
                name,
                type_parameters,
                parameter,
                result_type,
                documentation,
                result,
            } => {
                keyword_highlight(state, "fn", *fn_keyword_start);
                if let Some(name) = name {
                    highlight_state_add_token_with_start_and_length(
                        state,
                        lsp_types::SemanticTokenType::VARIABLE,
                        name.start,
                        name.value.len(),
                    );
                }
                if let Some(result_type) = result_type {
                    sloe_syntax_type_highlight(state, types, result_type);
                }
                if let Some(type_parameters) = type_parameters {
                    sloe_angled_type_parameters_highlight(state, type_parameters);
                }
                if let Some(parameter) = parameter {
                    sloe_syntax_pattern_highlight(state, patterns, types, parameter)
                }
                if let Some(documentation) = documentation {
                    sloe_syntax_comments_highlight(state, documentation);
                }
                if let Some(result) = result {
                    sloe_syntax_expression_highlight(state, expressions, patterns, types, result)
                }
            }
            sloe::SyntaxProjectElement::Unrecognized { .. } => {}
        }
    }
}
fn sloe_syntax_variant_highlight<Types>(
    state: &mut HighlightState,
    types: &sloe::core::Vec<Types, sloe::SyntaxType<Types>>,
    variant: &sloe::SyntaxVariant<Types>,
) {
    if let Some(name) = &variant.name {
        sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::ENUM_MEMBER);
    }
    if let Some(type_parameters) = &variant.type_parameters {
        sloe_angled_type_parameters_highlight(state, type_parameters);
    }
    if let Some(value) = &variant.value {
        sloe_syntax_type_highlight(state, types, value);
    }
}
fn sloe_syntax_comments_highlight(state: &mut HighlightState, comments: &sloe::SyntaxComments) {
    for line in std::iter::once(&comments.line0).chain(comments.line1_up.iter()) {
        highlight_state_add_token_with_start_and_length(
            state,
            lsp_types::SemanticTokenType::VARIABLE,
            line.start,
            line.value.encode_utf16().count(),
        );
    }
}
fn sloe_syntax_name_highlight(
    state: &mut HighlightState,
    name: &sloe::WithStartPosition<sloe::Name>,
    kind: lsp_types::SemanticTokenType,
) {
    highlight_state_add_token_with_start_and_length(
        state,
        kind,
        name.start,
        name.value.encode_utf16().count(),
    );
}
fn sloe_angled_type_parameters_highlight(
    state: &mut HighlightState,
    angled_type_parameters: &sloe::SyntaxAngledTypeParameters,
) {
    for name in &angled_type_parameters.names {
        sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::TYPE_PARAMETER);
    }
}
fn sloe_angled_type_arguments_highlight<Types>(
    state: &mut HighlightState,
    types: &sloe::core::Vec<Types, sloe::SyntaxType<Types>>,
    angled_type_arguments: &sloe::SyntaxAngledTypeArguments<Types>,
) {
    for argument in types.opt_span_slice(sloe::core::Opt::from_option(
        angled_type_arguments.types.as_ref(),
    )) {
        sloe_syntax_type_highlight(state, types, argument);
    }
}
fn sloe_syntax_field_highlight<Value>(
    state: &mut HighlightState,
    field: &sloe::SyntaxField<Value>,
    value_highlight: impl FnOnce(&mut HighlightState, &Value),
) {
    match field {
        sloe::SyntaxField::Parenthsized {
            open_paren_start: _,
            name,
            value,
            closed_paren_start: _,
        } => {
            if let Some(name) = name {
                sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::PROPERTY);
            }
            if let Some(value) = value {
                value_highlight(state, value);
            }
        }
        sloe::SyntaxField::Unparenthsized { name, value } => {
            sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::PROPERTY);
            if let Some(value) = value {
                value_highlight(state, value);
            }
        }
    }
}
fn sloe_syntax_pattern_highlight<Patterns, Types>(
    state: &mut HighlightState,
    patterns: &sloe::core::Vec<Patterns, sloe::SyntaxPattern<Patterns, Types>>,
    types: &sloe::core::Vec<Types, sloe::SyntaxType<Types>>,
    pattern: &sloe::SyntaxPattern<Patterns, Types>,
) {
    match pattern {
        sloe::SyntaxPattern::Variable { name, type_ } => {
            sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::PARAMETER);
            if let Some(type_) = type_ {
                sloe_syntax_type_highlight(state, types, type_)
            }
        }
        sloe::SyntaxPattern::Variant {
            name,
            type_arguments,
            value,
        } => {
            sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::ENUM_MEMBER);
            if let Some(type_arguments) = type_arguments {
                sloe_angled_type_arguments_highlight(state, types, type_arguments);
            }
            if let Some(value) = value {
                sloe_syntax_pattern_highlight(state, patterns, types, patterns.element(value))
            }
        }
        sloe::SyntaxPattern::Record {
            ampersand_start,
            fields,
        } => {
            keyword_highlight(state, "&", *ampersand_start);
            for field in fields {
                sloe_syntax_field_highlight(state, field, |state, value| {
                    sloe_syntax_pattern_highlight(state, patterns, types, value)
                });
            }
        }
        sloe::SyntaxPattern::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                sloe_syntax_pattern_highlight(state, patterns, types, patterns.element(inner));
            }
        }
    }
}
fn sloe_syntax_type_highlight<Types>(
    state: &mut HighlightState,
    types: &sloe::core::Vec<Types, sloe::SyntaxType<Types>>,
    type_: &sloe::SyntaxType<Types>,
) {
    match type_ {
        sloe::SyntaxType::Variable(name) => {
            sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::TYPE_PARAMETER);
        }
        sloe::SyntaxType::Record {
            ampersand_start,
            fields,
        } => {
            keyword_highlight(state, "&", *ampersand_start);
            for field in fields {
                sloe_syntax_field_highlight(state, field, |state, value| {
                    sloe_syntax_type_highlight(state, types, value)
                });
            }
        }
        sloe::SyntaxType::Construct { name, arguments } => {
            sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::TYPE);
            for argument in types.opt_span_slice(sloe::core::Opt::from_option(arguments.as_ref())) {
                sloe_syntax_type_highlight(state, types, argument);
            }
        }
        sloe::SyntaxType::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                sloe_syntax_type_highlight(state, types, types.element(inner));
            }
        }
    }
}
fn sloe_syntax_expression_highlight<Expressions, Patterns, Types>(
    state: &mut HighlightState,
    expressions: &sloe::core::Vec<
        Expressions,
        sloe::SyntaxExpression<Expressions, Patterns, Types>,
    >,
    patterns: &sloe::core::Vec<Patterns, sloe::SyntaxPattern<Patterns, Types>>,
    types: &sloe::core::Vec<Types, sloe::SyntaxType<Types>>,
    expression: &sloe::SyntaxExpression<Expressions, Patterns, Types>,
) {
    match expression {
        sloe::SyntaxExpression::Number { value, type_ } => {
            highlight_state_add_token_with_start_and_length(
                state,
                lsp_types::SemanticTokenType::NUMBER,
                value.start,
                value.value.len(),
            );
            if let Some(type_) = type_ {
                sloe_syntax_type_highlight(state, types, type_);
            }
        }
        sloe::SyntaxExpression::Char {
            open_quote_start,
            content: _,
            content_end,
            closed_quote_exists,
        } => {
            highlight_state_add_token_with_start_and_length(
                state,
                lsp_types::SemanticTokenType::STRING,
                *open_quote_start,
                (content_end.character - open_quote_start.character
                    + (if *closed_quote_exists { 1 } else { 0 })) as usize,
            );
        }
        sloe::SyntaxExpression::Str {
            open_quote_start,
            content: _,
            content_end,
            closed_quote_exists,
        } => {
            highlight_state_add_token_with_start_and_length(
                state,
                lsp_types::SemanticTokenType::STRING,
                *open_quote_start,
                (content_end.character - open_quote_start.character
                    + (if *closed_quote_exists { 1 } else { 0 })) as usize,
            );
        }
        sloe::SyntaxExpression::ReferenceOrCall {
            name,
            type_arguments,
            argument,
        } => {
            sloe_syntax_name_highlight(
                state,
                name,
                if argument.is_some() || type_arguments.is_some() {
                    lsp_types::SemanticTokenType::VARIABLE
                } else {
                    lsp_types::SemanticTokenType::FUNCTION
                },
            );
            if let Some(type_arguments) = type_arguments {
                sloe_angled_type_arguments_highlight(state, types, type_arguments);
            }
            if let Some(argument) = argument {
                sloe_syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    types,
                    expressions.element(argument),
                );
            }
        }
        sloe::SyntaxExpression::Variant {
            name,
            type_arguments,
            value,
        } => {
            sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::ENUM_MEMBER);
            if let Some(type_arguments) = type_arguments {
                sloe_angled_type_arguments_highlight(state, types, type_arguments);
            }
            if let Some(value) = value {
                sloe_syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    types,
                    expressions.element(value),
                );
            }
        }
        sloe::SyntaxExpression::Fn {
            fn_keyword_start,
            parameter,
            result_type,
            result,
        } => {
            keyword_highlight(state, "fn", *fn_keyword_start);
            if let Some(parameter) = parameter {
                sloe_syntax_pattern_highlight(state, patterns, types, parameter);
            }
            if let Some(result_type) = result_type {
                sloe_syntax_type_highlight(state, types, result_type);
            }
            if let Some(result) = result {
                sloe_syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    types,
                    expressions.element(result),
                );
            }
        }
        sloe::SyntaxExpression::Record {
            ampersand_start,
            fields,
        } => {
            keyword_highlight(state, "&", *ampersand_start);
            for field in fields {
                sloe_syntax_field_highlight(state, field, |state, value| {
                    sloe_syntax_expression_highlight(state, expressions, patterns, types, value);
                });
            }
        }
        sloe::SyntaxExpression::Parenthesized {
            open_paren_start: _,
            inner,
            closed_paren_start: _,
        } => {
            if let Some(inner) = inner {
                sloe_syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    types,
                    expressions.element(inner),
                )
            }
        }
        sloe::SyntaxExpression::Commented {
            comments,
            expression,
        } => {
            sloe_syntax_comments_highlight(state, comments);
            if let Some(expression) = expression {
                sloe_syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    types,
                    expressions.element(expression),
                )
            }
        }
        sloe::SyntaxExpression::Query {
            colon_start,
            queried,
            cases,
        } => {
            keyword_highlight(state, ":", *colon_start);
            if let Some(queried) = queried {
                sloe_syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    types,
                    expressions.element(queried),
                )
            }
            for case in cases {
                match case {
                    sloe::SyntaxExpressionQueryCase::Parenthesized {
                        open_paren_start: _,
                        pattern,
                        result,
                        closed_paren_start: _,
                    } => {
                        if let Some(pattern) = pattern {
                            sloe_syntax_pattern_highlight(state, patterns, types, pattern);
                        }
                        if let Some(result) = result {
                            sloe_syntax_expression_highlight(
                                state,
                                expressions,
                                patterns,
                                types,
                                result,
                            )
                        }
                    }
                    sloe::SyntaxExpressionQueryCase::Unparenthesized { pattern, result } => {
                        sloe_syntax_pattern_highlight(state, patterns, types, pattern);
                        if let Some(result) = result {
                            sloe_syntax_expression_highlight(
                                state,
                                expressions,
                                patterns,
                                types,
                                result,
                            )
                        }
                    }
                }
            }
        }
        sloe::SyntaxExpression::Origin {
            origin_keyword_start,
            name,
            result,
        } => {
            keyword_highlight(state, "origin", *origin_keyword_start);
            if let Some(name) = name {
                sloe_syntax_name_highlight(state, name, lsp_types::SemanticTokenType::VARIABLE);
            }
            if let Some(result) = result {
                sloe_syntax_expression_highlight(
                    state,
                    expressions,
                    patterns,
                    types,
                    expressions.element(result),
                )
            }
        }
    }
}

const token_types: [lsp_types::SemanticTokenType; 11] = [
    lsp_types::SemanticTokenType::NUMBER,
    lsp_types::SemanticTokenType::STRING,
    lsp_types::SemanticTokenType::NAMESPACE,
    lsp_types::SemanticTokenType::VARIABLE,
    lsp_types::SemanticTokenType::TYPE,
    lsp_types::SemanticTokenType::TYPE_PARAMETER,
    lsp_types::SemanticTokenType::KEYWORD,
    lsp_types::SemanticTokenType::ENUM_MEMBER,
    lsp_types::SemanticTokenType::PROPERTY,
    lsp_types::SemanticTokenType::COMMENT,
    lsp_types::SemanticTokenType::FUNCTION,
];

fn semantic_token_type_to_id(semantic_token: &lsp_types::SemanticTokenType) -> u32 {
    token_types
        .iter()
        .enumerate()
        .find_map(|(i, token)| {
            if token == semantic_token {
                Some(i as u32)
            } else {
                None
            }
        })
        .unwrap_or(0_u32)
}
#[derive(Copy, Clone)]
struct PositionDelta {
    line: u32,
    character: u32,
}
fn lsp_position_positive_delta(
    before: lsp_types::Position,
    after: lsp_types::Position,
) -> Result<PositionDelta, String> {
    match before.line.cmp(&after.line) {
        std::cmp::Ordering::Greater => Err(format!(
            "before line > after line (before: {}, after: {})",
            lsp_position_to_string(before),
            lsp_position_to_string(after)
        )),
        std::cmp::Ordering::Equal => {
            if before.character > after.character {
                Err(format!(
                    "before character > after character (before: {}, after: {})",
                    lsp_position_to_string(before),
                    lsp_position_to_string(after)
                ))
            } else {
                Ok(PositionDelta {
                    line: 0,
                    character: after.character - before.character,
                })
            }
        }
        std::cmp::Ordering::Less => Ok(PositionDelta {
            line: after.line - before.line,
            character: after.character,
        }),
    }
}
fn lsp_position_to_string(lsp_position: lsp_types::Position) -> String {
    format!("{}:{}", lsp_position.line, lsp_position.character)
}

fn respond_to_completion<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    completion_arguments: &lsp_types::CompletionParams,
) -> Option<lsp_types::CompletionResponse> {
    let Some(project_state) = state.projects.get(
        &completion_arguments
            .text_document_position
            .text_document
            .uri,
    ) else {
        return None;
    };
    let Some(symbol) = sloe::syntax_project_symbol_at_position(
        &project_state.syntax,
        completion_arguments.text_document_position.position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    match symbol {
        sloe::SyntaxSymbol::ProjectTypeOrUnknown { name: _, origins } => {
            Some(lsp_types::CompletionResponse::Array(
                project_state
                    .choice_types
                    .iter()
                    .map(|(choice_type_name, choice_type_info)| {
                        let mut inserted_text = String::new();
                        if choice_type_info.parameters.is_empty() {
                            inserted_text.push_str(choice_type_name);
                        } else {
                            inserted_text.push('(');
                            inserted_text.push_str(choice_type_name);
                            for parameter in &choice_type_info.parameters {
                                inserted_text.push(' ');
                                inserted_text.push_str(parameter);
                            }
                            inserted_text.push(')');
                        }
                        lsp_types::CompletionItem {
                            label: choice_type_name.to_string(),
                            kind: Some(lsp_types::CompletionItemKind::ENUM),
                            detail: Some(present_choice_type_markdown(
                                choice_type_name,
                                choice_type_info,
                            )),
                            insert_text: Some(inserted_text),
                            ..lsp_types::CompletionItem::default()
                        }
                    })
                    .chain(project_state.type_aliases.iter().map(
                        |(type_alias_name, type_alias_info)| {
                            let mut inserted_text = String::new();
                            if type_alias_info.parameters.is_empty() {
                                inserted_text.push_str(type_alias_name);
                            } else {
                                inserted_text.push('(');
                                inserted_text.push_str(type_alias_name);
                                for parameter in &type_alias_info.parameters {
                                    inserted_text.push(' ');
                                    inserted_text.push_str(parameter);
                                }
                                inserted_text.push(')');
                            }
                            lsp_types::CompletionItem {
                                label: type_alias_name.to_string(),
                                kind: Some(lsp_types::CompletionItemKind::STRUCT),
                                detail: Some(present_type_alias_markdown(
                                    type_alias_name,
                                    type_alias_info,
                                )),
                                insert_text: Some(inserted_text),
                                ..lsp_types::CompletionItem::default()
                            }
                        },
                    ))
                    .chain(origins.into_iter().map(|(origin_name, _origin_origin)| {
                        lsp_types::CompletionItem {
                            label: origin_name.to_string(),
                            kind: Some(lsp_types::CompletionItemKind::STRUCT),
                            detail: Some(format!("```sloe\norigin {}\n```", origin_name)),
                            ..lsp_types::CompletionItem::default()
                        }
                    }))
                    .collect(),
            ))
        }
        sloe::SyntaxSymbol::Origin { .. } => None,
        sloe::SyntaxSymbol::TypeVariable {
            name: _,
            use_start,
            scope,
        } => {
            let mut available_existing_variables = std::collections::HashSet::new();
            match scope {
                sloe::SyntaxProjectElement::TypeAlias {
                    type_keyword_start: _,
                    name: _,
                    parameters,
                    documentation: _,
                    type_: _,
                } => {
                    for parameter in parameters {
                        if parameter.start == use_start {
                            return None;
                        }
                        available_existing_variables.insert(&parameter.value);
                    }
                }
                sloe::SyntaxProjectElement::ChoiceType {
                    choice_keyword_start: _,
                    name: _,
                    parameters,
                    documentation: _,
                    variants: _,
                } => {
                    for parameter in parameters {
                        if parameter.start == use_start {
                            return None;
                        }
                        available_existing_variables.insert(&parameter.value);
                    }
                }
                sloe::SyntaxProjectElement::Fn {
                    fn_keyword_start: _,
                    name: _,
                    type_parameters,
                    parameter,
                    result_type,
                    documentation: _,
                    result: _,
                } => {
                    for parameter in type_parameters
                        .as_ref()
                        .map(|type_parameters| &type_parameters.names)
                        .into_iter()
                        .flatten()
                    {
                        if parameter.start == use_start {
                            return None;
                        }
                        available_existing_variables.insert(&parameter.value);
                    }
                    if let Some(parameter) = parameter {
                        sloe::syntax_pattern_type_variables_into(
                            &mut available_existing_variables,
                            parameter,
                            &state.syntax_patterns,
                            &state.syntax_types,
                        );
                    }
                    if let Some(result_type) = result_type {
                        sloe::syntax_type_variables_into(
                            &mut available_existing_variables,
                            result_type,
                            &state.syntax_types,
                        );
                    }
                }
                sloe::SyntaxProjectElement::Comments(_) => {}
                sloe::SyntaxProjectElement::Unrecognized { .. } => {}
            }
            Some(lsp_types::CompletionResponse::Array(
                available_existing_variables
                    .into_iter()
                    .map(|available_existing_variable| lsp_types::CompletionItem {
                        label: available_existing_variable.to_string(),
                        kind: Some(lsp_types::CompletionItemKind::TYPE_PARAMETER),
                        detail: Some("type variable".to_string()),
                        ..lsp_types::CompletionItem::default()
                    })
                    .collect(),
            ))
        }
        sloe::SyntaxSymbol::VariantOrUnknown(_) => Some(lsp_types::CompletionResponse::Array(
            project_state
                .choice_types
                .iter()
                .flat_map(|(choice_type_name, choice_type_info)| {
                    choice_type_info.variants.iter().map(|variant| {
                        let mut inserted_text = String::new();
                        // potential improvement: do not suggest type arguments when type is already known
                        // (e.g. in query patterns)
                        match &variant.value {
                            None => {
                                inserted_text.push_str(&variant.name);
                                angled_type_parameters_format(
                                    &mut inserted_text,
                                    &variant.type_parameters,
                                );
                            }
                            Some(value_type) => {
                                inserted_text.push('(');
                                inserted_text.push_str(&variant.name);
                                angled_type_parameters_format(
                                    &mut inserted_text,
                                    &variant.type_parameters,
                                );
                                inserted_text.push(' ');
                                match value_type {
                                    sloe::Type::Record(parameter_fields) => {
                                        inserted_text.push_str("&");
                                        for field in parameter_fields {
                                            inserted_text.push_str(" (");
                                            inserted_text.push_str(&field.name);
                                            inserted_text.push_str(" )");
                                        }
                                    }
                                    _ => {}
                                }
                                inserted_text.push(')');
                            }
                        }
                        lsp_types::CompletionItem {
                            label: variant.name.to_string(),
                            kind: Some(lsp_types::CompletionItemKind::ENUM_MEMBER),
                            detail: Some(format!(
                                "variant in\n{}",
                                present_choice_type_markdown(choice_type_name, choice_type_info)
                            )),
                            insert_text: Some(inserted_text),
                            ..lsp_types::CompletionItem::default()
                        }
                    })
                })
                .collect(),
        )),
        sloe::SyntaxSymbol::ProjectFnOrUnknown {
            name: _,
            pattern_variables,
            origins,
        } => Some(lsp_types::CompletionResponse::Array(
            project_state
                .fns
                .iter()
                .map(|(fn_name, fn_info)| {
                    let mut inserted_text = String::new();
                    match &fn_info.parameter_type {
                        None => {
                            inserted_text.push_str(fn_name);
                            angled_type_parameters_format(
                                &mut inserted_text,
                                &fn_info.type_parameters,
                            );
                        }
                        Some(parameter_type) => {
                            inserted_text.push('(');
                            inserted_text.push_str(fn_name);
                            angled_type_parameters_format(
                                &mut inserted_text,
                                &fn_info.type_parameters,
                            );
                            inserted_text.push(' ');
                            match parameter_type {
                                sloe::Type::Record(parameter_fields) => {
                                    inserted_text.push_str("&");
                                    for field in parameter_fields {
                                        inserted_text.push_str(" (");
                                        inserted_text.push_str(&field.name);
                                        inserted_text.push_str(" )");
                                    }
                                }
                                _ => {}
                            }
                            inserted_text.push(')');
                        }
                    }
                    lsp_types::CompletionItem {
                        label: fn_name.to_string(),
                        kind: Some(lsp_types::CompletionItemKind::FUNCTION),
                        detail: Some(present_project_fn_with_complete_type_markdown(
                            fn_name, fn_info,
                        )),
                        insert_text: Some(inserted_text),
                        ..lsp_types::CompletionItem::default()
                    }
                })
                .chain(pattern_variables.into_iter().map(
                    |(pattern_variable, _pattern_variable_origin)| {
                        // portential improvement: do not suggest variables and origins that have already been used earlier
                        lsp_types::CompletionItem {
                            label: pattern_variable.to_string(),
                            kind: Some(lsp_types::CompletionItemKind::VARIABLE),
                            detail: Some(format!("pattern variable {}", pattern_variable)),
                            ..lsp_types::CompletionItem::default()
                        }
                    },
                ))
                .chain(origins.into_iter().map(|(origin_name, _origin_origin)| {
                    lsp_types::CompletionItem {
                        label: origin_name.to_string(),
                        kind: Some(lsp_types::CompletionItemKind::VARIABLE),
                        detail: Some(format!("```sloe\norigin {}\n```", origin_name)),
                        ..lsp_types::CompletionItem::default()
                    }
                }))
                .collect(),
        )),
        sloe::SyntaxSymbol::PatternVariable { .. } => None,
    }
}

fn respond_to_document_formatting<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    formatting_arguments: &lsp_types::DocumentFormattingParams,
) -> Option<Vec<lsp_types::TextEdit>> {
    let Some(project_state) = state.projects.get(&formatting_arguments.text_document.uri) else {
        return None;
    };
    let formatted: String = sloe::syntax_project_format(
        &project_state.syntax,
        &project_state.source,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    );
    // diffing does not seem to be needed here. But maybe it's faster?
    Some(vec![lsp_types::TextEdit {
        range: lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 0,
            },
            end: lsp_types::Position {
                line: project_state.source.lines().count() as u32
                    + (
                        // restore last line break potentially eaten by .lines()
                        if project_state.source.ends_with(['\r', '\n']) {
                            1
                        } else {
                            0
                        }
                    )
                    + 1,
                character: 0,
            },
        },
        new_text: formatted,
    }])
}

fn respond_to_document_symbols<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    document_symbol_arguments: &lsp_types::DocumentSymbolParams,
) -> Option<lsp_types::DocumentSymbolResponse> {
    let project = state
        .projects
        .get(&document_symbol_arguments.text_document.uri)?;
    Some(lsp_types::DocumentSymbolResponse::Nested(
        project
            .syntax
            .elements
            .iter()
            .filter_map(|project_element| match project_element {
                sloe::SyntaxProjectElement::Comments { .. } => None,
                sloe::SyntaxProjectElement::Unrecognized { .. } => None,
                sloe::SyntaxProjectElement::TypeAlias {
                    type_keyword_start: _,
                    name,
                    parameters: _,
                    documentation: _,
                    type_,
                } => {
                    let Some(name) = name else {
                        return None;
                    };
                    Some(lsp_types::DocumentSymbol {
                        name: name.value.to_string(),
                        detail: None,
                        kind: lsp_types::SymbolKind::STRUCT,
                        tags: None,
                        #[allow(deprecated)]
                        deprecated: None,
                        range: lsp_types::Range {
                            start: name.start,
                            end: type_
                                .as_ref()
                                .map(|value| sloe::type_end(value, &state.syntax_types))
                                .unwrap_or_else(|| {
                                    sloe::name_end(sloe::with_start_position_as_ref(name))
                                }),
                        },
                        selection_range: sloe::syntax_name_range(sloe::with_start_position_as_ref(
                            name,
                        )),
                        children: None,
                    })
                }
                sloe::SyntaxProjectElement::ChoiceType {
                    choice_keyword_start: _,
                    name,
                    parameters: _,
                    documentation: _,
                    variants,
                } => {
                    let Some(name) = name else {
                        return None;
                    };
                    Some(lsp_types::DocumentSymbol {
                        name: name.value.to_string(),
                        detail: None,
                        kind: lsp_types::SymbolKind::ENUM,
                        tags: None,
                        #[allow(deprecated)]
                        deprecated: None,
                        range: lsp_types::Range {
                            start: name.start,
                            end: variants
                                .last()
                                .and_then(|variant| {
                                    variant
                                        .value
                                        .as_ref()
                                        .map(|value| sloe::type_end(value, &state.syntax_types))
                                        .or_else(|| {
                                            variant.name.as_ref().map(|name| {
                                                sloe::name_end(sloe::with_start_position_as_ref(
                                                    name,
                                                ))
                                            })
                                        })
                                })
                                .unwrap_or_else(|| {
                                    sloe::name_end(sloe::with_start_position_as_ref(name))
                                }),
                        },
                        selection_range: sloe::syntax_name_range(sloe::with_start_position_as_ref(
                            name,
                        )),
                        children: Some(
                            variants
                                .iter()
                                .filter_map(|variant| {
                                    let Some(variant_name_node) = &variant.name else {
                                        return None;
                                    };
                                    Some((
                                        variant_name_node,
                                        lsp_types::Range {
                                            start: variant_name_node.start,
                                            end: sloe::variant_end(variant, &state.syntax_types),
                                        },
                                    ))
                                })
                                .map(|(variant_name, variant_full_span)| {
                                    lsp_types::DocumentSymbol {
                                        name: variant_name.value.to_string(),
                                        detail: None,
                                        kind: lsp_types::SymbolKind::ENUM_MEMBER,
                                        tags: None,
                                        #[allow(deprecated)]
                                        deprecated: None,
                                        range: variant_full_span,
                                        selection_range: sloe::syntax_name_range(
                                            sloe::with_start_position_as_ref(variant_name),
                                        ),
                                        children: None,
                                    }
                                })
                                .collect::<Vec<_>>(),
                        ),
                    })
                }
                sloe::SyntaxProjectElement::Fn {
                    fn_keyword_start,
                    name,
                    type_parameters: _,
                    parameter: _,
                    result_type: _,
                    documentation: _,
                    result: maybe_result,
                } => {
                    let Some(name) = name else {
                        return None;
                    };
                    Some(lsp_types::DocumentSymbol {
                        name: name.value.to_string(),
                        detail: None,
                        kind: lsp_types::SymbolKind::FUNCTION,
                        tags: None,
                        #[allow(deprecated)]
                        deprecated: None,
                        range: lsp_types::Range {
                            start: *fn_keyword_start,
                            end: maybe_result
                                .as_ref()
                                .map(|result_slot| {
                                    sloe::expression_end(
                                        result_slot,
                                        &state.syntax_expressions,
                                        &state.syntax_patterns,
                                        &state.syntax_types,
                                    )
                                })
                                .unwrap_or_else(|| {
                                    sloe::name_end(sloe::with_start_position_as_ref(name))
                                }),
                        },
                        selection_range: sloe::syntax_name_range(sloe::with_start_position_as_ref(
                            name,
                        )),
                        children: None,
                    })
                }
            })
            .collect::<Vec<_>>(),
    ))
}

fn str_lsp_span_to_span(str: &str, range: lsp_types::Range) -> std::ops::Range<usize> {
    let start_line_offset: usize =
        str_offset_after_n_lsp_linebreaks(str, range.start.line as usize);
    let start_offset: usize = start_line_offset
        + str_starting_utf8_length_for_utf16_length(
            &str[start_line_offset..],
            range.start.character as usize,
        );
    // can be optimized by only counting after the start line
    let end_line_offset: usize = str_offset_after_n_lsp_linebreaks(str, range.end.line as usize);
    let end_offset: usize = end_line_offset
        + str_starting_utf8_length_for_utf16_length(
            &str[end_line_offset..],
            range.end.character as usize,
        );
    start_offset..end_offset
}
fn str_offset_after_n_lsp_linebreaks(str: &str, linebreak_count_to_skip: usize) -> usize {
    if linebreak_count_to_skip == 0 {
        return 0;
    }
    let mut offset_after_n_linebreaks: usize = 0;
    let mut encountered_linebreaks: usize = 0;
    'finding_after_n_linebreaks_offset: loop {
        if str[offset_after_n_linebreaks..].starts_with("\r\n") {
            encountered_linebreaks += 1;
            offset_after_n_linebreaks += 2;
            if encountered_linebreaks >= linebreak_count_to_skip {
                break 'finding_after_n_linebreaks_offset;
            }
        } else {
            match str[offset_after_n_linebreaks..].chars().next() {
                None => {
                    break 'finding_after_n_linebreaks_offset;
                }
                // see EOL in https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocuments
                Some('\r' | '\n') => {
                    encountered_linebreaks += 1;
                    offset_after_n_linebreaks += 1;
                    if encountered_linebreaks >= linebreak_count_to_skip {
                        break 'finding_after_n_linebreaks_offset;
                    }
                }
                Some(next_char) => {
                    offset_after_n_linebreaks += next_char.len_utf8();
                }
            }
        }
    }
    offset_after_n_linebreaks
}
fn string_replace_lsp_span(string: &mut String, range: lsp_types::Range, replacement: &str) {
    string.replace_range(str_lsp_span_to_span(string, range), replacement);
}
/// slightly faster version of `string_replace_lsp_span` for when you know the length
fn string_replace_lsp_span_for_length(
    string: &mut String,
    range: lsp_types::Range,
    range_length: usize,
    replacement: &str,
) {
    let start_line_offset: usize =
        str_offset_after_n_lsp_linebreaks(string, range.start.line as usize);
    let start_offset: usize = start_line_offset
        + str_starting_utf8_length_for_utf16_length(
            &string[start_line_offset..],
            range.start.character as usize,
        );
    let span_length_utf8: usize =
        str_starting_utf8_length_for_utf16_length(&string[start_offset..], range_length);
    string.replace_range(start_offset..(start_offset + span_length_utf8), replacement);
}
fn str_starting_utf8_length_for_utf16_length(slice: &str, starting_utf16_length: usize) -> usize {
    let mut utf8_length: usize = 0;
    let mut so_far_length_utf16: usize = 0;
    'traversing_utf16_length: for char in slice.chars() {
        if so_far_length_utf16 >= starting_utf16_length {
            break 'traversing_utf16_length;
        }
        utf8_length += char.len_utf8();
        so_far_length_utf16 += char.len_utf16();
    }
    utf8_length
}

/// "polyfill" for the removed lsp_types::Uri::to_file_path (removed after 0.95.1)
/// Inspired by (thank you!): https://github.com/tower-lsp-community/tower-lsp-server/blob/ff1562a33bda1da55ef4edbfc9ee24ecd50f6807/src/uri_ext.rs
fn lsp_uri_to_file_path(uri: &lsp_types::Uri) -> Option<std::borrow::Cow<'_, std::path::Path>> {
    let Ok(path_as_str) = uri.path().as_estr().decode().into_string() else {
        return None;
    };
    let path_as_file_path: std::borrow::Cow<std::path::Path> = match path_as_str {
        std::borrow::Cow::Borrowed(str) => std::borrow::Cow::Borrowed(std::path::Path::new(str)),
        std::borrow::Cow::Owned(owned) => std::borrow::Cow::Owned(std::path::PathBuf::from(owned)),
    };
    if cfg!(windows) {
        let Some(authority) = uri.authority() else {
            return None;
        };
        let host = authority.host();
        if host.as_str().is_empty() {
            // assume file:/// → path includes leading /
            let path_with_leading_slash_str: std::borrow::Cow<str> =
                path_as_file_path.to_string_lossy();
            let Some(path_without_leading_slash) = path_with_leading_slash_str.get(1..) else {
                return None;
            };
            Some(std::borrow::Cow::Owned(std::path::PathBuf::from(
                path_without_leading_slash,
            )))
        } else {
            let mut full_file_path: std::path::PathBuf =
                std::path::PathBuf::from(format!("{host}:"));
            full_file_path.push(path_as_file_path);
            Some(std::borrow::Cow::Owned(full_file_path))
        }
    } else {
        Some(path_as_file_path)
    }
}
