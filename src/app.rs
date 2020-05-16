use log::info;
use num_traits::FromPrimitive;
use rand::Rng;
use tonality::Tpc;
use yew::prelude::*;

use crate::chord::{Chord, Kind};

pub struct App {
    link: ComponentLink<Self>,
    chord: Chord,
    revealed: bool,
}

pub enum Msg {
    Reveal,
    Next,
}

fn random_chord() -> Chord {
    use crate::chord::{Tetrad, Triad};

    let mut rng = rand::thread_rng();
    let (tpc_lo, tpc_hi) = (Tpc::Fb as isize, Tpc::Bs as isize);
    loop {
        let tpc_num = rng.gen_range(tpc_lo, tpc_hi);
        let root: Tpc = FromPrimitive::from_isize(tpc_num).unwrap();
        let chord = match rng.gen_range(0, 9) {
            0 => Kind::Triad(Triad::Maj),
            1 => Kind::Triad(Triad::Min),
            2 => Kind::Triad(Triad::Dim),
            3 => Kind::Triad(Triad::Aug),
            4 => Kind::Tetrad(Tetrad::Dom7),
            5 => Kind::Tetrad(Tetrad::Dim7),
            6 => Kind::Tetrad(Tetrad::Maj7),
            7 => Kind::Tetrad(Tetrad::Min7),
            _ => Kind::Tetrad(Tetrad::Min7b5),
        };
        if let Some(chord) = Chord::new(root.clone(), chord.clone()) {
            return chord;
        } else {
            info!("Out of range with {:?}, {:?}", root, chord)
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let chord = random_chord();
        let revealed = false;
        Self {
            link,
            chord,
            revealed,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Reveal => self.revealed = true,
            Msg::Next => {
                self.chord = random_chord();
                self.revealed = false
            }
        }
        true
    }

    fn view(&self) -> Html {
        let answer = if self.revealed {
            self.chord.to_string()
        } else {
            String::new()
        };
        let button = if self.revealed {
            let on_next = self.link.callback(|_| Msg::Next);
            html! {
                <>
                <button class="btn btn-primary" onclick=on_next>{ "Next chord" }</button>
                </>
            }
        } else {
            let on_reveal = self.link.callback(|_| Msg::Reveal);
            html! {
                <button class="btn btn-primary" onclick=on_reveal>{ "Reveal answer" }</button>
            }
        };
        html! {
            <>
            <main class="container">
                <h1>{ "Identify the chord" }</h1>
                <div class="score-wrapper">{ self.chord.to_svg() }</div>
                <div class="answer">{ answer }</div>
                { button }
            </main>
            <footer class="footer">
                <div class="container text-muted">
                    { "The graphics are generated in your browser as an SVG using Steinberg's Bravura musical font. " }
                    { "The app is programmed in Rust using the Yew framework. " }
                    { "Repository: "}
                    <a href="https://github.com/stigjb/chord-quiz">{ "stigjb/chord-quiz" }</a>
                </div>
            </footer>
            </>
        }
    }
}
