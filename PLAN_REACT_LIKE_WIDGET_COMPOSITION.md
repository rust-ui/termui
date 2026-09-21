# Plan: React-Like Composition Syntax for Term/UI

**Status:** Card renderer implemented; JSX-like macro remains a proposal.

**Scope:** API and syntax proposal; no implementation decision yet.

## Summary

Term/UI's `Dialog` and `Card` APIs expose composable parts, but callers still render each part and route layout areas themselves. Explore a JSX-like Rust macro so nested component structure is easier to read and closer to the React API that Term/UI users may already know. The Card renderer and Rust demo now provide a second composition case; the macro remains unimplemented.

Rust has no native JSX syntax. A function-like macro can define an embedded UI DSL and expand it into ordinary Rust. The macro would improve the call-site syntax; it would not add React's runtime, implicit state, or event system.

## Current API

`Dialog` owns its open/close animation state. The application activates the trigger, advances animation, renders the overlay, and then renders children into the returned regions.

### Before

```rust
let trigger = DialogTrigger::new("Delete project");
let mut dialog = Dialog::new().with_duration(Duration::from_millis(180));

trigger.activate(&mut dialog);
dialog.tick(Duration::from_millis(180));
trigger.render(frame, trigger_area);

if let Some(areas) = DialogContent::new()
    .width(Constraint::Length(36))
    .height(Constraint::Length(10))
    .render(frame, area, &mut dialog)
{
    DialogHeader::new(DialogTitle::new("Delete project?"))
        .description(DialogDescription::new("This action cannot be undone."))
        .render(frame, areas.header);

    frame.render_widget(
        Paragraph::new("Project files and settings will be removed."),
        areas.body,
    );

    let actions = DialogFooter::new().render(frame, areas.footer);
    let [_spacer, cancel_area, confirm_area] = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(13),
        Constraint::Length(13),
    ])
    .areas(actions);
    DialogClose::new("Cancel").render(frame, cancel_area);
    Button::new("Delete")
        .variant(ButtonVariant::Destructive)
        .render(frame, confirm_area);
}
```

This is explicit and flexible, but visual hierarchy is spread across render calls. The caller must connect each returned area to its child.

## Desired Call-Site Shape

Keep the familiar React hierarchy, wrapped in a Rust macro invocation. The example below is a syntax sketch, not working API:

### After (proposal)

```rust
termui! {
    frame: frame,
    area: area,

    <Dialog state={&mut dialog}>
        <DialogTrigger label={"Delete project"} />
        <DialogContent
            width={Constraint::Length(36)}
            height={Constraint::Length(10)}
        >
            <DialogHeader>
                <DialogTitle>{"Delete project?"}</DialogTitle>
                <DialogDescription>
                    {"This action cannot be undone."}
                </DialogDescription>
            </DialogHeader>

            <DialogBody>
                {Paragraph::new("Project files and settings will be removed.")}
            </DialogBody>

            <DialogFooter>
                <DialogClose label={"Cancel"} />
                <Button variant={ButtonVariant::Destructive}>
                    {"Delete"}
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
}
```

The syntax borrows JSX's nested tags and component names. Rust values and text sit inside `{ ... }`, where they remain ordinary Rust expressions. The final syntax should be validated with a small prototype before promising this exact shape.

`DialogBody` in this sketch is a proposed DSL slot, not a current Term/UI type.

## Second Composition Case: Card

The new `Card` renderer is the second composition case after `Dialog`. It checks a different shape: a stateless root with header, content, and footer slots; nested title and description; and an optional action positioned beside the header text.

Term/UI now has a generic `Card` in `crates/termui-widgets/src/card.rs`. `Panel` remains a rounded `Block` shell, while `RadioCard` is a selectable control. The new `Card` exposes Shadcn-style header, content, and footer composition.

### Before (low-level building blocks)

The equivalent layout today uses `Panel` for the border and Ratatui layout/render calls for each region:

```rust
let panel = Panel::new().block();
let inner = panel.inner(area);
frame.render_widget(panel, area);

let [header, content, footer] = Layout::vertical([
    Constraint::Length(3),
    Constraint::Min(0),
    Constraint::Length(3),
])
.areas(inner);
let [header_text, action] = Layout::horizontal([
    Constraint::Min(0),
    Constraint::Length(14),
])
.areas(header);
let [title, description] = Layout::vertical([
    Constraint::Length(1),
    Constraint::Min(0),
])
.areas(header_text);

frame.render_widget(Paragraph::new("Card Title"), title);
frame.render_widget(Paragraph::new("Card Description"), description);
frame.render_widget(Paragraph::new("Card Action"), action);
frame.render_widget(Paragraph::new("Card Content"), content);
frame.render_widget(Paragraph::new("Card Footer"), footer);
```

