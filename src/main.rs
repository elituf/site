mod buttons;
mod content;

use gloo::timers::callback::Interval;
use yew::prelude::*;

const QUOTES: [&str; 6] = [
    "who are you",
    "lorem ipsum dolor sit amet",
    "hot garfields in your area",
    "remember to drink water",
    "i am not a web developer",
    "it's nice, isn't it?",
];

#[function_component(Page)]
fn page() -> Html {
    let quote = use_state(|| fastrand::choice(QUOTES).unwrap());
    let bullet = use_state(|| '◇');
    {
        let bullet = bullet.clone();
        use_effect(move || {
            let interval = Interval::new(1000, move || {
                bullet.set(if *bullet == '◇' { '◆' } else { '◇' });
            });
            || drop(interval)
        });
    }
    html! {
    <>
        <div class="box">
            <h1 class="main-title"> { "futile" } </h1>
        </div>
        <hr class="separator" />
        <div class="box">
            { content::content() }
        </div>
        <div style="position: absolute; bottom: 0; left: 0; right: 0;">
            { buttons::buttons() }
            <div class="footer">
                <p> { *quote } </p>
                <p style="user-select: none;"> { *bullet } </p>
                <p><a class="click" target="_blank" href="https://codeberg.org/futile"> { "git" } </a></p>
                <p><a class="click" target="_blank" href="https://steamcommunity.com/id/elituf"> { "steam" } </a></p>
            </div>
        </div>
    </>
    }
}

fn main() {
    yew::Renderer::<Page>::new().render();
}
