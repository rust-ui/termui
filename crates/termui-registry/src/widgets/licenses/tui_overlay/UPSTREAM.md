# Vendored source

Overlay source copied from Ferrit's `src/components/tui_overlay`, which vendors
[`jharsono/tui-overlay`](https://github.com/jharsono/tui-overlay), crate version
`0.1.2`. Ferrit's `src/components/ui/toast.rs` is also the basis for Term/UI's
animated error toast. The module stays local so widgets can build on its
overlay primitives without another crate boundary.

The upstream dual-license texts are included as `LICENSE-MIT` and
`LICENSE-APACHE`.
