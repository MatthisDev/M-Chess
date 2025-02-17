use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
                { self.chessboard() }
            </div>
        }
    }
}

impl Model {
    fn chessboard(&self) -> Html {
        let squares = (0..64).map(|i| {
            let class = if (i / 8 + i % 8) % 2 == 0 {
                "white"
            } else {
                "black"
            };
            html! { <div class={format!("square {}", class)}></div> }
        });

        html! {
            <div class="chessboard">
                { for squares }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
