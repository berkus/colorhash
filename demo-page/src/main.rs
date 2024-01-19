use {colorhash::ColorHash, leptos::*, once_cell::sync::Lazy, rand::Rng};

static COLOR_HASH: Lazy<ColorHash> = Lazy::new(|| ColorHash::new());

pub fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    mount_to_body(|| {
        view! { <App/> }
    });
}

#[component]
pub fn App() -> impl IntoView {
    let history = create_signal(vec![]);
    view! {
        <h1>Color Hash</h1>
        <p>Generate color based on the given string (using HSL color space and SHA256).</p>
        <Input items={history}/>
        <HistoryList history={history}/>
    }
}

fn new_item_id() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[component]
fn Input(items: (ReadSignal<Vec<ColorItem>>, WriteSignal<Vec<ColorItem>>)) -> impl IntoView {
    let (_, set_new_item) = items;
    let (default_value, set_default_value) = create_signal("".to_string());

    view! {
        <input
            type="text"
            autofocus=true
            placeholder="Type Here"
            on:keyup=move |event| {
                if !event_target_value(&event).is_empty() {
                    let input_value = event_target_value(&event);
                    let new_item = ColorItem {
                        key: new_item_id(),
                        str: input_value.clone(),
                    };
                    set_new_item
                        .update(|items| {
                            items.splice(..0, [new_item]);
                        });
                    set_default_value.set(input_value);
                }
            }
            prop:value=default_value
        />
    }
}

#[component]
fn HistoryList(
    history: (ReadSignal<Vec<ColorItem>>, WriteSignal<Vec<ColorItem>>),
) -> impl IntoView {
    let (list_state, _) = history;
    let my_history = move || {
        list_state
            .get()
            .iter()
            .map(|item| (item.key, item.str.clone()))
            .collect::<Vec<_>>()
    };

    view! {
        <ul id="history">
            <For
                each=my_history
                key=|item| item.0
                children=move |item| {
                    view! { <Item str=item.1/> }
                }
            />

        </ul>
    }
}

#[component]
fn Item(str: String) -> impl IntoView {
    let rgb = COLOR_HASH.rgb(&str).to_css_string();
    let rgb2 = rgb.clone();
    let hex = COLOR_HASH.hex(&str);

    view! {
        <li
            style:border-left-color=move || format!("{}", rgb)
            style:color=move || format!("{}", rgb2)
    >
            <span class="hex">{hex.to_uppercase()}</span>
            <span class="text">{str}</span>
        </li>
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ColorItem {
    pub key: u32,
    pub str: String,
}
