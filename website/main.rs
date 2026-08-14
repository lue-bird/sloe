#![allow(non_upper_case_globals)]

use gen_lsp_types as lsp_types;
use sloe_compile as sloe;

fn main() {
    yew::Renderer::<State>::new().render();
}

struct State {
    text_area_content: String,
    selected_example: Example,
    // there must be a better way to cache this...
    sloe_core_declarations_html_static: yew::Html,
}
enum Event {
    TextAreaContentChanged(String),
    ExampleSelected(Example),
}
impl yew::Component for State {
    type Message = Event;

    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        let selected_example: Example = web_sys::window()
            .and_then(|window| window.location().search().ok())
            .and_then(|search| {
                let example_name = search.trim_start_matches("?example=");
                example_infos
                    .into_iter()
                    .find(|(_, example_info)| example_info.name.replace(' ', "-") == example_name)
                    .map(|(example, _)| example)
            })
            .unwrap_or(Example::HelloWorld);
        State {
            text_area_content: example_source(selected_example).to_string(),
            selected_example,
            sloe_core_declarations_html_static: sloe_core_declarations_html(),
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, event: Event) -> bool {
        match event {
            Event::TextAreaContentChanged(new_text_area_content) => {
                self.text_area_content = new_text_area_content;
            }
            Event::ExampleSelected(selected_example) => {
                self.selected_example = selected_example;
                self.text_area_content = example_source(selected_example).to_string();
                if let Some(window) = web_sys::window()
                    && let Ok(history) = window.history()
                {
                    let _ = history.push_state_with_url(
                        &web_sys::wasm_bindgen::JsValue::NULL,
                        "",
                        Some(&format!(
                            "{}?example={}",
                            window
                                .location()
                                .pathname()
                                .unwrap_or_else(|_| String::new()),
                            example_name(selected_example).replace(' ', "-")
                        )),
                    );
                }
            }
        }
        true
    }

