# Widgets plan

| Widget | Status |
|---|---|
| Breadcrumbs | ⚠️ Waiting |
| Button | ✅ Done |
| Calendar | ⚠️ Waiting |
| Chart | ⚠️ Waiting |
| Checkbox | ⚠️ Waiting |
| Code viewer | ⚠️ Waiting |
| Color picker | ✅ Done |
| Command palette | ⚠️ Waiting |
| Confirmation prompt | ⚠️ Waiting |
| Date picker | ⚠️ Waiting |
| Dialog | ✅ Done |
| Diff viewer | ⚠️ Waiting |
| Empty state | ⚠️ Waiting |
| File picker | ⚠️ Waiting |
| Form field with validation | ⚠️ Waiting |
| Gauge | ⚠️ Waiting |
| Help screen | ⚠️ Waiting |
| Key binding bar | ✅ Done |
| List | ⚠️ Waiting |
| Loading state | ⚠️ Waiting |
| Log viewer | ⚠️ Waiting |
| Markdown viewer | ⚠️ Waiting |
| Menu | ⚠️ Waiting |
| Multi-select list | ⚠️ Waiting |
| Notification / toast | ✅ Done |
| Panel | ✅ Done |
| Password input | ⚠️ Waiting |
| Progress bar | ⚠️ Waiting |
| Radio group | ✅ Done |
| Resizable layout | ⚠️ Waiting |
| Scrollbar | ⚠️ Waiting |
| Search input | ⚠️ Waiting |
| Select list | ✅ Done |
| Sortable table | ⚠️ Waiting |
| Sparkline | ⚠️ Waiting |
| Split pane | ⚠️ Waiting |
| Spinner | ⚠️ Waiting |
| Status bar | ⚠️ Waiting |
| Table | ⚠️ Waiting |
| Tabs | ⚠️ Waiting |
| Text area | ⚠️ Waiting |
| Text input | ✅ Done |
| Toggle switch | ⚠️ Waiting |
| Tree view | ⚠️ Waiting |


## Button

```text


            +----------+
            |  Button  |
            +----------+


```

The box represents the button's blue area.

## Calendar

```text
       September 2026
 Su Mo Tu We Th Fr Sa
        1  2  3  4  5
  6  7  8  9 10 11 12
 13 14 15 16 17 18 19
 20 21 22 23 24 25 26
 27 28 29 30
```

## Chart

```text
  8 |          █
  6 |      █   █
  4 |  █   █   █
  2 |  █   █ █ █
  0 +------------
      A   B C D
```

## Checkbox

```text

        [ ]  Unchecked
        [x]  Checked

```

## Breadcrumbs

```text
Home / Projects / TermUI
```

## Code viewer

```text
 1  fn main() {
 2      println!("Hello");
 3  }
```

## Color picker

```text
╭─ Accent color ───────────────────╮
│ ● Slate  ● Gray  ● Red   ● Orange│
│ ● Green  ● Teal  ▣ Blue  ● Violet│
│ Selected  ██ Blue  #3B82F6       │
╰──────────────────────────────────╯
```

## Command palette

```text
  > Search commands...
    Open file
    Toggle theme
    Quit
```

## Confirmation prompt

```text
  Delete this file?
  [ Cancel ]  [ Delete ]
```

## Date picker

```text
  Select date: 2026-09-21
```

## Dialog

```text
╭─ Delete project? ─────────────╮
│ This action cannot be undone. │
│                 [ Cancel ] [ Delete ]
╰───────────────────────────────╯
```

## Diff viewer

```text
- old value
+ new value
  unchanged
```

## Empty state

```text
  No projects yet
  Create a project to get started.
       [ Create project ]
```

## File picker

```text
  src/
  > main.rs
    lib.rs
    widgets/
```

## Form field with validation

```text
  Email
  [ name@example.com       ]
  ✓ Valid email address
```

## Gauge

```text
  CPU  [████████░░]  80%
```

## Help screen

```text
  Keyboard shortcuts
  ↑/↓  Navigate    Enter  Select
  Esc  Back        ?      Help
```

## List

```text
  > First item
    Second item
    Third item
```

## Loading state

```text
  Loading data...
```

## Log viewer

```text
  10:42:01 INFO  Server started
  10:42:03 WARN  Retry scheduled
  10:42:05 ERROR Request failed
```

## Markdown viewer

```text
  # Heading
  Formatted **text** and a list:
  • First item
```

## Menu

```text
  File   Edit   View
  ┌──────────┐
  │ Open     │
  │ Save     │
  │ Quit     │
  └──────────┘
```

## Multi-select list

```text
  [x] Rust
  [ ] Go
  [x] TypeScript
```

## Notification / toast

```text
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╭─ Error ───────────────────────────────╮
│ Theme preferences could not be saved  │
│                                  x   │
╰───────────────────────────────────────╯
```

## Password input

```text
  Password
  [ ••••••••••••          ]
```

## Progress bar

```text
  Downloading  [██████░░░░]  60%
```

## Radio group

```text
╭─ ◉ Standard ───╮ ╭─ ○ High contrast ─╮
│ Balanced palette│ │ Stronger borders │
╰────────────────╯ ╰───────────────────╯
```

## Resizable layout

```text
  Sidebar       │ Main panel
  Files         │ Content
  Projects      │ Details
```

## Scrollbar

```text
  Content line 1  ░
  Content line 2  █
  Content line 3  ░
```

## Search input

```text
  / Find in files...
```

## Sortable table

```text
  Name ↑       Size
  README.md    2 KB
  src/         4 KB
```

## Sparkline

```text
  Activity  ▁▃▂▅▄▇▆█
```

## Split pane

```text
  Editor         │ Preview
  # Hello        │ Hello
```

## Spinner

```text
  ◐ Working...
```

## Status bar

```text
  main  │  Ln 12, Col 4  │  UTF-8
```

## Table

```text
  Name       Status
  API        Online
  Worker     Offline
```

## Tabs

```text
  [Overview]  Settings  Logs
```

## Text area

```text
  Write a message...
  │
  │
```

## Text input

```text
╭─ Email ───────────────────────────╮
│ ada@example.com                   │
╰───────────────────────────────────╯
```

## Toggle switch

```text
  Notifications  [ ON ]
```

## Tree view

```text
  ▾ src/
    ├─ main.rs
    └─ widgets/
       └─ button.rs
```
