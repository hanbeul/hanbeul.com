use yew::prelude::*;

pub struct Content {}

impl Component for Content {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Content {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="main">
                <div>
                    <img src="img/han.svg" alt="Avatar" />
                </div>
                <div class="box">
                    <div class="block is-size-3">{"Han Ha"}</div>
                    <div class="block is-size-4">{"Software Engineer"}</div>
                    <div class="block">
                        {"Javascript, Python, Full Stack, Docker, Systems Design"}
                        <br/>
                        {"Located in the DC Metro area"}
                    </div>
                    <div class="block">
                        <span class="icon">
                            <a href="https://github.com/hanbeul">
                                <i class="fab fa-github"></i>
                            </a>
                        </span>
                    </div>
                </div>
            </main>
        }
    }
}
