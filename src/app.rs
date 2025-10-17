use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use bevy::prelude::*;
use leptos_bevy_canvas::prelude::*;
use std::rc::Rc;

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

//
// CLIENT-SIDE (wasm32) IMPLEMENTATION
//
#[cfg(target_arch = "wasm32")]
#[component]
fn CanvasPage() -> impl IntoView {
    // 1. Bridge between Leptos and Bevy
    let (text_event_sender, bevy_text_receiver) = event_l2b::<TextEvent>();

    // 2. Input handler
    let on_input = move |evt| {
        text_event_sender
            .send(TextEvent {
                text: event_target_value(&evt),
            })
            .ok();
    };

    // 3. Render input + Bevy canvas (client only)
    view! {
        <h2>"Bevy Canvas Integration"</h2>
        <input type="text" on:input=on_input />
        <BevyCanvas init=move || init_bevy_app(bevy_text_receiver.clone()) />
    }
}

//
// SERVER-SIDE PLACEHOLDER
//
#[cfg(not(target_arch = "wasm32"))]
#[component]
fn CanvasPage() -> impl IntoView {
    view! {
        <p>
            <em>"Bevy Canvas is only available on the client side."</em>
        </p>
    }
}

// -------- Bevy Systems --------
pub fn set_text(mut event_reader: EventReader<TextEvent>) {
    for event in event_reader.read() {
        info!("Got text from Leptos: {}", event.text);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.6, 0.9),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            range: 20.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Initialize the Bevy app that runs inside the Leptos canvas
#[cfg(target_arch = "wasm32")]
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
    .add_systems(Startup, setup_scene)
    .add_systems(Update, set_text);

    app
}
