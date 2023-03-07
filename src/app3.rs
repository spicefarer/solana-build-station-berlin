#![allow(non_snake_case)]
use crate::solana::{get_nfts, Cluster, Nft};
use dioxus::prelude::*;
use dioxus_desktop::DesktopContext;

pub fn app3(cx: Scope) -> Element {
    let address = use_state(cx, || {
        "Eggs2iFRUm3RirAJuWeAdBRyk6br9q8TqFkGbPv11px4".to_string()
    });
    // This state holds the current nfts. When the address changes,
    // it will be cleared
    let nfts: &UseState<Vec<Nft>> = use_state(cx, || Vec::new());
    // We clone it so that it can be moved into the async future
    let cloned_nfts = nfts.clone();
    // Set up the future that gets the nfts whenever the address changes
    let future = use_future(cx, (address,), |(address,)| async move {
        match get_nfts(address.get(), Cluster::MainnetBeta).await {
            Ok(n) => cloned_nfts.set(n),
            Err(_) => (),
        };
    });

    render! {
        div {
            class: "container",
            input {
                align_self: "center",
                r#type: "text",
                value: "{address.get()}",
                autocomplete: "off",
                spellcheck: "false",
                onchange: move |evt| {
                    // Clear the old nfts
                    nfts.set(Vec::new());
                    // Set the new address
                    address.set(evt.value.clone().to_string());
                    // Force restart the future
                    future.restart();
                }
            }
            match nfts.get().len() {
                0 => rsx! {
                    Loading {}
                },
                _ => rsx! {
                    List {
                        nfts: nfts.clone()
                    }
                },
            }
        }
    }
}

#[inline_props]
fn Loading(cx: Scope) -> Element {
    render! {
        div {
            align_self: "center",
            class: "loader"
        }
    }
}

#[inline_props]
fn List(cx: Scope, nfts: UseState<Vec<Nft>>) -> Element {
    render! {
        div {
            class: "nft-list",
            for item in nfts.get().iter() {
                Nft {
                    item: item
                }
            }
        }
    }
}

#[inline_props]
fn Nft<'a>(cx: Scope<'a>, item: &'a Nft) -> Element<'a> {
    let window = dioxus_desktop::use_window(cx);
    render! {
        div {
            key: "{item.name}",
            onclick: move |_| {
                open_window(item.image.clone(), window);
            },
            class: "nft",
            style: "cursor: pointer",
            img {
                src: "{item.image}",
            }
            div {
                class: "label-secondary",
                "{item.name}"
            }
        }
    }
}

/// Open a new window with some data
fn open_window(uri: String, window: &DesktopContext) {
    // Create a new `Scope` / VirtualDom for the window
    let dom = VirtualDom::new(popup);

    // Get the current `cx` Scope out of the dom
    let dom_scope = dom.base_scope();
    // Provide the data for the popup
    dom_scope.provide_context(uri);
    // Display the window
    window.new_window(dom, Default::default());
}

fn popup(cx: Scope) -> Element {
    // Take the data out of the popup (it was provided above)
    let url = cx.consume_context::<String>().unwrap();
    cx.render(rsx! {
        img {
            // Quick hack to have the image scale nicely
            style: "object-fit: contain; width: 100%; height: 100vh; object-position: center center;",
            width: "100%",
            height: "100%",
            src: "{url}",
        }
    })
}
