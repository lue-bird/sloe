mod sloe;

#[sauron::wasm_bindgen(start)]
fn start() {
    sloe::origin_new!(mouse_trail_origin, MouseTrailOrigin);
    sauron::Program::mount_to_body(App {
        sloe_state: std::cell::Cell::new(Some(sloe::initial_state(mouse_trail_origin))),
    });
}

struct App<MouseTrailOrigin> {
    sloe_state: std::cell::Cell<Option<sloe::State<MouseTrailOrigin>>>,
}
impl<MouseTrailOrigin: 'static> sauron::Application for App<MouseTrailOrigin> {
    type MSG = sloe::Event;

    fn init(&mut self) -> sauron::Cmd<Self::MSG> {
        sauron::Cmd::none()
    }

    fn update(&mut self, event: Self::MSG) -> sauron::Cmd<Self::MSG> {
        let state = self.sloe_state.take().expect("state is initialized");
        let updated_state = sloe::update(sloe::Record·event·state { event, state });
        self.sloe_state.set(Some(updated_state));
        // uncomment to debug
        // web_sys::console::log_1(
        //     &web_sys::js_sys::JsString::from(format!(
        //         "sloe event {:?} → updated state {:?}",
        //         new_sloe_event, self.sloe_state
        //     )),
        // );
        sauron::Cmd::none()
    }

    fn view(&self) -> sauron::prelude::Node<Self::MSG> {
        sloe::origin_new!(htmls_origin, Htmls);
        let htmls = sloe::Vec::new(htmls_origin);
        sloe::origin_new!(modifiers_origin, Modifiers);
        let modifiers: sloe::Vec<Modifiers, sloe::Modifier<sloe::Event, Chars>> =
            sloe::Vec::new(modifiers_origin);
        sloe::origin_new!(chars_origin, Chars);
        let chars = sloe::Vec::new(chars_origin);
        let state = self.sloe_state.take().expect("state is initialized");
        let sloe_dom = sloe::view(sloe::Record·chars·htmls·modifiers·state {
            htmls,
            modifiers,
            chars,
            state,
        });
        self.sloe_state.set(Some(sloe_dom.state));
        sloe_dom_node_to_sauron(
            &sloe_dom.html,
            &sloe_dom.htmls,
            &sloe_dom.modifiers,
            &sloe_dom.chars,
        )
    }
}

fn sloe_dom_node_to_sauron<Htmls, Modifiers: 'static, Chars: 'static>(
    sloe_dom_node: &sloe::Html<Htmls, Modifiers, Chars>,
    htmls: &sloe::Vec<Htmls, sloe::Html<Htmls, Modifiers, Chars>>,
    modifiers: &sloe::Vec<Modifiers, sloe::Modifier<sloe::Event, Chars>>,
    chars: &sloe::Vec<Chars, char>,
) -> sauron::Node<sloe::Event> {
    match sloe_dom_node {
        sloe::Html::Text_static(text) => {
            sauron::Node::Leaf(sauron::vdom::Leaf::Text(std::borrow::Cow::Borrowed(text)))
        }
        sloe::Html::Text_dynamic(text) => sauron::text(
            chars
                .opt_span_slice(text.as_ref())
                .iter()
                .collect::<String>(),
        ),
        sloe::Html::Element(element) => sauron::vdom::Node::Element(sauron::vdom::Element::new(
            None,
            element.tag,
            modifiers
                .opt_span_slice(element.modifiers.as_ref())
                .iter()
                .map(|modifier| sloe_dom_modifier_to_sauron(modifier, chars)),
            htmls
                .opt_span_slice(element.subs.as_ref())
                .iter()
                .map(|sub| sloe_dom_node_to_sauron(sub, htmls, modifiers, chars)),
            false,
        )),
    }
}
fn sloe_dom_modifier_to_sauron<Chars: 'static>(
    sloe_dom_modifier: &sloe::Modifier<sloe::Event, Chars>,
    chars: &sloe::Vec<Chars, char>,
) -> sauron::Attribute<sloe::Event> {
    match sloe_dom_modifier {
        sloe::Modifier::Attribute_static(attribute) => sauron::Attribute {
            namespace: None,
            name: attribute.key,
            value: vec![sauron::AttributeValue::Simple(sauron::Value::Cow(
                std::borrow::Cow::Borrowed(attribute.value),
            ))],
        },
        sloe::Modifier::Attribute_dynamic(attribute) => sauron::Attribute {
            namespace: None,
            name: attribute.key,
            value: vec![sauron::AttributeValue::Simple(sauron::Value::Cow(
                std::borrow::Cow::Owned(
                    chars
                        .opt_span_slice(attribute.value.as_ref())
                        .iter()
                        .collect::<String>(),
                ),
            ))],
        },
        sloe::Modifier::Style_static(style) => sauron::Attribute {
            namespace: None,
            name: "style",
            value: vec![sauron::AttributeValue::Style(vec![sauron::vdom::Style {
                name: std::borrow::Cow::Borrowed(style.key),
                value: sauron::Value::Cow(std::borrow::Cow::Borrowed(style.value)),
            }])],
        },
        sloe::Modifier::Style_dynamic(style) => sauron::Attribute {
            namespace: None,
            name: "style",
            value: vec![sauron::AttributeValue::Style(vec![sauron::vdom::Style {
                name: std::borrow::Cow::Borrowed(style.key),
                value: sauron::Value::Cow(std::borrow::Cow::Owned(
                    chars
                        .opt_span_slice(style.value.as_ref())
                        .iter()
                        .collect::<String>(),
                )),
            }])],
        },
        sloe::Modifier::Property(property) => sauron::Attribute {
            namespace: None,
            name: property.key,
            value: vec![sauron::AttributeValue::Simple(
                sloe_modifier_property_value_to_sauron(&property.value, chars),
            )],
        },
        &sloe::Modifier::On_mouse_move(listen) => sauron::on_mousemove(move |event| {
            listen(sloe::Record·x·y {
                x: event.x().abs() as u32,
                y: event.y().abs() as u32,
            })
        }),
    }
}
fn sloe_modifier_property_value_to_sauron<Chars>(
    sloe_modifier_property_value: &sloe::Modifier_property_value<Chars>,
    chars: &sloe::Vec<Chars, char>,
) -> sauron::Value {
    match sloe_modifier_property_value {
        sloe::Modifier_property_value::True(()) => sauron::Value::Bool(true),
        sloe::Modifier_property_value::False(()) => sauron::Value::Bool(false),
        sloe::Modifier_property_value::Int(int) => sauron::Value::I32(*int),
        sloe::Modifier_property_value::String(span) => sauron::Value::Cow(std::borrow::Cow::Owned(
            chars
                .opt_span_slice(span.as_ref())
                .iter()
                .collect::<String>(),
        )),
    }
}
