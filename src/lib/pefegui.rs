use druid::{Widget};
use druid::widget::{Align, Flex, Label, Tabs};
use crate::pefegui::structs::PefeApp;

pub mod structs;
pub mod enums;

pub fn build_root_widget() -> impl Widget<PefeApp> {
    let tabs = Tabs::new()
        .with_tab("DOS", Label::new("DOS"))
        .with_tab("COFF", Label::new("COFF"));

    // a label that will determine its text based on the current app data.
    //let label = Label::new(|data: &PefeApp, _env: &Env| format!("Hello {}!", data.name));
    // a textbox that modifies `name`.
    /*let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(200.0)
        .lens(PefeApp::name);*/

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(tabs);/*
        .with_spacer(20.0)
        .with_child(textbox);*/

    // center the two widgets in the available space
    Align::centered(layout)
}
