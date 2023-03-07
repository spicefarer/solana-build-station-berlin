#![allow(non_snake_case)]
use crate::solana::{get_nfts, Cluster, Nft};
use dioxus::prelude::*;

pub fn app2(cx: Scope) -> Element {
    let data = use_future(cx, (), |_| async move {
        get_nfts(
            "Eggs2iFRUm3RirAJuWeAdBRyk6br9q8TqFkGbPv11px4",
            Cluster::MainnetBeta,
        )
        .await
    });

    render!(div {
        class: "container",
        match data.value() {
            Some(Ok(nfts)) => render! {
                List {
                    nfts: nfts.clone()
                }
            },
            Some(Err(e)) => render! {
                div {
                    "Error: {e:?}"
                }
            },
            None => render! {
                render! {
                    div {
                        align_self: "center",
                        class: "loader"
                    }
                }
            },
        }
    })
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
fn List(cx: Scope, nfts: Vec<Nft>) -> Element {
    render! {
        div {
            class: "nft-list",
            for item in nfts.iter() {
                Nft {
                    item: item.clone()
                }
            }
        }
    }
}

#[inline_props]
fn Nft(cx: Scope, item: Nft) -> Element {
    render! {
        div {
            key: "{item.name}",
            onclick: move |_| {
                // Do something with the nft?
            },
            class: "nft",
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
