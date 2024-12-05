mod buttons;
mod content;

use yew::prelude::*;

const QUOTES: [&str; 7] = [
    "who are you",
    "lorem ipsum dolor sit amet",
    "hot garfields in your area",
    "remember to drink water",
    "i am not a web developer",
    "it's nice, isn't it?",
    "do you like hurting other people?",
];

#[function_component(Page)]
fn page() -> Html {
    let quote = use_state(|| fastrand::choice(QUOTES).unwrap());
    html! {
    <>
        <div class="box">
            <h1 class="main-title"> { "futile" } </h1>
        </div>
        <hr class="separator" />
        <div class="box">
            { content::content() }
        </div>
        <div class="footer-container">
            { buttons::buttons() }
            <div class="footer quote">
                <p> { "« " } { *quote } { " »" } </p>
            </div>
            <div class="footer links">
                <a class="click" target="_blank" href="https://codeberg.org/futile"> { "git" } </a>
                <a class="click" target="_blank" href="https://steamcommunity.com/id/elituf"> { "steam" } </a>
            </div>
        </div>
    </>
    }
}

fn main() {
    yew::Renderer::<Page>::new().render();
}
