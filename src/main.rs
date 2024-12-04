use gloo::timers::callback::Interval;
use yew::prelude::*;

const HIRE_MESSAGE: &str = "hey!\ncould use a second pair of hands with something? programming, sys administration, or otherwise technical?\n\ni would love to work with/for you! reach out to me via email [me@futile.eu] (even just to exchange info for another platform)! paying me is optional, just be upfront with what you wish to do.";
const QUOTES: [&str; 6] = [
    "who are you",
    "lorem ipsum dolor sit amet",
    "hot garfields in your area",
    "remember to drink water",
    "i am not a web developer",
    "it's nice, isn't it?",
];

fn hire() -> Callback<MouseEvent> {
    Callback::from(|_| {
        web_sys::window()
            .unwrap()
            .alert_with_message(HIRE_MESSAGE)
            .unwrap();
    })
}

#[function_component(Page)]
fn page() -> Html {
    let hire = hire();
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
            <p class="content">
                { "hello, my name is futile. i am a weird concept from the internet (or a person from europe, depending on who you ask)." }<br /><br />
                { "i like programming (mostly rust), computers, linux, malicious software like game hacks and malware, and any vehicle whether it drives or flies." }<br /><br />
                { "everything i know about the aforementioned things is self-taught. i also speak english and russian fluently." }<br /><br />
                { "if you want to contact me, email " }<a class="click" href="mailto:me@futile.eu"> { "me@futile.eu" } </a><br /><br />
                <a class="button shiny" onclick={hire}> { "hire me!" } </a>
            </p>
        </div>
        <div class="button_container">
            <div class="buttons">
                <ul class="buttons">
                    <li><a href="https://futile.eu"><img src="static/buttons/futile.png" width="88px" height="31px" alt="button that says 'futile' in white, grotesque font, on a black background, with a white outline | by futile" /></a></li>
                    <li><a href="https://xenia.blahaj.land/"><img src="static/buttons/rose_88x31.png" width="88px" height="31px" alt="button that says 'rose' with a rose growing out of the 'o', with milk-chan to the right of the text | by rose" /></a></li>
                    <li><iframe src="https://incr.easrng.net/badge?key=futile" style="background: url(https://incr.easrng.net/bg.gif)" title="increment badge" width="88" height="31" frameborder="0"></iframe></li>
                    <li><a href="https://fedoraproject.org/spins/kde"><img src="static/buttons/fedora.png" width="88px" height="31px" alt="button that says 'powered by fedora' in white and blue, with the fedora logo to the left | by unknown; retouched by futile" /></a></li>
                    <li><a href="https://mozilla.org/firef&nbsp;x"><img src="static/buttons/firefox4.gif" width="88px" height="31px" alt="button that flashes between the texts 'tested on' and 'firefox' in firefox's orange, yellow, and purple scheme, with the firefox logo to the left | by unknown" /></a></li>
                    <li><a href="https://vscodium.com"><img src="static/buttons/vscodium.com.png" width="88px" height="31px" alt="button that says 'made with vscodium' in light blue, with the vscodium logo to the right | by unknown" /></a></li>
                    <li><img src="static/buttons/FirtniteButton.png" width="88px" height="31px" alt="button that says 'firtnite' (a misspelling of 'fortnite'), otherwise looks very similar to an old fortnite title screen | by unknown" /></li>
                    <li><a href="https://ublockorigin.com"><img src="static/buttons/ublock.png" width="88px" height="31px" alt="button (grey) that says 'ublock origin now!', with the ublock origin red logo to the left | by unknown" /></a></li>
                    <li><a href="https://rust-lang.org"><img src="static/buttons/rustNOW.png" width="88px" height="31px" alt="button (grey) that says 'rust now!', with the rust gear logo to the left and a yellow banner that says '1.82.0' on the to the bottom right | by futile" /></a></li>
                    <li><a href="https://1337x.to"><img src="static/buttons/piracy.png" width="88px" height="31px" alt="button (grey) that says 'piracy now!', with a jolly roger flag to the left and a black banner that says 'free' on the to the bottom right | by unknown; retouched by futile" /></a></li>
                </ul>
            </div>
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
