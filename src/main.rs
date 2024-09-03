use leptos::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Increment"
        </button>

        <button
            on:click=move |_| {
                set_count.update(|n| if *n > 0 { *n -= 1 });
            }
        >
            "Decrement"
        </button>

        <p 
            // class:red=move || count() % 2 == 1
            class=("red", move || count() % 2 == 1)
        >
            {count}
        </p>

        <p>
            {double_count}
        </p>

        <progress max="50" value=double_count />
    }
}

#[component]
fn App1() -> impl IntoView {
    let (x, set_x) = create_signal(0);
    view! {
        <button
            on:click={move |_| {
                set_x.update(|n| *n += 10);
            }}
        // set the `style` attribute
        style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", x() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", x)
            >
            "Click to Move"
            </button>
    }
}
