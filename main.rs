#![allow(non_upper_case_globals)]
use gen_lsp_types as lsp_types;
use sloe_compile as sloe;

struct State<Expressions, Patterns, Types> {
    projects: std::collections::HashMap<lsp_types::Uri, ProjectState<Expressions, Patterns, Types>>,
    syntax_expressions:
        sloe::core::Buf<Expressions, sloe::SyntaxExpression<Expressions, Patterns, Types>>,
    syntax_patterns: sloe::core::Buf<Patterns, sloe::SyntaxPattern<Patterns, Types>>,
    syntax_types: sloe::core::Buf<Types, sloe::SyntaxType<Types>>,
}
struct ProjectState<Expressions, Patterns, Types> {
    source: String,
    syntax: sloe::SyntaxProject<Expressions, Patterns, Types>,
    type_aliases: std::collections::HashMap<sloe::Name, sloe::CheckedTypeAlias>,
    fns: std::collections::HashMap<sloe::Name, sloe::CheckedProjectFn>,
    queries: std::collections::HashMap<lsp_types::Position, sloe::CheckedQuery>,
    spread_records: std::collections::HashMap<lsp_types::Position, Vec<sloe::Name>>,
}
fn main() {
    match main_or_err() {
        Ok(()) => {}
        Err(()) => std::process::exit(1),
    }
}
fn main_or_err() -> Result<(), ()> {
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
            "build" | "make" | "compile" | "transpile" | "b" | "m" => {
                println!(
                    "To compile a file to a specific target language, use e.g. sloe zig.

Full help:
{command_help}"
                );
                Ok(())
            }
            "init" | "initialize" | "new" | "create" | "setup" | "boilerplate" | "template"
            | "hello" | "hello-world" => {
                println!(
                    "Hi! There is no one starting point for every sloe program.
Just create a sloe.sloe file and compile it to any of sloe's target languages like rust with `sloe rust`.
Take a look at the examples at https://codeberg.org/lue-bird/sloe/examples for setup inspiration ^^

Full help:
{command_help}"
                );
                Ok(())
            }
            "typescript" | "tsc" | "ts" | "tsx" => {
                println!(
                    "Only `sloe js` exists.
Generated js files include types in the form of jsdoc comments which just work with ts.
A jsdoc overview: https://www.typescriptlang.org/docs/handbook/jsdoc-supported-types.html

Full help:
{command_help}"
                );
                Ok(())
            }
            "rust" | "rs" | "rustc" | "rooster" | "roost" => {
                let maybe_input_file_path: Option<String> = full_command.next();
                let maybe_output_file_path: Option<String> = full_command.next();
                build_main(
                    CompileOutputLanguage::Rust,
                    maybe_input_file_path.as_ref().map(std::path::Path::new),
                    maybe_output_file_path.as_ref().map(std::path::Path::new),
                )
            }
            "zig" | "z" => {
                let maybe_input_file_path: Option<String> = full_command.next();
                let maybe_output_file_path: Option<String> = full_command.next();
                build_main(
                    CompileOutputLanguage::Zig,
                    maybe_input_file_path.as_ref().map(std::path::Path::new),
                    maybe_output_file_path.as_ref().map(std::path::Path::new),
                )
            }
            "javascript" | "jsdoc" | "ecmascript" | "ecma" | "mjs" | "jsm" | "esm" | "js"
            | "es" => {
                let maybe_input_file_path: Option<String> = full_command.next();
                let maybe_output_file_path: Option<String> = full_command.next();
                build_main(
                    CompileOutputLanguage::Js,
                    maybe_input_file_path.as_ref().map(std::path::Path::new),
                    maybe_output_file_path.as_ref().map(std::path::Path::new),
                )
            }
            "check" | "analyze" | "errors" | "warn" | "warnings" | "examine" | "validate"
            | "review" | "verify" | "chk" | "ch" => {
                let maybe_input_file_path: Option<String> = full_command.next();
                check_main(maybe_input_file_path.as_ref().map(std::path::Path::new))
            }
            "doc" | "docs" | "documentation" | "core" | "stdlib" | "core-doc" | "core-docs"
            | "core-documentation" | "core-types" | "dc" => {
                println!("Here are all core declarations:\n");
                print_core_docs();
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
To compile to a rust file: sloe rs [input-file.sloe [output-file.rs]]
To compile to a zig file: sloe zig [input-file.sloe [output-file.zig]]
To compile to a javascript module: sloe js [input-file.sloe [output-file.mjs]]
To copy the rust hello-world project setup into the current directory: sloe init
To start the language server: sloe lsp
To print core declaration documentation: sloe core-docs
To print this help message: sloe help

To run a rust project: cargo run
To compile a rust project into an executable: cargo build --release

See the source code, see the full documentation, report bugs or leave any kind of feedback at https://codeberg.org/lue-bird/sloe";

fn print_core_docs() {
    for (core_choice_type_name, core_type_alias) in sloe::core_type_aliases.iter() {
        println!(
            "{}",
            present_type_alias_markdown(core_choice_type_name, core_type_alias)
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
    fn_name: &sloe::Name,
    fn_info: &sloe::CheckedProjectFn,
) -> String {
    let mut formatted = String::new();
    sloe::checked_project_fn_format(&mut formatted, fn_name, fn_info);
    format!(
        "```sloe
{}
```
{}
",
        formatted,
        documentation_comment_to_markdown(fn_info.documentation.as_deref().unwrap_or(""))
    )
}
fn present_type_alias_markdown(name: &sloe::Name, type_alias: &sloe::CheckedTypeAlias) -> String {
    let mut formatted = String::new();
    sloe::checked_type_alias_format(&mut formatted, name, type_alias);
    let description = format!("```sloe\n{formatted}\n```\n");
    match &type_alias.documentation {
        None => description,
        Some(documentation) => {
            description + documentation_comment_to_markdown(documentation).as_str()
        }
    }
}
fn present_pattern_variable_markdown(type_: Option<&sloe::Type>) -> String {
    match type_ {
        None => "pattern variable".to_string(),
        Some(type_) => {
            let mut type_string = "pattern variable of type\n```sloe\n".to_string();
            sloe::type_format(&mut type_string, 0, type_);
            type_string + "\n```\n"
        }
    }
}
fn default_output_file_path_for_sloe_input_file_path(
    input_file_path: &std::path::Path,
    language: CompileOutputLanguage,
) -> std::path::PathBuf {
    match language {
        CompileOutputLanguage::Rust => input_file_path.with_extension("rs"),
        CompileOutputLanguage::Zig => input_file_path.with_extension("zig"),
        CompileOutputLanguage::Js => input_file_path.with_extension("mjs"),
    }
}
fn rust_file_name_derive_mod_name(rust_file_name: &std::path::Path) -> Result<&str, ()> {
    match rust_file_name
        .file_prefix()
        .and_then(|os_str| os_str.to_str())
    {
        Some(mod_name) => Ok(mod_name),
        None => {
            eprintln!(
                "Can't compile to {rust_file_name:?} because there's no clear module name to extract.
For example when I see .../.../src/sloe.rs I assume the mod name to be sloe."
            );
            Err(())
        }
    }
}

fn check_main(maybe_input_file_path: Option<&std::path::Path>) -> Result<(), ()> {
    let input_file_path: &std::path::Path = match maybe_input_file_path {
        Some(input_file_path) => &input_file_path.with_extension("sloe"),
        None => std::path::Path::new("sloe.sloe"),
    };
    println!("...checking {input_file_path:?} for errors.");
    let project_source = match std::fs::read_to_string(input_file_path) {
        Err(read_error) => {
            eprintln!(
                "was looking for a file with the name {input_file_path:?} but failed: {read_error}"
            );
            return Err(());
        }
        Ok(project_source) => project_source,
    };
    sloe::core::origin_new!(expressions, Expressions);
    sloe::core::origin_new!(patterns, Patterns);
    sloe::core::origin_new!(types, Types);
    let mut syntax_expressions = sloe::core::Buf::new(expressions);
    let mut syntax_patterns = sloe::core::Buf::new(patterns);
    let mut syntax_types = sloe::core::Buf::new(types);
    let syntax_project = sloe::parse_project(
        &mut syntax_expressions,
        &mut syntax_patterns,
        &mut syntax_types,
        &project_source,
    );
    let mut output_errors: Vec<sloe::ErrorNode> = Vec::new();
    let _checked_info = sloe::syntax_project_check(
        &mut output_errors,
        &syntax_project,
        &syntax_expressions,
        &syntax_patterns,
        &syntax_types,
    );
    if output_errors.is_empty() {
        println!(
            "No errors found. Note that sloe <language target> already checks before building."
        );
        Ok(())
    } else {
        for output_error in output_errors.iter().rev() {
            eprintln!(
                "{input_file_path}:{span_start_line}:{span_start_column} {message}",
                input_file_path = input_file_path.to_string_lossy(),
                span_start_line = output_error.range.start.line + 1,
                span_start_column = output_error.range.start.character + 1,
                message = output_error.message
            );
        }
        Err(())
    }
    // potential improvement: statistics
}
#[derive(Clone, Copy)]
enum CompileOutputLanguage {
    Rust,
    Zig,
    Js,
}
fn build_main(
    output_language: CompileOutputLanguage,
    maybe_input_file_path: Option<&std::path::Path>,
    maybe_output_file_path: Option<&std::path::Path>,
) -> Result<(), ()> {
    let input_file_path: &std::path::Path = match maybe_input_file_path {
        Some(input_file_path) => &input_file_path.with_extension("sloe"),
        None => std::path::Path::new("sloe.sloe"),
    };
    let output_file_path: &std::path::Path = match maybe_output_file_path {
        Some(output_file_path) => {
            &default_output_file_path_for_sloe_input_file_path(output_file_path, output_language)
        }
        None => {
            &default_output_file_path_for_sloe_input_file_path(input_file_path, output_language)
        }
    };
    println!("...compiling {input_file_path:?} into {output_file_path:?}.");
    let project_source = match std::fs::read_to_string(input_file_path) {
        Err(read_error) => {
            eprintln!(
                "was looking for a file with the name {input_file_path:?} but failed: {read_error}"
            );
            return Err(());
        }
        Ok(project_source) => project_source,
    };
    sloe::core::origin_new!(expressions, Expressions);
    sloe::core::origin_new!(patterns, Patterns);
    sloe::core::origin_new!(types, Types);
    let mut syntax_expressions = sloe::core::Buf::new(expressions);
    let mut syntax_patterns = sloe::core::Buf::new(patterns);
    let mut syntax_types = sloe::core::Buf::new(types);
    let syntax_project = sloe::parse_project(
        &mut syntax_expressions,
        &mut syntax_patterns,
        &mut syntax_types,
        &project_source,
    );
    let mut output_errors: Vec<sloe::ErrorNode> = Vec::new();
    let checked_project = sloe::syntax_project_check(
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
            message = output_error.message
        );
    }
    if let Some(output_file_directory_path) = output_file_path.parent()
        && let Err(error) = std::fs::create_dir_all(output_file_directory_path)
    {
        eprintln!(
            "tried to create the directory containing the output file {output_file_path:?} but failed: {}",
            error
        );
        return Err(());
    }
    let output_rust_file_string: String = match output_language {
        CompileOutputLanguage::Rust => {
            let Ok(output_mod_name) = rust_file_name_derive_mod_name(output_file_path) else {
                return Err(());
            };
            let compiled_project = sloe::checked_project_to_rust(
                &checked_project,
                &syntax_expressions,
                &syntax_patterns,
                &syntax_types,
            );
            sloe::compiled_rust_to_file_content(&compiled_project, output_mod_name)
        }
        CompileOutputLanguage::Zig => sloe::checked_project_to_zig(
            &checked_project,
            &syntax_expressions,
            &syntax_patterns,
            &syntax_types,
        ),
        CompileOutputLanguage::Js => sloe::checked_project_to_js(
            &checked_project,
            &syntax_expressions,
            &syntax_patterns,
            &syntax_types,
        ),
    };
    if let Err(write_error) = std::fs::write(output_file_path, output_rust_file_string) {
        eprintln!(
            "tried to write the output into the file {output_file_path:?} but failed: {}",
            write_error
        );
        return Err(());
    }
    if output_errors.is_empty() {
        Ok(())
    } else {
        Err(())
    }
}
fn lsp_main() -> Result<(), ()> {
    match lsp_main_or_error() {
        Ok(()) => Ok(()),
        Err(error) => {
            eprintln!("{}", error);
            Err(())
        }
    }
}
fn lsp_main_or_error() -> Result<(), Box<dyn std::error::Error>> {
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
    expressions: sloe::core::Origin<Expressions, Expressions>,
    patterns: sloe::core::Origin<Patterns, Patterns>,
    types: sloe::core::Origin<Types, Types>,
) -> State<
    sloe::core::Origin<Expressions, Expressions>,
    sloe::core::Origin<Patterns, Patterns>,
    sloe::core::Origin<Types, Types>,
> {
    State {
        projects: std::collections::HashMap::with_capacity(1),
        syntax_expressions: sloe::core::Buf::new(expressions),
        syntax_patterns: sloe::core::Buf::new(patterns),
        syntax_types: sloe::core::Buf::new(types),
    }
}
fn server_capabilities() -> lsp_types::ServerCapabilities {
    lsp_types::ServerCapabilities {
        hover_provider: Some(lsp_types::HoverProvider::Bool(true)),
        definition_provider: Some(lsp_types::DefinitionProvider::Bool(true)),
        semantic_tokens_provider: Some(lsp_types::SemanticTokensProvider::SemanticTokensOptions(
            lsp_types::SemanticTokensOptions {
                work_done_progress_options: lsp_types::WorkDoneProgressOptions {
                    work_done_progress: None,
                },
                legend: lsp_types::SemanticTokensLegend {
                    token_modifiers: vec![],
                    token_types: sloe::token_types
                        .iter()
                        .map(|token_type| token_type.as_str().to_string())
                        .collect(),
                },
                range: None,
                full: Some(lsp_types::Full::Bool(true)),
            },
        )),
        text_document_sync: Some(lsp_types::TextDocumentSync::Kind(
            lsp_types::TextDocumentSyncKind::Incremental,
        )),
        rename_provider: Some(lsp_types::RenameProvider::RenameOptions(
            lsp_types::RenameOptions {
                prepare_provider: Some(true),
                work_done_progress_options: lsp_types::WorkDoneProgressOptions {
                    work_done_progress: None,
                },
            },
        )),
        references_provider: Some(lsp_types::ReferencesProvider::Bool(true)),
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: None,
            all_commit_characters: None,
            work_done_progress_options: lsp_types::WorkDoneProgressOptions {
                work_done_progress: None,
            },
            completion_item: Some(lsp_types::ServerCompletionItemOptions {
                label_details_support: None,
            }),
        }),
        document_formatting_provider: Some(lsp_types::DocumentFormattingProvider::Bool(true)),
        document_symbol_provider: Some(lsp_types::DocumentSymbolProvider::Bool(true)),
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
                    lsp_types::LspRequestMethod::from(request.method.as_str()),
                    request.params,
                ) {
                    eprintln!("request {} failed: {error}", request.method);
                }
            }
            lsp_server::Message::Notification(notification) => {
                if let Err(err) = handle_notification(
                    connection,
                    &mut state,
                    lsp_types::LspNotificationMethod::from(notification.method.as_str()),
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
    notification_method: lsp_types::LspNotificationMethod,
    notification_arguments_json: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    match notification_method {
        <lsp_types::DidOpenTextDocumentNotification as lsp_types::Notification>::METHOD => {
            let arguments: <lsp_types::DidOpenTextDocumentNotification as lsp_types::Notification>::Params =
                serde_json::from_value(notification_arguments_json)?;
            update_state_on_did_open_text_document(state, connection, arguments);
        }
        <lsp_types::DidCloseTextDocumentNotification as lsp_types::Notification>::METHOD => {
            let arguments: <lsp_types::DidCloseTextDocumentNotification as lsp_types::Notification>::Params =
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
        <lsp_types::DidChangeTextDocumentNotification as lsp_types::Notification>::METHOD => {
            let arguments: <lsp_types::DidChangeTextDocumentNotification as lsp_types::Notification>::Params =
                serde_json::from_value(notification_arguments_json)?;
            update_state_on_did_change_text_document(state, connection, arguments);
        }
        <lsp_types::ExitNotification as lsp_types::Notification>::METHOD => {}
        _ => {}
    }
    Ok(())
}
fn update_state_on_did_open_text_document<Expressions, Patterns, Types>(
    state: &mut State<Expressions, Patterns, Types>,
    connection: &lsp_server::Connection,
    arguments: lsp_types::DidOpenTextDocumentParams,
) {
    if arguments.text_document.language_id
        == lsp_types::LanguageKind::Custom(std::borrow::Cow::Borrowed("sloe"))
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
    request_method: lsp_types::LspRequestMethod,
    request_arguments_json: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let response: Result<serde_json::Value, lsp_server::ResponseError> = match request_method {
        <lsp_types::HoverRequest as lsp_types::Request>::METHOD => {
            let arguments: <lsp_types::HoverRequest as lsp_types::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let maybe_hover_result: <lsp_types::HoverRequest as lsp_types::Request>::Result =
                respond_to_hover(state, &arguments);
            Ok(serde_json::to_value(maybe_hover_result)?)
        }
        <lsp_types::DefinitionRequest as lsp_types::Request>::METHOD => {
            let arguments: <lsp_types::DefinitionRequest as lsp_types::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let maybe_hover_result: <lsp_types::DefinitionRequest as lsp_types::Request>::Result =
                respond_to_goto_definition(state, arguments);
            Ok(serde_json::to_value(maybe_hover_result)?)
        }
        <lsp_types::PrepareRenameRequest as lsp_types::Request>::METHOD => {
            let prepare_rename_arguments: <lsp_types::PrepareRenameRequest as lsp_types::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let response: <lsp_types::PrepareRenameRequest as lsp_types::Request>::Result =
                respond_to_prepare_rename(state, &prepare_rename_arguments);
            Ok(serde_json::to_value(response)?)
        }
        <lsp_types::RenameRequest as lsp_types::Request>::METHOD => {
            let arguments: <lsp_types::RenameRequest as lsp_types::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let maybe_rename_edits: Option<Vec<lsp_types::DocumentChange>> =
                respond_to_rename(state, arguments);
            let result: <lsp_types::RenameRequest as lsp_types::Request>::Result =
                maybe_rename_edits.map(|rename_edits| lsp_types::WorkspaceEdit {
                    changes: None,
                    document_changes: Some(rename_edits),
                    change_annotations: None,
                });
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::ReferencesRequest as lsp_types::Request>::METHOD => {
            let arguments: <lsp_types::ReferencesRequest as lsp_types::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::ReferencesRequest as lsp_types::Request>::Result =
                respond_to_references(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::SemanticTokensRequest as lsp_types::Request>::METHOD => {
            let arguments: <lsp_types::SemanticTokensRequest as lsp_types::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::SemanticTokensRequest as lsp_types::Request>::Result =
                respond_to_semantic_tokens_full(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::CompletionRequest as lsp_types::Request>::METHOD => {
            let arguments: <lsp_types::CompletionRequest as lsp_types::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::CompletionRequest as lsp_types::Request>::Result =
                respond_to_completion(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::DocumentFormattingRequest as lsp_types::Request>::METHOD => {
            let arguments: <lsp_types::DocumentFormattingRequest as lsp_types::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::DocumentFormattingRequest as lsp_types::Request>::Result =
                respond_to_document_formatting(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::DocumentSymbolRequest as lsp_types::Request>::METHOD => {
            let arguments: <lsp_types::DocumentSymbolRequest as lsp_types::Request>::Params =
                serde_json::from_value(request_arguments_json)?;
            let result: <lsp_types::DocumentSymbolRequest as lsp_types::Request>::Result =
                respond_to_document_symbols(state, &arguments);
            Ok(serde_json::to_value(result)?)
        }
        <lsp_types::ShutdownRequest as lsp_types::Request>::METHOD => {
            let result: <lsp_types::ShutdownRequest as lsp_types::Request>::Result = ();
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
    diagnostics: <lsp_types::PublishDiagnosticsNotification as lsp_types::Notification>::Params,
) {
    let diagnostics_json: serde_json::Value = match serde_json::to_value(diagnostics) {
        Ok(diagnostics_json) => diagnostics_json,
        Err(err) => {
            eprintln!("failed to encode diagnostics {err}");
            return;
        }
    };
    connection
        .sender
        .send(lsp_server::Message::Notification(
            lsp_server::Notification {
                method:
                    <lsp_types::PublishDiagnosticsNotification as lsp_types::Notification>::METHOD
                        .to_string(),
                params: diagnostics_json,
            },
        ))
        .unwrap_or_else(|err| {
            eprintln!("failed to send diagnostics {err}");
        });
}

fn update_state_on_did_change_text_document<Expressions, Patterns, Types>(
    state: &mut State<Expressions, Patterns, Types>,
    connection: &lsp_server::Connection,
    did_change_text_document: lsp_types::DidChangeTextDocumentParams,
) {
    let project_count = state.projects.len();
    if let Some(project_state) = state.projects.get_mut(
        &did_change_text_document
            .text_document
            .text_document_identifier
            .uri,
    ) {
        let mut updated_source: String = std::mem::take(&mut project_state.source);
        for change in did_change_text_document.content_changes {
            match change {
                lsp_types::TextDocumentContentChangeEvent::TextDocumentContentChangeWholeDocument(new_text) => {
                    updated_source = new_text.text;
                }
                lsp_types::TextDocumentContentChangeEvent::TextDocumentContentChangePartial(change) => {
                    #[allow(deprecated)] match change.range_length {
                        // zed for example does not send a range length
                        None => {
                            string_replace_lsp_span(&mut updated_source, change.range, &change.text);
                        }
                        // sending a range length is deprecated but e.g. vscode still sends it
                        // which allows us to do a faster string replace
                        Some(range_length) => {
                            string_replace_lsp_span_for_length(
                                &mut updated_source,
                                change.range,
                                range_length as usize,
                                &change.text,
                            );
                        }
                    }
                }
            }
        }
        for syntax_project_element in project_state.syntax.elements.drain(..) {
            sloe::syntax_project_element_rid(
                syntax_project_element,
                &mut state.syntax_expressions,
                &mut state.syntax_patterns,
                &mut state.syntax_types,
            );
        }
        if project_count == 1 {
            fn vec_should_be_empty<Origin, Element>(vec: &sloe::core::Buf<Origin, Element>) {
                if !vec.maybe_uninit_elements().is_empty() || !vec.vacant_spans().is_empty() {
                    eprintln!(
                        "vec not empty after rid step. remaining vacant spans: {:?}, remaining elements ({} including vacant) maybe uninit: {:?}",
                        vec.vacant_spans(),
                        vec.maybe_uninit_elements().len(),
                        vec.maybe_uninit_elements()
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| {
                                !vec.vacant_spans()
                                    .iter()
                                    .any(|vacant_span| vacant_span.to_range().contains(i))
                            })
                    );
                }
            }
            vec_should_be_empty(&state.syntax_expressions);
            vec_should_be_empty(&state.syntax_patterns);
            vec_should_be_empty(&state.syntax_types);
        }
        *project_state = initialize_project_state_from_source(
            connection,
            did_change_text_document
                .text_document
                .text_document_identifier
                .uri,
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
    expressions: &mut sloe::core::Buf<
        Expressions,
        sloe::SyntaxExpression<Expressions, Patterns, Types>,
    >,
    patterns: &mut sloe::core::Buf<Patterns, sloe::SyntaxPattern<Patterns, Types>>,
    types: &mut sloe::core::Buf<Types, sloe::SyntaxType<Types>>,
    source: String,
) -> ProjectState<Expressions, Patterns, Types> {
    let parsed_project = sloe::parse_project(expressions, patterns, types, &source);
    let mut errors: Vec<sloe::ErrorNode> = Vec::new();
    let checked_project =
        sloe::syntax_project_check(&mut errors, &parsed_project, expressions, patterns, types);
    if let Some(input_file_path) = lsp_uri_to_file_path(&uri)
        && let output_file_path = default_output_file_path_for_sloe_input_file_path(
            input_file_path,
            CompileOutputLanguage::Rust,
        )
        && std::fs::exists(&output_file_path).is_ok_and(|exists| exists)
        && let Ok(output_mod_name) = rust_file_name_derive_mod_name(&output_file_path)
    {
        let compiled_project =
            sloe::checked_project_to_rust(&checked_project, expressions, patterns, types);
        let _: std::io::Result<()> = std::fs::write(
            &output_file_path,
            sloe::compiled_rust_to_file_content(&compiled_project, output_mod_name),
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
        type_aliases: checked_project.checked_type_aliases,
        fns: checked_project.checked_project_fns,
        queries: checked_project.checked_queries,
        spread_records: checked_project.checked_spread_records,
        syntax: parsed_project,
    }
}
fn sloe_error_node_to_diagnostic(problem: &sloe::ErrorNode) -> lsp_types::Diagnostic {
    lsp_types::Diagnostic {
        range: problem.range,
        severity: Some(lsp_types::DiagnosticSeverity::Warning),
        code: None,
        code_description: None,
        source: None,
        // preferably we would render errors as markdown
        // but this is not implemented in any big editors, yet (vscode, zed, gram, ...)
        message: lsp_types::Message::String(problem.message.to_string()),
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
    let Some(symbol) = sloe::project_symbol_at_position(
        &project_state.syntax,
        &project_state.type_aliases,
        &project_state.queries,
        &project_state.spread_records,
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
            construct_info: _,
            origins: _,
        } => project_state
            .type_aliases
            .iter()
            .find_map(|(type_alias_name, type_alias_info)| {
                if type_alias_name == symbol_name.value {
                    Some(lsp_types::Hover {
                        contents: lsp_types::Contents::MarkupContent(lsp_types::MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: present_type_alias_markdown(type_alias_name, type_alias_info),
                        }),
                        range: Some(sloe::name_range(symbol_name)),
                    })
                } else {
                    None
                }
            }),
        sloe::SyntaxSymbol::Origin {
            name,
            use_start,
            origin: _,
        } => Some(lsp_types::Hover {
            contents: lsp_types::Contents::MarkupContent(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: format!(
                    "Origin `^{name}` whose variable is of type
```sloe
Origin {name}, .{name} .
```
The first argument is a unique, local type with the same name as the variable.
The second argument is an empty record with a field of the same name.
It's used for APIs like `Origin-add`/`Origin-part` and `Origin-erase`"
                ),
            }),
            range: Some(sloe::name_range(sloe::WithStartPosition {
                start: use_start,
                value: name,
            })),
        }),
        sloe::SyntaxSymbol::TypeVariable {
            name,
            use_start,
            scope: _,
        } => Some(lsp_types::Hover {
            contents: lsp_types::Contents::MarkupContent(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: "type variable".to_string(),
            }),
            range: Some(sloe::name_range(sloe::WithStartPosition {
                start: use_start,
                value: name,
            })),
        }),
        sloe::SyntaxSymbol::VariantOrUnknown(_) => None,
        sloe::SyntaxSymbol::ProjectFnOrUnknown {
            name: symbol_name,
            construct_info: _,
            pattern_variables: _,
            origins: _,
        } => project_state.fns.iter().find_map(|(fn_name, fn_info)| {
            if fn_name == symbol_name.value {
                let formatted = present_project_fn_with_complete_type_markdown(fn_name, fn_info);
                Some(lsp_types::Hover {
                    contents: lsp_types::Contents::MarkupContent(lsp_types::MarkupContent {
                        kind: lsp_types::MarkupKind::Markdown,
                        value: formatted,
                    }),
                    range: Some(sloe::name_range(symbol_name)),
                })
            } else {
                None
            }
        }),
        sloe::SyntaxSymbol::PatternVariable {
            name,
            use_start,
            origin,
        } => Some(lsp_types::Hover {
            contents: lsp_types::Contents::MarkupContent(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: present_pattern_variable_markdown(origin.type_.as_ref()),
            }),
            range: Some(sloe::name_range(sloe::WithStartPosition {
                start: use_start,
                value: name,
            })),
        }),
    }
}

fn respond_to_goto_definition<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    goto_definition_arguments: lsp_types::DefinitionParams,
) -> Option<lsp_types::DefinitionResponse> {
    let Some(project_state) = state.projects.get(
        &goto_definition_arguments
            .text_document_position_params
            .text_document
            .uri,
    ) else {
        return None;
    };
    let Some(symbol) = sloe::project_symbol_at_position(
        &project_state.syntax,
        &project_state.type_aliases,
        &project_state.queries,
        &project_state.spread_records,
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
        sloe::SyntaxSymbol::VariantOrUnknown(_) => None,
        sloe::SyntaxSymbol::ProjectFnOrUnknown {
            name: symbol_name,
            construct_info: _,
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
                } if &fn_name.value == symbol_name.value => {
                    Some(sloe::name_range(sloe::with_start_position_as_ref(fn_name)))
                }
                _ => None,
            }),
        sloe::SyntaxSymbol::ProjectTypeOrUnknown {
            name: symbol_name,
            construct_info: _,
            origins: _,
        } => project_state
            .syntax
            .elements
            .iter()
            .find_map(|element| match element {
                sloe::SyntaxProjectElement::TypeAlias {
                    name: Some(type_alias_name),
                    ..
                } if &type_alias_name.value == symbol_name.value => Some(sloe::name_range(
                    sloe::with_start_position_as_ref(type_alias_name),
                )),
                _ => None,
            }),
        sloe::SyntaxSymbol::Origin {
            name,
            use_start: _,
            origin,
        } => Some(sloe::name_range(sloe::WithStartPosition {
            value: name,
            start: origin.start,
        })),
        sloe::SyntaxSymbol::TypeVariable { .. } => None,
        sloe::SyntaxSymbol::PatternVariable {
            name,
            use_start: _,
            origin,
        } => Some(sloe::name_range(sloe::WithStartPosition {
            value: name,
            start: origin.start,
        })),
    };
    origin_name_range.map(|origin_name_range| {
        lsp_types::DefinitionResponse::Definition(lsp_types::Definition::Location(
            lsp_types::Location {
                uri: goto_definition_arguments
                    .text_document_position_params
                    .text_document
                    .uri,
                range: origin_name_range,
            },
        ))
    })
}

fn respond_to_prepare_rename<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    prepare_rename_arguments: &lsp_types::PrepareRenameParams,
) -> Option<lsp_types::PrepareRenameResult> {
    let Some(project_state) = state.projects.get(
        &prepare_rename_arguments
            .text_document_position_params
            .text_document
            .uri,
    ) else {
        return None;
    };
    let Some(symbol) = sloe::project_symbol_at_position(
        &project_state.syntax,
        &project_state.type_aliases,
        &project_state.queries,
        &project_state.spread_records,
        prepare_rename_arguments
            .text_document_position_params
            .position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    let symbol_range = match symbol {
        sloe::SyntaxSymbol::ProjectTypeOrUnknown {
            name,
            construct_info: _,
            origins: _,
        } => sloe::name_range(name),
        sloe::SyntaxSymbol::Origin {
            name,
            use_start,
            origin: _,
        } => sloe::name_range(sloe::WithStartPosition {
            value: name,
            start: use_start,
        }),
        sloe::SyntaxSymbol::TypeVariable {
            name,
            use_start,
            scope: _,
        } => sloe::name_range(sloe::WithStartPosition {
            value: name,
            start: use_start,
        }),
        sloe::SyntaxSymbol::VariantOrUnknown(name) => sloe::name_range(name),
        sloe::SyntaxSymbol::ProjectFnOrUnknown {
            name,
            construct_info: _,
            pattern_variables: _,
            origins: _,
        } => sloe::name_range(name),
        sloe::SyntaxSymbol::PatternVariable {
            name,
            use_start,
            origin: _,
        } => sloe::name_range(sloe::WithStartPosition {
            value: name,
            start: use_start,
        }),
    };
    Some(lsp_types::PrepareRenameResult::Range(symbol_range))
}

fn respond_to_rename<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    rename_arguments: lsp_types::RenameParams,
) -> Option<Vec<lsp_types::DocumentChange>> {
    let Some(project_state) = state.projects.get(
        &rename_arguments
            .text_document_position_params
            .text_document
            .uri,
    ) else {
        return None;
    };
    let Some(symbol) = sloe::project_symbol_at_position(
        &project_state.syntax,
        &project_state.type_aliases,
        &project_state.queries,
        &project_state.spread_records,
        rename_arguments.text_document_position_params.position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    let mut symbol_ranges = sloe::syntax_project_symbol_uses(
        &project_state.syntax,
        &symbol,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    );
    if let Some(symbol_origin_range) =
        sloe::syntax_project_symbol_origin_range(&project_state.syntax, &symbol)
    {
        symbol_ranges.push(symbol_origin_range);
    }
    Some(vec![lsp_types::DocumentChange::TextDocumentEdit(
        lsp_types::TextDocumentEdit {
            text_document: lsp_types::OptionalVersionedTextDocumentIdentifier {
                text_document_identifier: lsp_types::TextDocumentIdentifier {
                    uri: rename_arguments
                        .text_document_position_params
                        .text_document
                        .uri,
                },
                version: None,
            },
            edits: symbol_ranges
                .into_iter()
                .map(|symbol_range| {
                    lsp_types::Edit::TextEdit(lsp_types::TextEdit {
                        range: symbol_range,
                        new_text: rename_arguments.new_name.clone(),
                    })
                })
                .collect(),
        },
    )])
}

fn respond_to_references<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    references_arguments: &lsp_types::ReferenceParams,
) -> Option<Vec<lsp_types::Location>> {
    let Some(project_state) = state.projects.get(
        &references_arguments
            .text_document_position_params
            .text_document
            .uri,
    ) else {
        return None;
    };
    let Some(symbol) = sloe::project_symbol_at_position(
        &project_state.syntax,
        &project_state.type_aliases,
        &project_state.queries,
        &project_state.spread_records,
        references_arguments.text_document_position_params.position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    let mut symbol_ranges = sloe::syntax_project_symbol_uses(
        &project_state.syntax,
        &symbol,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    );
    if references_arguments.context.include_declaration
        && let Some(symbol_origin_range) =
            sloe::syntax_project_symbol_origin_range(&project_state.syntax, &symbol)
    {
        symbol_ranges.push(symbol_origin_range);
    }
    Some(
        symbol_ranges
            .into_iter()
            .map(|range| lsp_types::Location {
                uri: references_arguments
                    .text_document_position_params
                    .text_document
                    .uri
                    .clone(),
                range: range,
            })
            .collect(),
    )
}

fn respond_to_semantic_tokens_full<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    semantic_tokens_arguments: &lsp_types::SemanticTokensParams,
) -> Option<lsp_types::SemanticTokens> {
    let Some(project_state) = state
        .projects
        .get(&semantic_tokens_arguments.text_document.uri)
    else {
        return None;
    };
    let mut highlight_state = sloe::HighlightState {
        tokens: Vec::with_capacity(project_state.source.len() / 2),
        previous_token_start: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    sloe::project_highlight(
        &mut highlight_state,
        &project_state.syntax,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    );
    Some(lsp_types::SemanticTokens {
        result_id: None,
        data: highlight_state.tokens,
    })
}

fn lsp_position_add_characters(
    position: lsp_types::Position,
    additional_characters: u32,
) -> lsp_types::Position {
    lsp_types::Position {
        line: position.line,
        character: position.character + additional_characters,
    }
}

fn respond_to_completion<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    completion_arguments: &lsp_types::CompletionParams,
) -> Option<lsp_types::CompletionResponse> {
    let Some(project_state) = state.projects.get(
        &completion_arguments
            .text_document_position_params
            .text_document
            .uri,
    ) else {
        return None;
    };
    let Some(symbol) = sloe::project_symbol_at_position(
        &project_state.syntax,
        &project_state.type_aliases,
        &project_state.queries,
        &project_state.spread_records,
        completion_arguments.text_document_position_params.position,
        &state.syntax_expressions,
        &state.syntax_patterns,
        &state.syntax_types,
    ) else {
        return None;
    };
    match symbol {
        sloe::SyntaxSymbol::ProjectTypeOrUnknown {
            name: _,
            construct_info,
            origins,
        } => match construct_info {
            sloe_compile::ConstructInfo::Declaration => None,
            sloe_compile::ConstructInfo::NotExpectingArgument => {
                Some(lsp_types::CompletionResponse::CompletionItemList(
                    project_state
                        .type_aliases
                        .iter()
                        .filter_map(|(type_alias_name, type_alias_info)| {
                            match type_alias_info.parameters.as_slice() {
                                [_, ..] => None,
                                [] => Some(lsp_types::CompletionItem {
                                    label: type_alias_name.to_string(),
                                    kind: Some(lsp_types::CompletionItemKind::Struct),
                                    documentation: Some(lsp_documentation_markdown(
                                        present_type_alias_markdown(
                                            type_alias_name,
                                            type_alias_info,
                                        ),
                                    )),
                                    ..lsp_types::CompletionItem::default()
                                }),
                            }
                        })
                        .chain(
                            origins
                                .into_keys()
                                .map(|origin_name| lsp_types::CompletionItem {
                                    label: origin_name.to_string(),
                                    kind: Some(lsp_types::CompletionItemKind::Struct),
                                    documentation: Some(lsp_documentation_markdown(format!(
                                        "```sloe\n^{}\n```",
                                        origin_name
                                    ))),
                                    ..lsp_types::CompletionItem::default()
                                }),
                        )
                        .collect(),
                ))
            }
            sloe_compile::ConstructInfo::ArgumentMissing => {
                Some(lsp_types::CompletionResponse::CompletionItemList(
                    project_state
                        .type_aliases
                        .iter()
                        .filter_map(|(type_alias_name, type_alias_info)| {
                            match type_alias_info.parameters.as_slice() {
                                [] => None,
                                [parameter0, parameter1_up @ ..] => {
                                    let mut snippet = String::new();
                                    snippet.push('(');
                                    snippet.push_str(type_alias_name);
                                    snippet.push_str(" (${1:");
                                    snippet.push_str(parameter0);
                                    snippet.push_str("})");
                                    use std::fmt::Write as _;
                                    for (parameter_position, parameter) in
                                        parameter1_up.iter().enumerate().map(|(i, e)| (i + 2, e))
                                    {
                                        snippet.push_str(", (${");
                                        let _ = write!(snippet, "{}", parameter_position);
                                        snippet.push(':');
                                        snippet.push_str(parameter);
                                        snippet.push_str("})");
                                    }
                                    snippet.push(')');
                                    Some(lsp_types::CompletionItem {
                                        label: type_alias_name.to_string(),
                                        kind: Some(lsp_types::CompletionItemKind::Struct),
                                        documentation: Some(lsp_documentation_markdown(
                                            present_type_alias_markdown(
                                                type_alias_name,
                                                type_alias_info,
                                            ),
                                        )),
                                        insert_text_format: Some(
                                            lsp_types::InsertTextFormat::Snippet,
                                        ),
                                        insert_text: Some(snippet),
                                        ..lsp_types::CompletionItem::default()
                                    })
                                }
                            }
                        })
                        .collect(),
                ))
            }
            sloe_compile::ConstructInfo::ArgumentExists => {
                Some(lsp_types::CompletionResponse::CompletionItemList(
                    project_state
                        .type_aliases
                        .iter()
                        .filter_map(|(type_alias_name, type_alias_info)| {
                            match type_alias_info.parameters.as_slice() {
                                [] => None,
                                [_, ..] => Some(lsp_types::CompletionItem {
                                    label: type_alias_name.to_string(),
                                    kind: Some(lsp_types::CompletionItemKind::Struct),
                                    documentation: Some(lsp_documentation_markdown(
                                        present_type_alias_markdown(
                                            type_alias_name,
                                            type_alias_info,
                                        ),
                                    )),
                                    ..lsp_types::CompletionItem::default()
                                }),
                            }
                        })
                        .collect(),
                ))
            }
        },
        sloe::SyntaxSymbol::Origin { .. } => None,
        sloe::SyntaxSymbol::TypeVariable {
            name: _,
            use_start,
            scope,
        } => {
            let mut available_existing_variables = std::collections::HashSet::new();
            match scope {
                sloe::SyntaxProjectElement::TypeAlias {
                    ty_keyword_start: _,
                    name: _,
                    parameters,
                    documentation: _,
                    type_: _,
                } => {
                    if let Some(parameters) = parameters {
                        for (underscore_start, name) in std::iter::once((
                            parameters.parameter0_underscore_start,
                            &parameters.parameter0,
                        ))
                        .chain(
                            parameters.parameter1_up.iter().filter_map(|parameter| {
                                parameter
                                    .underscore_start
                                    .map(|underscore_start| (underscore_start, &parameter.name))
                            }),
                        ) {
                            if lsp_position_add_characters(underscore_start, 1) == use_start {
                                return None;
                            }
                            available_existing_variables.insert(name);
                        }
                    }
                }
                sloe::SyntaxProjectElement::Fn {
                    fn_keyword_start: _,
                    name: _,
                    type_parameters,
                    parameter,
                    colon_start: _,
                    result_type,
                    equals_start: _,
                    documentation: _,
                    result: _,
                } => {
                    for type_parameter in type_parameters {
                        if let Some(underscore_start) = type_parameter.underscore_start {
                            if lsp_position_add_characters(underscore_start, 1) == use_start {
                                return None;
                            }
                            available_existing_variables.insert(&type_parameter.name);
                        }
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
            Some(lsp_types::CompletionResponse::CompletionItemList(
                available_existing_variables
                    .into_iter()
                    .map(|available_existing_variable| lsp_types::CompletionItem {
                        label: format!("_{}", available_existing_variable),
                        insert_text: Some(available_existing_variable.to_string()),
                        kind: Some(lsp_types::CompletionItemKind::TypeParameter),
                        documentation: Some(lsp_documentation_markdown(
                            "type variable".to_string(),
                        )),
                        ..lsp_types::CompletionItem::default()
                    })
                    .collect(),
            ))
        }
        sloe::SyntaxSymbol::VariantOrUnknown(_) => {
            // improvement possibility: if type is known (aka query pattern), suggest all names from the choice,
            // otherwise suggest all known names and add <> after it on completion
            None
        }
        sloe::SyntaxSymbol::ProjectFnOrUnknown {
            name: _,
            construct_info,
            pattern_variables,
            origins,
        } => match construct_info {
            sloe_compile::ConstructInfo::Declaration => None,
            sloe_compile::ConstructInfo::NotExpectingArgument => {
                Some(lsp_types::CompletionResponse::CompletionItemList(
                    project_state
                        .fns
                        .iter()
                        .filter_map(|(fn_name, fn_info)| {
                            if fn_info.type_parameters.is_empty() {
                                Some(lsp_types::CompletionItem {
                                    label: fn_name.to_string(),
                                    kind: Some(lsp_types::CompletionItemKind::Function),
                                    documentation: Some(lsp_documentation_markdown(
                                        present_project_fn_with_complete_type_markdown(
                                            fn_name, fn_info,
                                        ),
                                    )),
                                    ..lsp_types::CompletionItem::default()
                                })
                            } else {
                                None
                            }
                        })
                        .chain(pattern_variables.into_iter().map(
                            |(pattern_variable, pattern_variable_origin)| {
                                lsp_types::CompletionItem {
                                    label: pattern_variable.to_string(),
                                    kind: Some(lsp_types::CompletionItemKind::Variable),
                                    documentation: Some(lsp_documentation_markdown(
                                        present_pattern_variable_markdown(
                                            pattern_variable_origin.type_.as_ref(),
                                        ),
                                    )),
                                    ..lsp_types::CompletionItem::default()
                                }
                            },
                        ))
                        .chain(
                            origins
                                .into_keys()
                                .map(|origin_name| lsp_types::CompletionItem {
                                    label: origin_name.to_string(),
                                    kind: Some(lsp_types::CompletionItemKind::Variable),
                                    documentation: Some(lsp_documentation_markdown(
                                        "^origin variable".to_string(),
                                    )),
                                    ..lsp_types::CompletionItem::default()
                                }),
                        )
                        .collect(),
                ))
            }
            sloe_compile::ConstructInfo::ArgumentMissing => {
                Some(lsp_types::CompletionResponse::CompletionItemList(
                    project_state
                        .fns
                        .iter()
                        .map(|(fn_name, fn_info)| {
                            let mut snippet = String::new();
                            snippet.push('(');
                            snippet.push_str(fn_name);
                            use std::fmt::Write as _;
                            for (type_parameter_index, type_parameter) in
                                fn_info.type_parameters.iter().enumerate()
                            {
                                snippet.push_str("{${");
                                let _ = write!(snippet, "{}", type_parameter_index);
                                snippet.push_str(":_");
                                snippet.push_str(type_parameter);
                                snippet.push_str("}}");
                            }
                            if let Some(parameter_type) = &fn_info.parameter_type {
                                match parameter_type {
                                    sloe::Type::Record(parameter_fields) => {
                                        for (field_index, field) in
                                            parameter_fields.iter().enumerate()
                                        {
                                            snippet.push_str(" .");
                                            snippet.push_str(&field.name);
                                            snippet.push_str(" (${");
                                            let _ = write!(
                                                snippet,
                                                "{}",
                                                fn_info.type_parameters.len() + 1 + field_index
                                            );
                                            snippet.push(':');
                                            snippet.push_str(&field.name);
                                            snippet.push_str("})");
                                        }
                                    }
                                    sloe::Type::Variable(name) => {
                                        snippet.push_str(" (${");
                                        let _ = write!(
                                            snippet,
                                            "{}",
                                            fn_info.type_parameters.len() + 1
                                        );
                                        snippet.push(':');
                                        snippet.push_str(name);
                                        snippet.push_str("})");
                                    }
                                    sloe::Type::Origin(origin_name) => {
                                        snippet.push_str(" (${");
                                        let _ = write!(
                                            snippet,
                                            "{}",
                                            fn_info.type_parameters.len() + 1
                                        );
                                        snippet.push(':');
                                        snippet.push_str(origin_name);
                                        snippet.push_str("})");
                                    }
                                    sloe::Type::Choice(_) => {
                                        snippet.push_str(" ($1)");
                                    }
                                    sloe::Type::CoreConstruct { .. } => {
                                        snippet.push_str(" ($1)");
                                    }
                                }
                            }
                            snippet.push(')');
                            lsp_types::CompletionItem {
                                label: fn_name.to_string(),
                                kind: Some(lsp_types::CompletionItemKind::Function),
                                documentation: Some(lsp_documentation_markdown(
                                    present_project_fn_with_complete_type_markdown(
                                        fn_name, fn_info,
                                    ),
                                )),
                                insert_text_format: Some(lsp_types::InsertTextFormat::Snippet),
                                insert_text: Some(snippet),
                                ..lsp_types::CompletionItem::default()
                            }
                        })
                        .collect(),
                ))
            }
            sloe_compile::ConstructInfo::ArgumentExists => {
                Some(lsp_types::CompletionResponse::CompletionItemList(
                    project_state
                        .fns
                        .iter()
                        .map(|(fn_name, fn_info)| lsp_types::CompletionItem {
                            label: fn_name.to_string(),
                            kind: Some(lsp_types::CompletionItemKind::Function),
                            documentation: Some(lsp_documentation_markdown(
                                present_project_fn_with_complete_type_markdown(fn_name, fn_info),
                            )),
                            ..lsp_types::CompletionItem::default()
                        })
                        .collect(),
                ))
            }
        },
        sloe::SyntaxSymbol::PatternVariable {
            name: _,
            use_start: _,
            origin: _,
        } => None,
    }
}
fn lsp_documentation_markdown(markdown: String) -> lsp_types::Documentation {
    lsp_types::Documentation::MarkupContent(lsp_types::MarkupContent {
        kind: lsp_types::MarkupKind::Markdown,
        value: markdown,
    })
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
    Some(lsp_types::DocumentSymbolResponse::DocumentSymbolList(
        project
            .syntax
            .elements
            .iter()
            .filter_map(|project_element| match project_element {
                sloe::SyntaxProjectElement::Comments { .. } => None,
                sloe::SyntaxProjectElement::Unrecognized { .. } => None,
                sloe::SyntaxProjectElement::TypeAlias {
                    ty_keyword_start: _,
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
                        kind: lsp_types::SymbolKind::Struct,
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
                        selection_range: sloe::name_range(sloe::with_start_position_as_ref(name)),
                        children: None,
                    })
                }
                sloe::SyntaxProjectElement::Fn {
                    fn_keyword_start,
                    name,
                    type_parameters: _,
                    parameter: _,
                    colon_start: _,
                    result_type: _,
                    equals_start: _,
                    documentation: _,
                    result: maybe_result,
                } => {
                    let Some(name) = name else {
                        return None;
                    };
                    Some(lsp_types::DocumentSymbol {
                        name: name.value.to_string(),
                        detail: None,
                        kind: lsp_types::SymbolKind::Function,
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
                        selection_range: sloe::name_range(sloe::with_start_position_as_ref(name)),
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

/// This not airtight and thus should not be relied upon for critical code.
/// For example, this does not work for localhost or file:... redox uris.
/// If a better solution is needed in the future, use the `url` crate
fn lsp_uri_to_file_path(uri: &lsp_types::Uri) -> Option<&'_ std::path::Path> {
    uri.0
        .strip_prefix("file://")
        .map(|path| std::path::Path::new(path))
}
