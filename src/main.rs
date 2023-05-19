use leptos::{component, create_signal, view, IntoView, Scope, SignalGet, SignalUpdate};

fn main() {
    leptos::mount_to_body(
        |cx| view! (cx, <div> <h1>"Hello, Leptose!"</h1> </div><div><p>"I can write a static website here"</p></div><div><App/></div><div><Test/></div>),
    );
}

#[component]
fn Test(cx: Scope) -> impl IntoView {
    let (word, set_word) = create_signal(cx, "Hello");

    view! { cx,
        <button
            on:click=move |_| {
                set_word.update(|n| {
                if *n == "Hello" {
                    *n = "World"
                } else {
                    *n = "Hello"
                }
            });
        }
        >
            "Click me: "
            // also word() works here
            {move || word.get()}
        </button>
        <h1>"I am a big boy " { move || word.get()}" !!!"</h1>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
        <button
            on:click=move |_| {
                set_count.update(|n| *n -= 1);
            }
        >
            "Don't click me: "
            {move || count.get()}
        </button>
        <h1
            class:red=move || count.get() == 5
            >
            "I am a big boy " { move || count.get()}" !!!"</h1>
    }
}
