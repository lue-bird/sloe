#![allow(non_upper_case_globals)]
use sloe_compile as sloe;

struct State<Expressions, Patterns, Types> {
    projects: std::collections::HashMap<lsp_types::Uri, ProjectState<Expressions, Patterns, Types>>,
    syntax_expressions: sloe::core::Vec<Expressions, sloe::SyntaxExpression<Expressions, Patterns>>,
    syntax_patterns: sloe::core::Vec<Patterns, sloe::SyntaxPattern<Patterns, Types>>,
    syntax_types: sloe::core::Vec<Types, sloe::SyntaxType<Types>>,
}
struct ProjectState<Expressions, Patterns, Types> {
    source: String,
    syntax: sloe::SyntaxProject<Expressions, Patterns, Types>,
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
                print!(include_str!("core-types.sloe"));
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
        r#"// enabling deref_patterns is sadly required for matching recursive choice types
#![feature(deref_patterns)]
#![allow(incomplete_features)]

mod sloe;

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
        "rust-toolchain.toml",
        "this allows rust tooling to build the project with nightly features",
        r#"[toolchain]
channel = "nightly"
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
            let mut expressions = sloe::core::vec_empty(sloe::core::origin_new!());
            let mut patterns = sloe::core::vec_empty(sloe::core::origin_new!());
            let syntax_project =
                sloe::parse_syntax_project(&mut expressions, &mut patterns, &project_source);
            let mut output_errors: Vec<sloe::ErrorNode> = Vec::new();
            let compiled_project: sloe::CompiledProject =
                sloe::project_compile_to_rust(&mut output_errors, &syntax_project);
            for output_error in &output_errors {
                eprintln!(
                    "{input_file_path:?}:{span_start_line}:{span_start_column} {message}",
                    span_start_line = output_error.span.start.line + 1,
                    span_start_column = output_error.span.start.character + 1,
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
    let state = initial_state(
        sloe::core::origin_new!(),
        sloe::core::origin_new!(),
        sloe::core::origin_new!(),
    );
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
                    span: None,
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
        Ok(response_value) => {
            send_response_ok(connection, request_id, response_value)?;
        }
        Err(response_error) => send_response_error(connection, request_id, response_error)?,
    }
    Ok(())
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
        .send(lsp_server::Message::Response(response))?;
    Ok(())
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
        .send(lsp_server::Message::Response(response))?;
    Ok(())
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
            match (change.span, change.span_length) {
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
            updated_source,
        );
    }
}

fn initialize_project_state_from_source<Expressions, Patterns>(
    connection: &lsp_server::Connection,
    uri: lsp_types::Uri,
    expressions: &mut sloe::core::Vec<Expressions, sloe::SyntaxExpression<Expressions, Patterns>>,
    patterns: &mut sloe::core::Vec<Patterns, sloe::SyntaxPattern>,
    source: String,
) -> ProjectState<Expressions, Patterns> {
    let parsed_project = sloe::parse_syntax_project(expressions, patterns, &source);
    let mut errors: Vec<sloe::ErrorNode> = Vec::new();
    let compiled_project: sloe::CompiledProject =
        sloe::project_compile_to_rust(&mut errors, &parsed_project);
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
        choice_types: compiled_project.choice_types,
        fns: compiled_project.fns,
        records: compiled_project.records,
        syntax: parsed_project,
    }
}
fn sloe_error_node_to_diagnostic(problem: &sloe::ErrorNode) -> lsp_types::Diagnostic {
    lsp_types::Diagnostic {
        span: problem.span,
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
    let hovered_project_state = state.projects.get(
        &hover_arguments
            .text_document_position_params
            .text_document
            .uri,
    )?;
    todo!()
}

fn respond_to_goto_definition<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    goto_definition_arguments: lsp_types::GotoDefinitionParams,
) -> Option<lsp_types::GotoDefinitionResponse> {
    let goto_symbol_project_state = state.projects.get(
        &goto_definition_arguments
            .text_document_position_params
            .text_document
            .uri,
    )?;
    todo!()
}

fn respond_to_prepare_rename<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    prepare_rename_arguments: &lsp_types::TextDocumentPositionParams,
) -> Option<Result<lsp_types::PrepareRenameResponse, lsp_server::ResponseError>> {
    let project_state = state
        .projects
        .get(&prepare_rename_arguments.text_document.uri)?;
    todo!()
}

fn respond_to_rename<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    rename_arguments: lsp_types::RenameParams,
) -> Option<Vec<lsp_types::TextDocumentEdit>> {
    let to_prepare_for_rename_project_state = state
        .projects
        .get(&rename_arguments.text_document_position.text_document.uri)?;
    todo!()
}

