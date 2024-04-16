[Stylist tailwind](Stylist-tailwind) contains a bunch of tailwind classes that you can use in Stylist.

## Install

Add the following to your `Cargo.toml`:

```toml
stylist-tailwind = "0.2.0"
```

## Usage

For detailed usage, please see
[documentation](https://docs.rs/crate/stylist-tailwind/).

```rust
use yew::prelude::*;
use stylist::Style;

#[styled_component]
fn MyStyledComponent() -> Html {
    html! {<div class={Style::new(tw!("w-full px-4 my-8")).unwrap()}>{"Hello World!"}</div>}
}
```
