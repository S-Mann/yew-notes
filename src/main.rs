use yew::prelude::*;

use yew_router::router::Router;
use yew_router::Switch;

mod about;
mod home;
mod navbar;

use about::component::Model as AboutModel;
use home::component::Model as HomeModel;
use navbar::component::Model as NavBarModel;

#[derive(Switch, Debug, Clone)]
enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/"]
    Home,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    _link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <NavBarModel/>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::About => html!{<AboutModel/>},
                            AppRoute::Home => html!{<HomeModel/>},
                        }
                    })
                />
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
