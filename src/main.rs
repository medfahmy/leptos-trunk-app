use leptos::*;
use std::marker::PhantomData;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // mount_to_body(|| view! { 
    //     <SizeOf<u8> />
    //     <br />
    //     <SizeOf<usize> />
    //     <br />
    //     <SizeOf<String> />
    // })

    mount_to_body(|| view! {
        <ManyCounters />
    })
}

#[component]
fn ManyCounters() -> impl IntoView {
    let len = 5;
    let counters = (1..=len).map(|n| create_signal(n));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count(count() + 1)>{count}</button>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}

#[component]
fn Collect() -> impl IntoView {
    let words = vec!["Hello", "Beautiful", "World"];

    mount_to_body(|| view! { 
        <p>{words.clone()}</p>
        <ul>
            {words
                .into_iter()
                .map(|word| view! { <li>{word}</li> })
                // .collect::<Vec<_>>()}
                .collect_view()}
        </ul>
    })
}

#[component]
fn AppInto() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1) }>
            "Increment"
        </button>

        <button on:click=move |_| { set_count.update(|n| if *n > 0 { *n -= 1 }) }>
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

        <SimpleProgress progress=count.into() />
        <SimpleProgress progress=Signal::derive(double_count) />
    }
}

#[component]
fn SimpleProgress(progress: Signal<i32>) -> impl IntoView {
    view! {
        <progress max="50" value=progress />
    }
}


#[component]
fn SizeOf<T: Sized>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}

#[component]
fn Progress(
    #[prop(optional)] optional: u16,
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max=max value=progress />
    }
}

#[component]
fn App2() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1) } >
            "Increment"
        </button>

        <button on:click=move |_| { set_count.update(|n| if *n > 0 { *n -= 1 }) } >
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

        <Progress progress=count />
        <Progress progress=Signal::derive(double_count) />
    }
}

#[component]
fn App0() -> impl IntoView {
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

#[component]
fn Raw() -> impl IntoView {
    let html = "<p>This HTML will be injected.</p>";
    view! {
        <div inner_html=html/>
    }
}
