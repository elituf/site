use yew::prelude::*;

fn firtnite() -> Callback<MouseEvent> {
    Callback::from(|_| {
        web_sys::window()
            .unwrap()
            .alert_with_message("firt night")
            .unwrap();
    })
}

pub fn buttons() -> Html {
    html! {
    <div class="buttons">
        <ul class="buttons">
            <li><a href="https://futile.eu"><img
                src="static/buttons/futile.png" width="88px" height="31px"
                alt="button that says 'futile' in white, grotesque font, on a black background, with a white outline | by futile"
            /></a></li>
            <li><a href="https://xenia.blahaj.land/"><img
                src="static/buttons/rose_88x31.png" width="88px" height="31px"
                alt="button that says 'rose' with a rose growing out of the 'o', with milk-chan to the right of the text | by rose"
            /></a></li>
            <li><iframe
                src="https://incr.easrng.net/badge?key=futile" width="88" height="31" frameborder="0" title="increment badge"
                style="background: url(https://incr.easrng.net/bg.gif)"
            /></li>
            <li><a href="https://fedoraproject.org/spins/kde"><img
                src="static/buttons/fedora.png" width="88px" height="31px"
                alt="button that says 'powered by fedora' in white and blue, with the fedora logo to the left | by unknown; retouched by futile"
            /></a></li>
            <li><a href="https://mozilla.org/firefox"><img
                src="static/buttons/firefox4.gif" width="88px" height="31px"
                alt="button that flashes between the texts 'tested on' and 'firefox' in firefox's orange, yellow, and purple scheme, with the firefox logo to the left | by unknown"
            /></a></li>
            <li><a href="https://vscodium.com"><img
                src="static/buttons/vscodium.com.png" width="88px" height="31px"
                alt="button that says 'made with vscodium' in light blue, with the vscodium logo to the right | by unknown"
            /></a></li>
            <li><img
                src="static/buttons/FirtniteButton.png" onclick={ firtnite() } width="88px" height="31px"
                alt="button that says 'firtnite' (a misspelling of 'fortnite'), otherwise looks very similar to an old fortnite title screen | by unknown"
            /></li>
            <li><a href="https://ublockorigin.com"><img
                src="static/buttons/ublock.png" width="88px" height="31px"
                alt="button (grey) that says 'ublock origin now!', with the ublock origin red logo to the left | by unknown"
            /></a></li>
            <li><a href="https://rust-lang.org"><img
                src="static/buttons/rustNOW.png" width="88px" height="31px"
                alt="button (grey) that says 'rust now!', with the rust gear logo to the left and a yellow banner that says '1.82.0' on the to the bottom right | by futile"
            /></a></li>
            <li><a href="https://1337x.to"><img
                src="static/buttons/piracy.png" width="88px" height="31px"
                alt="button (grey) that says 'piracy now!', with a jolly roger flag to the left and a black banner that says 'free' on the to the bottom right | by unknown; retouched by futile"
            /></a></li>
        </ul>
    </div>
    }
}