Before the `Card` component, this had the right ingredients but no semantic card parts. The caller owned the slot layout, including the header's two-column arrangement.

### After (proposal)

```rust
termui! {
    frame: frame,
    area: area,

    <Card>
        <CardHeader>
            <CardTitle>{"Card Title"}</CardTitle>
            <CardDescription>{"Card Description"}</CardDescription>
            <CardAction>{"Card Action"}</CardAction>
        </CardHeader>
        <CardContent>
            {Paragraph::new("Card Content")}
        </CardContent>
        <CardFooter>
            {Paragraph::new("Card Footer")}
        </CardFooter>
    </Card>
}
```

This follows Shadcn's documented composition. `CardAction` should occupy the header's top-right slot; title and description use the remaining header area. The macro or Card renderer needs to derive that layout from the named parts. This is still a syntax sketch, not implemented API. [Shadcn Card documentation](https://ui.shadcn.com/docs/components/base/card)

### What Card Tests

- **Nested and named slots:** `CardHeader` owns three semantic children; the other parts are siblings under `Card`.
- **Nonuniform layout:** header action sits beside title/description, while content and footer stack vertically.
- **Sizing rules:** Ratatui needs a concrete `Rect` and row constraints. Decide whether Card takes the caller's full area, supports explicit header/footer rows, or requires child-specific sizing hints.
- **Optional action and content:** define defaults for a missing `CardAction`, empty description, or omitted footer.
- **Styling boundaries:** choose where padding, borders, separators, and compact sizing live: in Card part components, Card configuration, or caller-provided styles.

Card gives the DSL a useful test without Dialog's open state and event-routing questions. If both examples fit a common grammar while using different layout rules, that supports a reusable syntax layer over widget-specific renderers.

## Why Explore This

- **Make composition visible.** Parent-child structure becomes clear at a glance, especially for dialogs with headers, body content, and actions.
- **Reduce area plumbing.** The macro can calculate dialog regions and render children into the matching slots.
- **Lower the learning curve.** React and shadcn-style names are familiar to many Term/UI users; nested syntax makes the relationship recognizable.
- **Keep customization.** A caller should still be able to insert a regular Ratatui widget or a Term/UI component as body content.
- **Preserve Rust control.** State, time, and input stay explicit and type-checked; the macro expands to normal Rust calls.

The aim is React-like composition and readability, not a React runtime clone.

## Proposed Direction

Prototype a function-like macro named `termui!` (name open) that expands nested tags into the existing component constructors and render methods. Keep current Rust APIs available as the underlying primitives and as an escape hatch.

Treat Dialog and Card as validation cases, not as a commitment to implement both components in the first change. Dialog tests animated state and named areas; Card tests static nested slots and asymmetric header layout. If both fit a common syntax while keeping rendering rules local to each widget, that is evidence for a reusable DSL. If not, prefer the current component API or a Rust builder over a Dialog-only macro.

Likely implementation options:

| Option | Benefits | Costs |
|---|---|---|
| Keep explicit `.render()` calls | Smallest API, idiomatic Ratatui, no parser | Call sites remain verbose; hierarchy is less obvious |
| Add a builder with child render closures | Typed Rust API, no custom tag parser | Less JSX-like; closure and lifetime API needs care |
| Prototype a `macro_rules!` DSL | No separate proc-macro crate; compile-time expansion | Nested tags and arbitrary child expressions can make matching brittle |
| Add a function-like procedural macro | Better control over nested syntax and diagnostics | Separate proc-macro crate; parser, maintenance, compile-time complexity |

Recommendation: first prove the syntax and rendering contract with a small function-like macro prototype. Add a proc-macro crate only if the prototype shows value across several widgets. Check workspace conventions and dependency policy before selecting parser dependencies; do not add a dependency just to match the visual syntax.

## State and Event Model

Rendering syntax cannot decide how an application handles input. Application code must still open and close the dialog from its event loop, and call `tick` as time advances. For example:

```rust
if trigger_was_activated {
    dialog.open();
}
if let Some(position) = outside_click {
    dialog.close_on_outside_click(position);
}
dialog.tick(elapsed);
```

The macro may associate a trigger with its parent dialog during rendering, but mouse hit-testing, keyboard focus, activation, and application-specific callbacks need a defined event API. Do not imply that JSX-like syntax alone provides React state or browser event behavior.

## Challenges and Design Questions

1. **Text and Rust expressions:** JSX allows plain text between tags. Rust macros receive tokens, so the first version should require text and arbitrary values inside `{ ... }`. Supporting unquoted text would need whitespace and punctuation rules and may create surprising parsing behavior.
2. **Child types:** `Paragraph` implements Ratatui's widget rendering model; Term/UI components expose their own render methods. Decide how the DSL distinguishes ordinary Ratatui widgets from Term/UI components without erasing useful types or forcing every widget into one abstraction.
3. **Layout slots:** Decide whether child tags such as `DialogHeader`, `DialogBody`, and `DialogFooter` are named slots, or whether the macro partitions direct children by component type. Named slots are clearer and easier to validate.
4. **Optional and duplicate children:** Define defaults and diagnostics for missing content, two headers, multiple bodies, or a footer without a content shell.
5. **Ownership and lifetimes:** Current render methods often consume component values. The macro must preserve Rust's ownership rules and avoid requiring unnecessary clones or `'static` closures.
6. **State ownership:** React context can pass state implicitly. Rust should keep the parent `Dialog` state explicit at the root or in the application's state, then generate direct borrows for child behavior.
7. **Events and focus:** A trigger that renders a button does not automatically receive mouse or keyboard events. Define how applications report activation and focus, or keep event routing in the app.
8. **Errors and diagnostics:** Invalid tag nesting and unsupported attributes should point to the relevant source token and explain expected children or attributes.
9. **Extensibility:** A closed list of supported tags is easy to implement but limits custom components. A general child-rendering contract is flexible but may add a broad abstraction before multiple widgets need it.
10. **Macro crate and dependencies:** Procedural macros must live in a proc-macro crate separate from the crate that invokes them. This adds workspace surface and maintenance. Evaluate parser dependencies and compile impact first.
11. **Ratatui frame model:** Ratatui redraws the UI each frame. The macro should expand to immediate render calls and not imply a retained component tree or automatic state updates.
12. **Copy and docs:** Document the macro as Rust syntax inspired by JSX. Keep examples honest about explicit state updates and input handling.

## Suggested Work Stages

1. Write a tiny syntax prototype for `Dialog`, using only existing state and render behavior.
2. Express the Card tree with the same syntax; specify header action placement and row-sizing rules.
3. Compare whether Dialog and Card share syntax while delegating layout/state behavior to their own renderers.
4. Decide between a macro, a builder API, and explicit render calls using readability, diagnostics, compile cost, and API complexity.
5. If proceeding, design event routing separately from rendering syntax. Keep state ownership and animation ticking explicit.
6. Document the accepted syntax with side-by-side examples and limitations. Add a concise `CHANGELOG.md` entry for visible or architectural changes.

## Acceptance Criteria for a Prototype

- A dialog can express trigger, content, header, title, description, body, and footer in nested source order.
- Arbitrary Ratatui widget content remains usable in the body.
- Existing explicit component APIs remain usable.
- State updates, animation ticking, and event routing stay clear in the example.
- Invalid nesting and unknown attributes produce useful compile errors.
- Both Dialog and Card can use the proposed composition model without special-case parser growth; each keeps its own layout and state rules.
- No runtime component tree or unnecessary cloning is introduced just to imitate JSX.

## Sources

- Rust Reference, [Macros](https://doc.rust-lang.org/reference/macros.html): Rust macros extend syntax and use a macro invocation with a delimiter.
- Rust Reference, [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html): function-like procedural macros process token streams and must be defined in a `proc-macro` crate.
- Rust By Example, [Domain Specific Languages](https://doc.rust-lang.org/stable/rust-by-example/macros/dsl.html): macros can provide a small embedded language that expands to regular Rust.
- React documentation, [Passing JSX as children](https://react.dev/learn/passing-props-to-a-component): nested JSX is passed to a component through its `children` prop.
- Shadcn/ui, [Card](https://ui.shadcn.com/docs/components/base/card): documented Card composition and the header action's top-right placement.
- Ratatui documentation, [Rendering](https://ratatui.rs/concepts/rendering/): Ratatui uses immediate-mode rendering and redraws the UI each frame.
- Ratatui documentation, [Widgets](https://ratatui.rs/concepts/widgets/): widgets can be composed by rendering child widgets into selected areas.
