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
                    <div class="block is-size-3">
                        <span style="margin-right: 1rem">
                            {"Han Ha"}
                        </span>
                        <span class="icon">
                            <a href="https://github.com/hanbeul">
                                <i class="fab fa-github"></i>
                            </a>
                        </span>
                    </div>
                    <div class="block"><span class="is-size-4" style="margin-right: 0.5rem">{"Software Engineer"}</span><span>{"located in the DC Metro area"}</span></div>
                    <div class="block">
                        <span>{"Javascript, TypeScript, React, Python, Docker"}</span>
                    </div>
                    <div class="block">
                        {"This website was written in Rust and compiled to WebAssembly!"}
                    </div>
                    <div class="block">
                    </div>
                </div>
            </main>
        }
    }
}