fn respond_to_references<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    references_arguments: &lsp_types::ReferenceParams,
) -> Option<Vec<lsp_types::Location>> {
    let to_find_project_state = state.projects.get(
        &references_arguments
            .text_document_position
            .text_document
            .uri,
    )?;
    todo!()
}

fn respond_to_semantic_tokens_full<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    semantic_tokens_arguments: &lsp_types::SemanticTokensParams,
) -> Option<lsp_types::SemanticTokensResult> {
    let project_state = state
        .projects
        .get(&semantic_tokens_arguments.text_document.uri)?;
    let mut highlighting: Vec<sloe::SyntaxNode<sloe::SyntaxHighlightKind>> =
        Vec::with_capacity(project_state.source.len() / 16);
    todo!();
    Some(lsp_types::SemanticTokensResult::Tokens(
        lsp_types::SemanticTokens {
            result_id: None,
            data: highlighting
                .into_iter()
                .scan(
                    lsp_types::Position {
                        line: 0,
                        character: 0,
                    },
                    |previous_start_location, segment| {
                        if (segment.span.end.line != segment.span.start.line)
                            || (segment.span.end.character < segment.span.start.character)
                        {
                            eprintln!(
                                "bad highlight token span: must be single-line and positive {:?}",
                                segment.span
                            );
                            return None;
                        }
                        match lsp_position_positive_delta(
                            *previous_start_location,
                            segment.span.start,
                        ) {
                            Err(error) => {
                                eprintln!("bad highlight token order {error}");
                                None
                            }
                            Ok(delta) => {
                                let token = lsp_types::SemanticToken {
                                    delta_line: delta.line,
                                    delta_start: delta.character,
                                    length: segment.span.end.character
                                        - segment.span.start.character,
                                    token_type: semantic_token_type_to_id(
                                        &sloe_syntax_highlight_kind_to_lsp_semantic_token_type(
                                            segment.value,
                                        ),
                                    ),
                                    token_modifiers_bitset: 0_u32,
                                };
                                segment.span.start.clone_into(previous_start_location);
                                Some(token)
                            }
                        }
                    },
                )
                .collect::<Vec<lsp_types::SemanticToken>>(),
        },
    ))
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
fn sloe_syntax_highlight_kind_to_lsp_semantic_token_type(
    sloe_syntax_highlight_kind: sloe::SyntaxHighlightKind,
) -> lsp_types::SemanticTokenType {
    match sloe_syntax_highlight_kind {
        sloe::SyntaxHighlightKind::KeySymbol => lsp_types::SemanticTokenType::KEYWORD,
        sloe::SyntaxHighlightKind::Field => lsp_types::SemanticTokenType::PROPERTY,
        sloe::SyntaxHighlightKind::Type => lsp_types::SemanticTokenType::TYPE,
        sloe::SyntaxHighlightKind::Variable => lsp_types::SemanticTokenType::VARIABLE,
        sloe::SyntaxHighlightKind::Variant => lsp_types::SemanticTokenType::ENUM_MEMBER,
        sloe::SyntaxHighlightKind::DeclaredVariable => lsp_types::SemanticTokenType::FUNCTION,
        sloe::SyntaxHighlightKind::Comment => lsp_types::SemanticTokenType::COMMENT,
        sloe::SyntaxHighlightKind::Number => lsp_types::SemanticTokenType::NUMBER,
        sloe::SyntaxHighlightKind::String => lsp_types::SemanticTokenType::STRING,
        sloe::SyntaxHighlightKind::TypeVariable => lsp_types::SemanticTokenType::TYPE_PARAMETER,
    }
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
    let completion_project = state.projects.get(
        &completion_arguments
            .text_document_position
            .text_document
            .uri,
    )?;
    todo!()
}

fn respond_to_document_formatting<Expressions, Patterns, Types>(
    state: &State<Expressions, Patterns, Types>,
    formatting_arguments: &lsp_types::DocumentFormattingParams,
) -> Option<Vec<lsp_types::TextEdit>> {
    let to_format_project = state
        .projects
        .get(&formatting_arguments.text_document.uri)?;
    let formatted: String = todo!();
    // diffing does not seem to be needed here. But maybe it's faster?
    Some(vec![lsp_types::TextEdit {
        span: lsp_types::Span {
            start: lsp_types::Position {
                line: 0,
                character: 0,
            },
            end: lsp_types::Position {
                line: to_format_project.source.lines().count() as u32
                    + (
                        // restore last line break potentially eaten by .lines()
                        if to_format_project.source.ends_with(['\r', '\n']) {
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
                sloe::SyntaxProjectElement::Unrecognized(_) => None,
                sloe::SyntaxProjectElement::ChoiceType {
                    name: maybe_name,
                    parameters: _,
                    variants,
                } => {
                    let name_node = maybe_name.as_ref()?;
                    Some(lsp_types::DocumentSymbol {
                        name: name_node.value.to_string(),
                        detail: None,
                        kind: lsp_types::SymbolKind::ENUM,
                        tags: None,
                        #[allow(deprecated)]
                        deprecated: None,
                        span: lsp_types::Span {
                            start: name_node.span.start,
                            end: variants
                                .last()
                                .and_then(|variant| {
                                    variant
                                        .value
                                        .as_ref()
                                        .map(|value| value.span.end)
                                        .or_else(|| variant.name.as_ref().map(|n| n.span.end))
                                })
                                .unwrap_or_else(|| name_node.span.end),
                        },
                        selection_span: name_node.span,
                        children: Some(
                            variants
                                .iter()
                                .filter_map(|variant| {
                                    let variant_name_node = variant.name.as_ref()?;
                                    Some((
                                        variant_name_node,
                                        lsp_types::Span {
                                            start: variant_name_node.span.start,
                                            end: variant
                                                .value
                                                .as_ref()
                                                .map(|node| node.span.end)
                                                .unwrap_or(variant_name_node.span.end),
                                        },
                                    ))
                                })
                                .map(|(variant_name_node, variant_full_span)| {
                                    lsp_types::DocumentSymbol {
                                        name: variant_name_node.value.to_string(),
                                        detail: None,
                                        kind: lsp_types::SymbolKind::ENUM_MEMBER,
                                        tags: None,
                                        #[allow(deprecated)]
                                        deprecated: None,
                                        span: variant_full_span,
                                        selection_span: variant_name_node.span,
                                        children: None,
                                    }
                                })
                                .collect::<Vec<_>>(),
                        ),
                    })
                }
                sloe::SyntaxProjectElement::Fn {
                    name: maybe_name,
                    result: maybe_result,
                } => {
                    let name_node = maybe_name.as_ref()?;
                    Some(lsp_types::DocumentSymbol {
                        name: name_node.value.to_string(),
                        detail: None,
                        kind: lsp_types::SymbolKind::FUNCTION,
                        tags: None,
                        #[allow(deprecated)]
                        deprecated: None,
                        span: lsp_types::Span {
                            start: name_node.span.start,
                            end: maybe_result
                                .as_ref()
                                .map(|n| n.span.end)
                                .unwrap_or_else(|| name_node.span.end),
                        },
                        selection_span: name_node.span,
                        children: None,
                    })
                }
            })
            .collect::<Vec<_>>(),
    ))
}

fn lsp_position_add_characters(
    position: lsp_types::Position,
    additional_character_count: i32,
) -> lsp_types::Position {
    lsp_types::Position {
        line: position.line,
        character: (position.character as i32 + additional_character_count) as u32,
    }
}

fn lsp_span_includes_position(span: lsp_types::Span, position: lsp_types::Position) -> bool {
    (
        // position >= span.start
        (position.line > span.start.line)
            || ((position.line == span.start.line) && (position.character >= span.start.character))
    ) && (
        // position <= span.end
        (position.line < span.end.line)
            || ((position.line == span.end.line) && (position.character <= span.end.character))
    )
}

fn str_lsp_span_to_span(str: &str, span: lsp_types::Span) -> std::ops::Span<usize> {
    let start_line_offset: usize = str_offset_after_n_lsp_linebreaks(str, span.start.line as usize);
    let start_offset: usize = start_line_offset
        + str_starting_utf8_length_for_utf16_length(
            &str[start_line_offset..],
            span.start.character as usize,
        );
    // can be optimized by only counting after the start line
    let end_line_offset: usize = str_offset_after_n_lsp_linebreaks(str, span.end.line as usize);
    let end_offset: usize = end_line_offset
        + str_starting_utf8_length_for_utf16_length(
            &str[end_line_offset..],
            span.end.character as usize,
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
fn string_replace_lsp_span(string: &mut String, span: lsp_types::Span, replacement: &str) {
    string.replace_span(str_lsp_span_to_span(string, span), replacement);
}
/// slightly faster version of `string_replace_lsp_span` for when you know the length
fn string_replace_lsp_span_for_length(
    string: &mut String,
    span: lsp_types::Span,
    span_length: usize,
    replacement: &str,
) {
    let start_line_offset: usize =
        str_offset_after_n_lsp_linebreaks(string, span.start.line as usize);
    let start_offset: usize = start_line_offset
        + str_starting_utf8_length_for_utf16_length(
            &string[start_line_offset..],
            span.start.character as usize,
        );
    let span_length_utf8: usize =
        str_starting_utf8_length_for_utf16_length(&string[start_offset..], span_length);
    string.replace_span(start_offset..(start_offset + span_length_utf8), replacement);
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