    fn view(&self, context: &yew::Context<Self>) -> yew::Html {
        html_element(
            "main",
            [],
            [
                html_element(
                    "h2",
                    [("style", "white-space: pre-line;".into())],
                    [html_text(
                        "small, fast programming language
where indexes are valid
and values can't be shared: sloe",
                    )],
                ),
                html_link_to("https://codeberg.org/lue-bird/sloe", "source code"),
                html_text(". Try an example: "),
                html_element(
                    "p",
                    [
                        ("id", "example-select".into()),
                        ("style", "display: inline".into()),
                    ],
                    example_infos
                        .into_iter()
                        .map(|(example_kind, example_info)| {
                            let mut button = yew::virtual_dom::VTag::new("button");
                            let link = context.link().clone();
                            button.add_listener(std::rc::Rc::new(yew_listener(
                                yew::virtual_dom::ListenerKind::onpointerdown,
                                move |_| {
                                    link.send_message(Event::ExampleSelected(example_kind));
                                },
                            )));
                            button.add_child(html_text(example_info.name));
                            yew::Html::from(button)
                        }),
                ),
                linebreak_html(),
                playground_html(self.selected_example, &self.text_area_content, context),
                installation_html(),
                usage_html(),
                self.sloe_core_declarations_html_static.clone(),
            ],
        )
    }
}
fn installation_html() -> yew::Html {
    html_element(
        "section",
        [],
        [
            sub_heading_html("install"),
            html_element(
                "ol",
                [],
                [
                    html_link_to("https://rust-lang.org/tools/install/", "install rust"),
                    html_element(
                        "code",
                        [],
                        [html_text(
                            "cargo install --git https://github.com/lue-bird/sloe sloe",
                        )],
                    ),
                ]
                .map(|item| html_element("li", [], [item])),
            ),
        ],
    )
}
fn usage_html() -> yew::Html {
    html_element(
        "section",
        [],
        [
            sub_heading_html("use"),
            html_element(
                "ul",
                [],
                [
                    html_link_to(
                        "https://codeberg.org/lue-bird/sloe#editor-setups",
                        "detailed lsp setups and extensions",
                    ),
                    html_link_to(
                        "https://codeberg.org/lue-bird/sloe",
                        "more examples of e.g. compiling to zig or rust",
                    ),
                ]
                .map(|item| html_element("li", [], [item])),
            ),
        ],
    )
}
fn playground_html(
    selected_example: Example,
    text_area_content: &str,
    context: &yew::Context<State>,
) -> yew::Html {
    // stacked on top but still allow filling height:
    // https://stackoverflow.com/a/51949049
    // Originally I was doing position:absolute for the text area
    // and field-sizing: content and as a fallback for firefox height: line count * factor
    // but this ended up an infuriating mess since height:..em was rendering at different lengths in gecko.
    // I also tried display:flex but this didn't consistently ignore the textarea space and also didn't fill its height
    let mut text_area_stack = yew::virtual_dom::VTag::new("div");
    text_area_stack.add_attribute("style", "display: grid;");

    let mut interactive_text_area = yew::virtual_dom::VTag::new("textarea");
    interactive_text_area.add_attribute("autocorrect", "off");
    interactive_text_area.add_attribute("spellcheck", "false");
    interactive_text_area.add_attribute("autofocus", "true");
    interactive_text_area.add_attribute("name", "playground");
    interactive_text_area.add_attribute(
        "style",
        r#"grid-column: 1;
        grid-row: 1;
        height: 100%;
        width: 100%;
        background: none;
        color: transparent;
        border: none;
        line-height: inherit;
        resize: none;
        overflow: hidden;
        font-family: "Liga NovaMono", monospace, sans-serif;
        font-size: medium;
        resize: none;
        caret-color: white;
        position: relative;
        top: 0.335em;
        left: -0.1em"#,
    );
    interactive_text_area.add_property("value", text_area_content);
    let cursor_offset = text_area_content.find("insert your name here").unwrap_or(0);
    interactive_text_area.add_property("selectionStart", cursor_offset);
    interactive_text_area.add_property("selectionEnd", cursor_offset);
    // this seems a bit over the top for a simple event handler
    let link = context.link().clone();
    interactive_text_area.add_listener(std::rc::Rc::new(yew_listener(
        yew::virtual_dom::ListenerKind::oninput,
        move |event: web_sys::Event| {
            let Some(event_target) = event.target() else {
                return;
            };
            let text_area_object: web_sys::HtmlTextAreaElement = web_sys::HtmlTextAreaElement::from(
                web_sys::wasm_bindgen::JsValue::from(event_target),
            );
            link.send_message(Event::TextAreaContentChanged(text_area_object.value()));
        },
    )));
    text_area_stack.add_child(interactive_text_area.into());

    sloe::core::origin_new!(expressions_origin, Expressions);
    sloe::core::origin_new!(patterns_origin, Patterns);
    sloe::core::origin_new!(types_origin, Types);
    let mut expressions = sloe::core::Buf::new(expressions_origin);
    let mut patterns = sloe::core::Buf::new(patterns_origin);
    let mut types = sloe::core::Buf::new(types_origin);
    let syntax_project = sloe::parse_project(
        &mut expressions,
        &mut patterns,
        &mut types,
        text_area_content,
    );
    let mut highlights = sloe::HighlightState {
        tokens: Vec::with_capacity(text_area_content.len() / 2),
        previous_token_start: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    sloe::project_highlight(
        &mut highlights,
        &syntax_project,
        &expressions,
        &patterns,
        &types,
    );
    text_area_stack.add_child(html_element(
        "div",
        [
            ("aria-hidden", "true".into()),
            (
                "style",
                "grid-column: 1; grid-row: 1; z-index: 1; pointer-events: none; user-select: none;"
                    .into(),
            ),
        ],
        [highlighted_sloe_source_to_html(
            text_area_content,
            &mut highlights.tokens.into_iter(),
        )],
    ));
    let mut full = yew::virtual_dom::VTag::new("div");
    full.add_child(text_area_stack.into());

    let mut evaluated_variables_html = yew::virtual_dom::VTag::new("ul");
    evaluated_variables_html.add_attribute("style", r#"list-style-type: "↪ ""#);
    let mut errors = Vec::new();
    let checked_project = sloe::syntax_project_check(
        &mut errors,
        &syntax_project,
        &expressions,
        &patterns,
        &types,
    );
    let compiled_project =
        sloe::checked_project_to_js(&checked_project, &expressions, &patterns, &types);
    for (project_fn_name, _project_fn) in
        checked_project
            .checked_project_fns
            .iter()
            .filter(|(fn_name, project_fn)| {
                !sloe::core_fns.contains_key(*fn_name)
                    && match &project_fn.parameter_type {
                        Some(sloe::Type::Record(fields)) => fields.is_empty(),
                        Some(sloe::Type::CoreConstruct { name, arguments: _ }) => name == "Origin",
                        _ => false,
                    }
            })
    {
        // would be faster if we only use one return for all results.
        // If performance is bad, do that instead
        let to_evaluate = format!(
            "{}\n\nreturn {}({{}});",
            compiled_project.replace("export ", ""),
            sloe::name_to_lowercase_js(project_fn_name),
        );
        let mut evaluated_variable_html = yew::virtual_dom::VTag::new("li");
        evaluated_variable_html.add_child(html_element(
            "code",
            [],
            [html_text_dynamic(project_fn_name)],
        ));
        evaluated_variable_html.add_child(html_text(" is "));
        let function_to_evaluate = web_sys::js_sys::Function::new_no_args(&to_evaluate);
        let evaluated = function_to_evaluate.call(&web_sys::wasm_bindgen::JsValue::NULL, ());
        match evaluated {
            Ok(evaluated) => {
                let mut result_as_sloe = String::new();
                sloe_value_as_js_value_print(&mut result_as_sloe, &evaluated);
                evaluated_variable_html.add_child(html_element(
                    "code",
                    [],
                    [html_text_dynamic(result_as_sloe)],
                ));
            }
            Err(error) => evaluated_variable_html.add_child(html_text_dynamic(
                match web_sys::wasm_bindgen::JsCast::dyn_ref::<web_sys::js_sys::Error>(&error) {
                    Some(error) => format!("error: {:?}", error.message()),
                    None => format!("error: {:?}", error),
                },
            )),
        }
        evaluated_variables_html.add_child(evaluated_variable_html.into());
    }
    full.add_child(evaluated_variables_html.into());
    let mut errors_html = yew::virtual_dom::VTag::new("ul");
    errors_html.add_attribute("style", r#"list-style-type: "⚠︎ ""#);
    for error in errors {
        errors_html.add_child(html_element(
            "li",
            [],
            [html_text_dynamic(format!(
                "line {} char {}: {}",
                error.range.start.line, error.range.start.character, error.message
            ))],
        ));
    }
    full.add_child(errors_html.into());

    full.add_child(html_element(
        "p",
        [("style", "font: inherit; white-space: pre-line;".into())],
        [
            html_text("💡 "),
            html_text(example_explainer(selected_example)),
        ],
    ));
    full.into()
}
fn sloe_value_as_js_value_print(formatted: &mut String, js_value: &web_sys::wasm_bindgen::JsValue) {
    use std::fmt::Write as _;
    if let Some(number) = js_value.as_f64() {
        let _ = write!(formatted, "{}", number);
    } else if let Some(str) = js_value.as_string() {
        let _ = write!(formatted, "{:?}", str);
    } else if js_value.is_function() {
        formatted.push_str("[function]");
    } else if js_value.is_undefined() {
        formatted.push('.');
    } else if js_value.is_array() {
        formatted.push('(');
        let array = web_sys::js_sys::Array::from(js_value);
        let mut elements = array.iter();
        if let Some(element0) = elements.next() {
            if element0.is_string() {
                formatted.push_str("chars in ");
                let _ = write!(formatted, "{:?}", array.join(""));
            } else {
                formatted.push_str("; ");
                sloe_value_as_js_value_print(formatted, &element0);
                for element in elements {
                    formatted.push_str(" ; ");
                    sloe_value_as_js_value_print(formatted, &element);
                }
            }
        } else {
            formatted.push_str("Buf-empty");
        }
        formatted.push(')');
    } else if let Some(object) = web_sys::wasm_bindgen::JsCast::dyn_ref(js_value)
        && let Ok(entries) =
            web_sys::js_sys::Object::entries_typed::<web_sys::wasm_bindgen::JsValue>(object)
        && let mut entries_iterator = entries.iter()
        && let Some(entry0) = entries_iterator.next()
    {
        formatted.push('(');
        formatted.push(if entries.length() == 1 { '|' } else { '.' });
        formatted.push_str(&ToString::to_string(&entry0.get0()).replace("_", "-"));
        formatted.push(' ');
        sloe_value_as_js_value_print(formatted, &entry0.get1());
        for entry in entries_iterator {
            formatted.push(' ');
            formatted.push(if entries.length() == 1 { '|' } else { '.' });
            formatted.push_str(&ToString::to_string(&entry.get0()).replace("_", "-"));
            formatted.push(' ');
            sloe_value_as_js_value_print(formatted, &entry.get1());
        }
        formatted.push(')');
    } else {
        let _ = write!(formatted, "{:?}", js_value);
    }
}
fn sloe_core_declarations_html() -> yew::Html {
    let mut section = yew::virtual_dom::VTag::new("section");
    section.add_child(sub_heading_html("core declarations"));
    let mut type_aliases_sorted = sloe::core_type_aliases.iter().collect::<Vec<_>>();
    type_aliases_sorted.sort_unstable_by_key(|(name, _)| *name);
    section.add_children(type_aliases_sorted.into_iter().map(
        |(core_choice_type_name, core_choice_type_info)| {
            sloe_type_alias_to_html(core_choice_type_name, core_choice_type_info)
        },
    ));
    let mut project_fns_sorted = sloe::core_fns.iter().collect::<Vec<(&sloe::Name, _)>>();
    project_fns_sorted.sort_unstable_by_key(|(name, _)| *name);
    section.add_children(project_fns_sorted.into_iter().map(
        |(core_variable_name, core_variable_info)| {
            sloe_project_fn_to_html(core_variable_name, core_variable_info)
        },
    ));
    section.into()
}
fn sloe_project_source_to_html(project_source: &str) -> yew::Html {
    sloe::core::origin_new!(expressions_origin, Expressions);
    sloe::core::origin_new!(patterns_origin, Patterns);
    sloe::core::origin_new!(types_origin, Types);
    let mut expressions = sloe::core::Buf::new(expressions_origin);
    let mut patterns = sloe::core::Buf::new(patterns_origin);
    let mut types = sloe::core::Buf::new(types_origin);
    let syntax_project =
        sloe::parse_project(&mut expressions, &mut patterns, &mut types, project_source);
    let mut highlights = sloe::HighlightState {
        tokens: Vec::with_capacity(project_source.len() / 2),
        previous_token_start: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    sloe::project_highlight(
        &mut highlights,
        &syntax_project,
        &expressions,
        &patterns,
        &types,
    );
    highlighted_sloe_source_to_html(project_source, &mut highlights.tokens.into_iter())
}
fn sloe_type_alias_to_html(name: &sloe::Name, type_alias: &sloe::CheckedTypeAlias) -> yew::Html {
    let mut section_html = yew::virtual_dom::VTag::new("section");
    section_html.add_child(documentation_heading_html(name));
    let mut formatted = String::new();
    sloe::checked_type_alias_format(&mut formatted, name, type_alias);
    section_html.add_child(sloe_project_source_to_html(&formatted));
    if let Some(documentation) = &type_alias.documentation {
        section_html.add_child(sloe_documentation_markdown_to_html(documentation));
    }
    section_html.into()
}
fn sloe_project_fn_to_html(name: &sloe::Name, project_fn: &sloe::CheckedProjectFn) -> yew::Html {
    let mut section_html = yew::virtual_dom::VTag::new("section");
    section_html.add_child(documentation_heading_html(name));
    let mut formatted = String::new();
    sloe::checked_project_fn_format(&mut formatted, name, project_fn);
    sloe::core::origin_new!(types_origin, Types);
    let mut types = sloe::core::Buf::new(types_origin);
    let mut parse_state = sloe::ParseState {
        source: &formatted,
        offset_utf8: 0,
        position: lsp_types::Position {
            line: 0,
            character: 0,
        },
    };
    if let Some(project_fn_signature) =
        sloe::parse_project_fn_signature(&mut parse_state, &mut types)
    {
        let mut highlights = sloe::HighlightState {
            tokens: Vec::with_capacity(formatted.len() / 2),
            previous_token_start: lsp_types::Position {
                line: 0,
                character: 0,
            },
        };
        sloe::project_fn_signature_highlight(&mut highlights, &types, &project_fn_signature);
        section_html.add_child(highlighted_sloe_source_to_html(
            &formatted,
            highlights.tokens.into_iter(),
        ));
    }
    if let Some(documentation) = &project_fn.documentation {
        section_html.add_child(sloe_documentation_markdown_to_html(documentation));
    }
    section_html.into()
}
fn documentation_heading_html(name: &str) -> yew::Html {
    html_element("h4", [], [html_link_to_self(name)])
}
fn html_link_to_self(name: &str) -> yew::Html {
    let id = name.replace(" ", "-");
    html_element(
        "a",
        [("href", format!("#{id}").into()), ("id", id.into())],
        [html_text_dynamic(format!("#{name}"))],
    )
}
fn sub_heading_html(name: &str) -> yew::Html {
    html_element("h3", [], [html_link_to_self(name)])
}
fn highlighted_sloe_source_to_html(
    source: &str,
    mut highlights: impl Iterator<Item = lsp_types::SemanticToken>,
) -> yew::Html {
    let mut html = yew::virtual_dom::VTag::new("pre");
    html.add_attribute("style", r#"line-height: inherit; font-size: medium; font-family: "Liga NovaMono", monospace, sans-serif; margin-top: 0.5em"#);

    let mut previous_token_start = lsp_types::Position {
        line: 0,
        character: 0,
    };
    // can be optimized
    let mut maybe_next_highlight = highlights.next();
    for (source_line_index, source_line) in source.lines().enumerate() {
        let mut current_offset_in_line: usize = 0;
        while let Some(highlight) = maybe_next_highlight
            && let highlight_start = (lsp_types::Position {
                line: previous_token_start.line + highlight.delta_line,
                character: if highlight.delta_line == 0 {
                    previous_token_start.character + highlight.delta_start
                } else {
                    highlight.delta_start
                },
            })
            && source_line_index as u32 == highlight_start.line
            && let highlight_range = (lsp_types::Range {
                start: highlight_start,
                end: lsp_types::Position {
                    line: highlight_start.line,
                    character: highlight_start.character + highlight.length,
                },
            })
        {
            let highlight_start_offset_in_line =
                utf16_offset_to_utf8_in(source_line, highlight_range.start.character as usize);
            let highlight_end_offset_in_line =
                utf16_offset_to_utf8_in(source_line, highlight_range.end.character as usize);

            html.add_child(html_element(
                "code",
                [],
                [html_text_dynamic(
                    &source_line[current_offset_in_line..highlight_start_offset_in_line],
                )],
            ));
            html.add_child(html_element(
                "code",
                [(
                    "style",
                    format!(
                        "color: {}",
                        sloe_syntax_highlight_kind_to_css_color(
                            &sloe::token_types[highlight.token_type as usize]
                        )
                    )
                    .into(),
                )],
                [html_text_dynamic(
                    &source_line[highlight_start_offset_in_line..highlight_end_offset_in_line],
                )],
            ));

            current_offset_in_line = highlight_end_offset_in_line;
            previous_token_start = highlight_start;
            maybe_next_highlight = highlights.next();
        }
        html.add_child(html_element(
            "code",
            [],
            [html_text_dynamic(&source_line[current_offset_in_line..])],
        ));
        html.add_child(html_element("code", [], [html_text("\n")]));
    }
    html.into()
}
fn sloe_syntax_highlight_kind_to_css_color(kind: &lsp_types::SemanticTokenTypes) -> &'static str {
    match kind {
        lsp_types::SemanticTokenTypes::Type => "rgb(0,255,255)",
        lsp_types::SemanticTokenTypes::TypeParameter
        | lsp_types::SemanticTokenTypes::Variable
        | lsp_types::SemanticTokenTypes::Parameter => "rgb(130,140,255)",
        lsp_types::SemanticTokenTypes::EnumMember => "rgb(120,235,30)",
        lsp_types::SemanticTokenTypes::Property => "rgb(255, 145, 0)",
        lsp_types::SemanticTokenTypes::Function => "rgb(255, 225, 140)",
        lsp_types::SemanticTokenTypes::Comment => "rgb(140,140,140)",
        lsp_types::SemanticTokenTypes::String | lsp_types::SemanticTokenTypes::Number => {
            "rgb(225,105,240)"
        }
        lsp_types::SemanticTokenTypes::Keyword => "rgb(255,60,100)",
        _ => "white",
    }
}
fn sloe_documentation_markdown_to_html(sloe_documentation_markdown: &str) -> yew::Html {
    let mut html = yew::virtual_dom::VTag::new("p");
    let mut maybe_current_code_block_start_line_index: Option<usize> = None;
    for (sloe_documentation_markdown_line_index, sloe_documentation_markdown_line) in
        sloe_documentation_markdown.lines().enumerate()
    {
        match sloe_documentation_markdown_line {
            "```" | "```sloe" => match maybe_current_code_block_start_line_index {
                None => {
                    maybe_current_code_block_start_line_index =
                        Some(sloe_documentation_markdown_line_index);
                }
                Some(current_code_block_start_line_index) => {
                    maybe_current_code_block_start_line_index = None;
                    html.add_child(sloe_project_source_to_html(
                        &sloe_documentation_markdown
                            .lines()
                            .skip(current_code_block_start_line_index + 1)
                            .take(
                                sloe_documentation_markdown_line_index
                                    - current_code_block_start_line_index
                                    - 1,
                            )
                            .collect::<Vec<&str>>()
                            .join("\n"),
                    ));
                }
            },
            "" => {
                if maybe_current_code_block_start_line_index.is_none() {
                    html.add_child(linebreak_html());
                }
            }
            _ => {
                if maybe_current_code_block_start_line_index.is_none() {
                    // insert space before because otherwise if the previous line ends in
                    // punctuation like , the text in the next line would be attached directly after it
                    html.add_child(html_text(" "));
                    html.add_child(html_text_dynamic(sloe_documentation_markdown_line));
                }
            }
        }
    }
    html.into()
}

#[derive(Copy, Clone)]
struct ExampleInfo {
    source: &'static str,
    explainer: &'static str,
    name: &'static str,
}
fn example_source(example: Example) -> &'static str {
    example_info(example).source
}
fn example_explainer(example: Example) -> &'static str {
    example_info(example).explainer
}
const fn example_name(example: Example) -> &'static str {
    example_info(example).name
}
#[derive(Copy, Clone)]
enum Example {
    HelloWorld,
    Variable,
    Numbers,
    Text,
    FunctionCall,
    Types,
    Function,
    Record,
    Choice,
    Match,
    Vec,
    Comment,
    Extras,
}
static example_infos: [(Example, ExampleInfo); 13] = {
    const fn entry(example: Example) -> (Example, ExampleInfo) {
        (example, example_info(example))
    }
    [
        entry(Example::HelloWorld),
        entry(Example::Variable),
        entry(Example::Numbers),
        entry(Example::Text),
        entry(Example::FunctionCall),
        entry(Example::Types),
        entry(Example::Function),
        entry(Example::Match),
        entry(Example::Record),
        entry(Example::Choice),
        entry(Example::Vec),
        entry(Example::Comment),
        entry(Example::Extras),
    ]
};
const fn example_info(example: Example) -> ExampleInfo {
    // TODO convert all of them and add new ones
    match example {
        Example::HelloWorld => ExampleInfo {
            name:"hello world",
            source: r#"
fn Hi
    origin Origin _origin
    : .buf Buf _origin, char .span Span _origin =
    Greet .name "world" .buf Buf-empty{char} origin

fn Greet
    .name name str .buf buf Buf _origin, char
    : .buf Buf _origin, char .span Span _origin =
    ? .buf buf .span |{Opt Span _origin}no . [string]
    ? Buf-char-opt-span-add-str .. string .new "Hello, " [string]
    ? Buf-char-span-add-str .. string .new name [string]
    Buf-char-span-add-str .. string .new "!\n"
"#,
            explainer: "What a mouthful!
We declare a Greet function which takes a name string and a buffer to add the message to.
We then append the name along with other strings to buffer to form a message span in the buffer.
For more details, click through the examples above and try changing things.",
        },
        Example::Variable => ExampleInfo {
            name: "declare a function",
            source: r#"
fn Your-project-function-name . : str =
    "Yahallo there, cutie"
"#,
            explainer: "Add a new function to your project by choosing an uppercase name consisting of a-z, A-Z, 0-9 or - after `fn` at the start of a line,
followed by its typed parameter pattern, a :, the result type, an =, the resulting expression (for example \":)\").
sloe also has \"core\" functions like I32-add-clamp that any project can reference.
To see the full list, scroll down or search the site for #some-name-to-search-for.",
        },
        Example::Numbers => ExampleInfo {
            name: "numbers",
            source: r#"
fn Number-with-a-decimal-point . : f32 = -2.7 f32
fn F32-not-ending-in-decimal-point . : f32 = 4 f32

fn Signed-integer . : i32 = -2 i32
fn Signed-integer-zero . : i32 = 0 i32

fn Unsigned-integer . : u32 = 2 u32
fn Unsigned-integer-zero . : u32 = 0 u32

fn Positive-integer . : p32 = 2 p32
"#,
            explainer: "some basic types of numbers:
- f32: floating point number; can have a decimal point and can have a sign
- i32: integer; whole number; can have a sign but no decimal point
- u32: unsigned integer: number without a sign or decimal point.
  Used mainly for indexes, counting and similar
- p32: positive integer: non-zero number without a sign, decimal point.
  Used mainly for lengths, counts and similar",
        },
        Example::Text => ExampleInfo {
            name: "text",
            source: r#"
fn Single-character . : char = 'a'
fn Escaped-quote . : char = '\''
fn Escaped-backslash . : char = '\\'
fn Escaped-tab . : char = '\t'
fn Escaped-linebreak . : char = '\n'
fn Escaped-carriage-return . : char = '\r'
fn By-code-point-hex . : char = '\u{1F648}'

fn Cat . : str = "₍^. .^₎⟆"
fn Escaped-double-quote . : str = "\"hello\""
fn Strings-have-at-least-1-char . : .start char .after Opt str =
    Str-start "Hello"
"#,
            explainer: "single characters (of type char) are wrapped in '...', non-empty strings (of type str) are wrapped in \"...\".
String builders and string slices use entirely different types, str refers to static memory",
        },
        Example::FunctionCall => ExampleInfo {
            name: "function call",
            source: r#"
fn Regular-call . : f32 =
    F32-add-clamp .a 1.2 f32 .b 2.3 f32

fn Nested-call . : f32 =
    F32-mul-clamp F32-dup 7 f32
"#,
            explainer: "Function names are followed by whitespace, then an argument (In other languages this is often done with: function(arg)).
As a result, the argument could itself be another function. Here, Nested-call first duplicates 7 into .a 7 .b 7, then multiplies this result, resulting in 49",
        },
        Example::Types => ExampleInfo {
            name: "declare a type",
            source: r#"
ty string-alias str
ty Stringify _from
    Fn _from, str
ty Pair-alias _a, _b
    .a _a .b _b

fn Using-the-alias arguments Pair-alias i32, i32 : i32 =
    I32-mul-clamp arguments
"#,
            explainer: "Any expression can have its explicit type in front which is checked to match.
This can make it easier to know what for example a long chain of operations returns.
More importantly though, some syntax requires explicit types, like function parameters or empty vectors.
Variable types are represented as _ followed by a lowercase name.
They represent a type that is filled in when constructed; for example `Stringify u32` fills in the `_from` variable with `u32`.

You can also create short name aliases for types you often use using type, type name, space-separated variables, result type.
These will become especially useful in the example for records
",
        },
        Example::Function => ExampleInfo {
            name: "function",
            source: r#"
fn I32-subtract .from from i32 .minus minus i32 : i32 =
    I32-add-clamp .a from .b I32-negate-clamp minus

fn Use-int-subtract . : i32 =
    I32-subtract .from 3 i32 .minus 4 i32

fn Same-in-same-out anything _in : _in =
    anything

fn Use-same-in-same-out . : str =
    Same-in-same-out "oo ee oo"

fn Function-returning-a-function . : Fn .a i32 .b i32, i32 =
    [.a a i32 .b b i32] I32-add-clamp .a a .b b

fn Call-function-value . : i32 =
    # you can't just call a value, use the "Call" core function
    Call
    .fn Function-returning-a-function .
    .in .a 63 i32 .b 6 i32

fn Buf-with-capacity{_element}
    .origin origin Origin _o
    .length length u32
    : Buf _o, _element =
    Buf-pre-allocate-at-least
    .length length
    .buf Buf-empty{_element} origin
"#,
            explainer: "As seen in the last example, function declarations can also have type parameters if only the result type uses them.
Each is specified in braces {_}. When calling, each type argument is aso wrapped in braces {}.

Project fns are not values themselves; they must always be followed by an argument.
To create a function value (of type Fn), put a typed pattern in brackets [] followed by its result.
The simplest pattern is a variable name followed by its type.
See the query example for other kinds of patterns.
These local functions can NOT reference local variables introduced outside of the parameters (closures),
This may seem surprising! But it's actually pretty nice to know what kind of inputs (and outputs) a function has!
Functions in sloe:
- always take up very little space and dealing with them never interacts with the heap
- can be trivially duplicated and scrapped
- are easy and safe to pass to other threads and languages
To actually pass in variables from the surounding context, accept it as a parameter to that function"
        },
        Example::Match => ExampleInfo {
            name: "query",
            source: r#"
fn With-intermediate-local-variable . : f32 =
    ? F32-mul-clamp .a 3.14 f32 .b 2.07 f32 [intermediate]
    F32-mul-clamp F32-dup intermediate

fn Opt-p32-to-u32 maybe Opt p32 : u32 =
    ? maybe
    [|no .] 0 u32
    [|yes p32] P32-to-u32 p32
"#,
            explainer: "To decide what to do based on the shape of some value, put a ? before the value, then one or more cases.
A case consists of an untyped pattern in brackets [] followed by the result.

In the simplest case, you can assign a shorter name to some value with `? value [name]`.
But you can also decompose it into its fields or even match on its variants in multiple cases.
A query is checked for missing cases so you don't forget some shape.
Works for \"record\" and \"choice\" types whose examples show how to pattern match them.
To avoid increasing levels of indentation, you can keep the last case result unindented
(In other languages, this usually done with an early return, elseif or let else)."
        },
        Example::Record => ExampleInfo {
            name: "record",
            source: r#"
ty empty-record
    .

fn Multiple-shapes-of-data-bundled-together
    . : .weight f32 .color color .position position =
    .weight 1.0 f32
    .color (.r 255 u32 .g 100 u32 .b 40 u32)
    .position (.x 0.0 f32 .y 0.0 f32)

ty color .r u32 .g u32 .b u32
ty position .x f32 .y f32

fn Default-config . : config =
    .line-separator "\r\n"
    .element-separator ";"
    .version 2 p32

ty config
    .line-separator str
    .element-separator str
    .version p32

ty vector .x f32 .y f32

fn Example-vector . : vector =
    .x 2.0 f32 .y 3.0 f32

fn Vector-length vector vector : Opt f32 =
    ? vector [.x x .y y]
    F32-pow
    .exponent 0.5 f32
    .base
    F32-add-clamp
    .a F32-mul-clamp F32-dup x
    .b F32-mul-clamp F32-dup y

fn Use-vector-length . : Opt f32 =
    Vector-length Example-vector .
"#,
            explainer: "Passing some infos which are connected to each other as separate arguments is inconvenient and error-prone.
A \"record\" gives the individual values a field name and combines them into one value
(other languages usually call this struct(ure) or data object).
When you end up passing this record a bunch, it's probably a good idea to make a type alias for it.

Remember this weird . that you've seen for functions without proper inputs?
It's just an empty record! You may be used to seeing this as `()` or `void` in other languages",
        },
        Example::Choice => ExampleInfo {
            name: "choice",
            source: r#"
ty bool
    |true .
    |false .

fn Bool-order .a a bool .b b bool : order =
    ? .a a .b b
    [.a |false . .b |true .] |{order}less .
    [.a |true . .b |true .] |{order}equal .
    [.a |false . .b |false .] |{order}equal .
    [.a |true . .b |false .] |{order}greater .

ty Type-syntax _types
    |variable str
    |construct .name str .arguments Span _types
    |tuple Span _types
    |function .inputs Span _types .output Slot _types

fn Type-rid
    .type type Type-syntax _types
    .buf buf Buf _types, Type-syntax _types
    : Buf _types, Type-syntax _types =
    ? type
    [|variable variable] (
        ? Str-rid variable [.]
        buf
        )
    [|tuple parts] Type-span-rid .buf buf .span parts
    [|construct .name name .arguments arguments] (
        ? Str-rid name [.]
        Type-span-rid .buf buf .span arguments
        )
    [|function .inputs inputs .output output] (
        ? Buf-remove .buf buf .slot output [.buf buf .element output]
        ? Type-rid .buf buf .type output [buf]
        Type-span-rid .buf buf .span inputs
        )

fn Type-span-rid
    .span span Span _types
    .buf buf Buf _types, Type-syntax _types
    : Buf _types, Type-syntax _types =
    Span-fold
    .direction |{|up . |down .}up .
    .span span
    .state buf
    .step
    [.state buf Buf _types, Type-syntax _types .slot slot Slot _types]
    ? Buf-remove .buf buf .slot slot [.buf buf .element type]
    Type-rid .buf buf .type type
"#,
            explainer: "Some info can come in multiple shapes (variants).
For example there could be an error or a value, nothing or something, different state per page etc.
To construct a variant, put a bar |, then an explicit type in braces {} then its value.
Each variant has a value! If you have nothing to attach to a variant, just use the empty record `.`.
In other languages, this is typically done with object hierarchies or a kind enum + union of value types.
The most common choice type in sloe is `Opt _`, the optional type which is `|yes _ |no .`.
In this example we also met `order` and `|up . |down .`.

To learn about empty choice types, go to `Choice-empty-to`",
        },
        Example::Vec => ExampleInfo {
            name: "Buf & array",
            source: r#"
fn Number-buffer-sum . : u32 =
    ^example-origin
    ? Buf-empty{u32} example-origin [buf]
    ? Buf-add .buf buf .new 1234 u32 [.buf buf .slot slot]
    ? Slot-to-span slot [span]
    ? Buf-span-add-array .buf buf .span span .new Example-array .
    [.buf buf .span span]
    ? U32s-sum .buf buf .span span [.buf buf .sum sum]
    ? Buf-rid buf [.]
    sum

fn Example-array . : Array u32, .el0 u32 .el1 u32 .el2 u32 =
    # arrays are not supposed to be stored, just an example
    ; 3 u32 ; 2 u32 ; 1 u32

fn U32s-sum
    .span span Span _origin
    .buf buf Buf _origin, u32
    :
    .sum u32
    .buf Buf _origin, u32
    =
    Span-fold
    .direction |{|up . |down .}up .
    .span span
    .state (.sum 0 u32 .buf buf)
    .step
    [
    .state (.sum so-far u32 .buf buf Buf _origin, u32)
    .slot slot Slot _origin
    ]
    ? Buf-remove .buf buf .slot slot [.buf buf .element element]
    .buf buf .sum U32-add-clamp .a so-far .b element
"#,
            explainer: "an Array holds an exact amount of elements of the same type.
This is super convenient, as we can store it on the stack and pass it around freely,
never having to think about hoow to free it. Create one by prefixing all elements with ;.

A Buf holds a variable amount of elements of the same element type.
To get its full power we need to store its elements on the heap.
To refer to slices or specific elements inside the heap, one might think
a simple pointer is enough but...
If the Buf has fully occupied its allotted space on the heap and a new element gets added,
the Buf must move its elements to a larger space, invalidating your pointers
To combat this, you could move all slices and specific elements to each their own little space on the heap.
This works well! But it can be slower as related memory is more fragmented and thus harder to find by the CPU.
So, indexes and ranges? Yes! If we address the problems
- we don't know which Buf to search in. Indexes are easy to confuse, leading to bugs
- because we cannot guarantee you've chosen the right Buf,
  we need to bounds-check on every access etc. to be memory safe.
  The cost is small but it's a bit sad
The solution is qzite simple: Attach a unique type to each Buf
and the indexes&ranges it gives out.
- `Buf _origin, _element`: the resizable array on the heap, marked with _origin
- `Slot _origin` an index into the `Buf` with the same origin
- `Span _origin` a range (start index + length) into the `Buf` with the same origin
Unique new origins can be created with `^new-origin`;
the expression after that will be able to make use of it (and never thereafter).
Of course origins can not be duplicated. That's the whole trick!

And you may have noticed already...
Every value in sloe can only be used exactly once.
If you want more of it, call the `-dup` helpers like `U32-dup` when they exist.
If you want less of it, call the `-rid` helpers like `Str-rid` when they exist.
Just like that I can enforce that every `Buf` gets created with `Buf-empty`
and propertly cleaned up with `Buf-rid` (among other cleanup functions).

Now go snoop around in the `Buf` operation docs.
If you're wondering how to store `Buf`s persistently, check `Origin-erase`",
        },
        Example::Comment => ExampleInfo {
            name: "comment",
            source: r#"
#shegang

fn Documented-after-the-type . : .
    # fn documentation.
    # All comments can span
    # multiple lines.
    =
    .

ty string str
    # type documentation

fn Nice . : . =
    # in front of any expression
    .
"#,
            explainer: "Comments can be put in front of expressions or at the file level.
Documentation comments can be put after the type of a ty or fn declaration.",
        },
        Example::Extras => ExampleInfo {
            name: "extras",
            source: r#"
fn Make-it-3d xy .x f32 .y f32 : .x f32 .y f32 .z f32 =
    .. xy .z 0 f32
"#,
            explainer: "Sloe has special syntax sugar for combining records and fields (\"spreading\" its fields into the record).
Between, before or after the existing fields and spreads, add .. followed by a variable or any other expression.

This feature is not strictly necessary but it can make builders that carry e.g. a buf and span at every step a bit nicer.",
        }
    }
}

// //

fn html_text(content: &'static str) -> yew::Html {
    yew::Html::VText(yew::virtual_dom::VText {
        text: yew::AttrValue::Static(content),
    })
}
fn html_text_dynamic(content: impl ToString) -> yew::Html {
    yew::Html::VText(yew::virtual_dom::VText::from(content))
}
fn html_link_to(resource: &str, name: &'static str) -> yew::Html {
    html_element("a", [("href", resource.into())], [html_text(name)])
}
fn linebreak_html() -> yew::Html {
    html_element("br", [], [])
}
fn html_element(
    tag: &'static str,
    modifiers: impl IntoIterator<Item = (&'static str, yew::AttrValue)>,
    subs: impl IntoIterator<Item = yew::Html>,
) -> yew::Html {
    let mut yew_element = yew::virtual_dom::VTag::new(tag);
    for (modifier_key, modifier_value) in modifiers {
        yew_element.add_attribute(modifier_key, modifier_value);
    }
    yew_element.add_children(subs);
    yew::Html::VTag(std::rc::Rc::new(yew_element))
}
fn yew_listener(
    kind: yew::virtual_dom::ListenerKind,
    handle: impl Fn(web_sys::Event),
) -> impl yew::virtual_dom::Listener {
    YewGenericListener { kind, handle }
}
struct YewGenericListener<Handle> {
    kind: yew::virtual_dom::ListenerKind,
    handle: Handle,
}
impl<Handle: Fn(web_sys::Event)> yew::virtual_dom::Listener for YewGenericListener<Handle> {
    fn kind(&self) -> yew::virtual_dom::ListenerKind {
        self.kind.clone()
    }
    fn handle(&self, event: web_sys::Event) {
        (self.handle)(event)
    }
    fn passive(&self) -> bool {
        true
    }
}

// //

fn utf16_offset_to_utf8_in(source: &str, utf16_offset: usize) -> usize {
    let mut utf8_length: usize = 0;
    let mut so_far_length_utf16: usize = 0;
    'traversing_utf16_length: for char in source.chars() {
        if so_far_length_utf16 >= utf16_offset {
            break 'traversing_utf16_length;
        }
        utf8_length += char.len_utf8();
        so_far_length_utf16 += char.len_utf16();
    }
    utf8_length
    // below does not work for string containing 2-part UTF-16 characters
    // source
    //     .encode_utf16()
    //     .take(utf16_offset)
    //     .map(|utf16_char| {
    //         char::decode_utf16([utf16_char])
    //             .map(|r| r.map(char::len_utf8).unwrap_or(0))
    //             .sum::<usize>()
    //     })
    //     .sum()
}
