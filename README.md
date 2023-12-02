Prettygooey is a set of themed UI components for the [iced](https://iced.rs/) GUI library.

![Showcase](docs/img/showcase.png)

⚠️ **Prettygooey, like iced, is experimental software.** ⚠️

# Getting started

This guide assumes you have basic knowledge of iced.

In your [`Sandbox`](https://docs.rs/iced/latest/iced/trait.Sandbox.html)'s constructor, create an instance of `prettygooey::theme::Theme`.

```rust
fn new() -> Self {
	Self {
		theme: Theme {
			accent_color: AccentColor::Magenta,
		},
	}
}
```

In your `Sandbox`'s `view` function, return an instance of our primary container to apply the background.

```rust
fn view(&self) -> Element<'_, Self::Message> {
	// Pass a row or column of widgets to the container
	self.theme.primary_container(column![]).into()
}
```

All supported widgets can be created via the Theme instance.

```rust
let button = self.theme.button("Click me");
```

Widgets may provide optional customizations. Take the Text widget for example. By importing the extension trait `prettygooey::theme::ThemeExt`, you expose a `variant` method that'll let you switch the variant to Dimmed:

```rust
self.theme
	.text("Theme selection")
	.variant(TextVariant::Dimmed),
```
