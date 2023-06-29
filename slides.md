# Build Native Solana Desktop Tooling with Rust & Egui

--- :: master: title1

# Build Native Solana Desktop Tooling with Rust & ~~Egui~~ Dioxus

--- :: master: title1

# Attention Economy

## Nobody has time for anything anymore

### Why would you listen to what I have to say

--- :: master: title1

- Good tooling is the critical component to turbocharge your velocity
- Build a native app that users / team members can install
- No unholy mash-up of NPM packages, localhost servers, and broken dependencies
- Doesn't break every second week. Binaries just keep working
- Use all the native Solana libraries directly from Rust.

---

- Dioxus is Native apps on a Turbo
- One codebase can support many targets
  - Desktop app: macOS, Linux, Windows
  - Web App
  - Fullstack Server app
  - LiveView apps
  - TUI (Terminal App)
  - Mobile app (in the future)
- Hot Reload to iterate quickly

---

# Write tooling once

# Run Tooling anywhere

---

# Super Easy

---

# Isn't Rust supposed to be difficult?

---

# Solana & Rust: Chewing Glass

![Iron Teeth, the mark of a Senior Solana Developer](image)

---

# Truth

## Developing on Solana is kinda hard

---

# Rust, most beloved programming language on StackOverflow for the past years

---

# How can it be teeth-shattering hard and most beloved at the same time?

---

image ists not me its y0uo

---

# Solana is still in development

- Things change
- Documentation becomes outdated
- If there even is documentation
- Rust makes an already difficult platform even more difficult

---

# Developing UI Tooling with Rust is fun

---

explorer

---

inspector

---

ebou

---

# Dioxus

<https://dioxuslabs.com>

---

# Simple Example

``` rs
fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    render! {
        h1 { "Count: {count}" }
        button { onclick: move |_| count += 1, "+" }
        button { onclick: move |_| count -= 1, "-" }
    }
}
```

---

image of counter example

---

``` rs
fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    render! {

    }
}

#[inline_props]
fn Button<'a>(cx: Scope<'a>, title: &'a str, onclick: EventHandler<'a, ()>) -> Element<'a> {
    render! {
      button {
        class: "button-style",
        onclick: move |_| onclick.call(()),
        "{title}"
      }
    }
}

```

---

# So how does this work?

---

schema1

---

schema2

---

# But isn't that like Electron?

- Electron brings its own Blink Web WebView
- Chrome is slower than WebView
- Electron is slow because Javascript is slow
- Dioxus uses native Rust
- The WebView doesn't process. Just a renderer.
- Millions of dollars have been spend on making HTML & CSS Phast
- Rust consumes far less memory than Javascript

---

# But the UI ain't native

- Actually, WebView Buttons, Checkboxes are all native by default
- Text Rendering is native by default
- Video Rendering: Native.
- Accessibility: Native
- And, you can always hook up a different renderer

---

platforms image

---

tui image

---

blitz image

---

# ~Jesus~ Solana Enters the chat

---

# What we will be building

screenshot of app

---

# Demo

---

Free NFT from the collection for the first good question after the talk

---

crates io solana + nft

---

# Get the account balance

``` rs
pub fn get_balance(pubkey: &Pubkey) -> Result<SolanaBalance, SolanaError> {
    let rpc = RpcClient::new("https://api.mainnet-beta.solana.com");

    let acc = rpc.get_account(&pubkey)?;
    Ok(SolanaBalance {
        lamports: acc.lamports,
        sol: (acc.lamports as f64) / 1000000000.0,
    })
}
```

---

# Getting Tokens

``` rs
let token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;
let rpc = RpcClient::new("https://api.mainnet-beta.solana.com".into());

let pubkey = Pubkey::from_str("Eggs2iFRUm3RirAJuWeAdBRyk6br9q8TqFkGbPv11px4")?;
let commitment = CommitmentConfig::finalized();

let accounts = rpc
  .get_token_accounts_by_owner_with_commitment(
      &pubkey,
      TokenAccountsFilter::ProgramId(token_program),
      commitment,
  )?
  .value;
```

---

## Loading NFTs

Some more complexity (thanks Solana). Example in the Repository.

``` rs
pub struct Nft {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
}
```

---

## Loading NFTs

``` rs
pub fn app(cx: Scope) -> Element {
    let data = use_future(cx, (), |_| async move {
        get_nfts(
            "Eggs2iFRUm3RirAJuWeAdBRyk6br9q8TqFkGbPv11px4",
            Cluster::MainnetBeta,
        )
        .await
    });
```

---

## Waiting for Data

``` rs
let data = ...;
match data.value() {
    Some(Ok(nfts)) => render! {
        List {
            nfts: nfts.clone()
        }
    },
    Some(Err(e)) => render! {
        div { "Error: {e:?}" }
    },
    None => render! {
        render! {
            div { class: "loader" }
        }
    },
}
```

---

## Displaying the List

``` rs
#[inline_props]
fn List(cx: Scope, nfts: Vec<Nft>) -> Element {
    render! {
        div {
            class: "nft-list",
            for item in nfts.iter() {
                h3 {
                    "{item.name}"
                }
            }
        }
    }
}
```

---

# Running it as an app

``` rs
fn main() {
    let style = include_str!("../style.css");
    let config = Config::new()
        .with_window(default_window())
        .with_custom_head(
            format!(
                r#"
        <title>Hello Solana</title>
        <style>{style}</style>
        "#
            )
            .into(),
        );
    dioxus_desktop::launch_cfg(solana_example::app, config);
}
```

---

# Next

## Display the NFT properly

``` rs
#[inline_props]
fn Nft(cx: Scope, item: Nft) -> Element {
    render! {
        div {
            key: "{item.name}",
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
```

---

``` rs
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
```

---

# Define New Window Context

``` rs
fn popup(cx: Scope) -> Element {
    cx.render(rsx! {
        img { 
            src: "...",
        }
    })
}

```

---

# Open new window

``` rs
let window = dioxus_desktop::use_window(cx);
let dom = VirtualDom::new(popup);
window.new_window(dom, Default::default());
```

## Actually more complex. Docs in the code

---

# Tasks

---

Take any of the three apps (app1, app2, app3) and try to improve upon it

- app1: Basics
- app2: Display proper Nft
- app3: Open new Window, Search for addresses

---

# Other ideas

- Click a Nft to show more info (description, attributes)
- Open Mint in a new browser
- Show Nfts in a Sidebar
- Figure out how to show Nfts and other tokens
- Show the amount of Nfts that user owns
- Just to something completely different with Dioxus

---

# Repository
