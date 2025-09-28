use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use bevy::prelude::*;
use leptos_bevy_canvas::prelude::*;

/// -------- Leptos Shell --------
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/bevytos.css" />
        <Title text="Welcome to Leptos" />

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("canvas") view=CanvasPage />
                </Routes>
            </main>
        </Router>
    }
}

/// -------- Leptos Home --------
#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <p>
            <a href="/canvas">"Go to Bevy Canvas"</a>
        </p>
    }
}

/// -------- Bevy Event --------
#[derive(Event)]
pub struct TextEvent {
    pub text: String,
}

#[component]
fn CanvasPage() -> impl IntoView {
    // 🚀 1. Event bridge between Leptos and Bevy
    let (text_event_sender, bevy_text_receiver) = event_l2b::<TextEvent>();

    // 🚀 2. Leptos -> Bevy: send input events
    let on_input = move |evt| {
        text_event_sender
            .send(TextEvent {
                text: event_target_value(&evt),
            })
            .ok();
    };

    // 🚀 3. Render input + BevyCanvas
    view! {
        <h2>"Bevy Canvas Integration"</h2>
        <input type="text" on:input=on_input />

        <BevyCanvas init=move || init_bevy_app(bevy_text_receiver) />
    }
}

// -------- Bevy Side --------
pub fn set_text(mut event_reader: EventReader<TextEvent>) {
    for event in event_reader.read() {
        info!("Got text from Leptos: {}", event.text);
    }
}

fn init_bevy_app(receiver: BevyEventReceiver<TextEvent>) -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy_canvas".into()),
            resolution: (400., 300.).into(),
            ..default()
        }),
        ..default()
    }))
    .import_event_from_leptos(receiver)
    .add_systems(Update, set_text);

    app
}
