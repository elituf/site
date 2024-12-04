use yew::prelude::*;

const HIRE_MESSAGE: &str = "hey!\ncould use a second pair of hands with something? programming, sys administration, or otherwise technical?\n\ni would love to work with/for you! reach out to me via email [me@futile.eu] (even just to exchange info for another platform)! paying me is optional, just be upfront with what you wish to do.";

fn hire() -> Callback<MouseEvent> {
    Callback::from(|_| {
        web_sys::window()
            .unwrap()
            .alert_with_message(HIRE_MESSAGE)
            .unwrap();
    })
}

pub fn content() -> Html {
    html! {
    <p class="content">
        { "hello, my name is futile. i am a weird concept from the internet (or a person from europe, depending on who you ask)." }<br /><br />
        { "i like programming (mostly rust), computers, linux, malicious software like game hacks and malware, and any vehicle whether it drives or flies." }<br /><br />
        { "everything i know about the aforementioned things is self-taught. i also speak english and russian fluently." }<br /><br />
        { "if you want to contact me, email " }<a class="click" href="mailto:me@futile.eu"> { "me@futile.eu" } </a><br /><br />
        <a class="button shiny" onclick={ hire() }> { "hire me!" } </a>
    </p>
    }
}
