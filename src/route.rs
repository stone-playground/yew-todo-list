use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/all")]
    All,
    #[at("/active")]
    Active,
    #[at("/completed")]
    Completed,
}

pub type AppLink = Link<AppRoute>;
