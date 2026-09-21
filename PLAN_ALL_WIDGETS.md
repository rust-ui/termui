# Widgets plan

Mode describes each demo: Static Rust frame or Dynamic Ratzilla/WASM.

| Widget | Mode | Status |
|---|---|---|
| Breadcrumbs | Static | ✅ Done |
| Button | Dynamic | ✅ Done |
| Card | Static | ✅ Done |
| Calendar | Static | ✅ Done |
| Chart | Static | ✅ Done |
| Checkbox | Static | ✅ Done |
| Color picker | Static | ✅ Done |
| Command palette | Static | ✅ Done |
| Confirmation prompt | Static | ✅ Done |
| Date picker | Static | ✅ Done |
| Dialog | Dynamic | ✅ Done |
| Diff viewer | Static | ✅ Done |
| Empty state | Static | ✅ Done |
| File picker | Static | ✅ Done |
| Form field with validation | Static | ✅ Done |
| Progress | Static | ✅ Done |
| Resizable Layout | Dynamic | ✅ Done |
| Help screen | Static | ✅ Done |
| List | Static | ✅ Done |
| Loading state | Static | ✅ Done |
| Log viewer | Static | ✅ Done |
| Markdown viewer | Static | ✅ Done |
| Menu | Static | ✅ Done |
| Multi-select list | Static | ✅ Done |
| Notification / toast | Static | ✅ Done |
| Panel | Static | ✅ Done |
| Password input | Static | ✅ Done |
| Progress bar | Static | ✅ Done |
| Radio card | Static | ✅ Done |
| Resizable layout | Static | ✅ Done |
| Scrollbar | Static | ✅ Done |
| Search input | Static | ✅ Done |
| Select list | Static | ✅ Done |
| Sortable table | Static | ✅ Done |
| Sparkline | Static | ✅ Done |
| Split pane | Static | ✅ Done |
| Spinner | Static | ✅ Done |
| Status bar | Static | ✅ Done |
| Table | Static | ✅ Done |
| Tabs | Static | ✅ Done |
| Text area | Static | ✅ Done |
| Text input | Dynamic | ✅ Done |
| Tooltip | Static | ✅ Done |
| Toggle switch | Static | ✅ Done |
| Tree view | Static | ✅ Done |


## Button

```text


            +----------+
            |  Button  |
            +----------+


```

The box represents the button's blue area.

## Card

```text
╭────────────────────────────────────────╮
│ Card Title                             │
│ Card Description                       │
│ Card Content                           │
│────────────────────────────────────────│
│ Card Footer                            │
╰────────────────────────────────────────╯
```

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

## File picker

```text
╭─ Files ─────╮
│   ▾ src/    │
│ >   main.rs │
│       lib.rs│
╰─────────────╯
```

## Resizable layout

```text
Sidebar       │ Main panel
Files         │ Content
Projects      │ Details
```

## Sortable table

```text
Name ↑       Size
README.md    2 KB
src/         4 KB
```

## Split pane

```text
Editor         │ Preview
# Hello        │ Hello
```

## Table

```text
Name       Status
API        Online
Worker     Offline
```

## Tree view

```text
▾ src/
  main.rs
  ▾ widgets/
    button.rs
```

## Markdown viewer

```text
Heading
Formatted text and a list:
• First item
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

## Form field with validation

```text
  Email
  [ name@example.com       ]
  ✓ Valid email address
```

## Progress

```text
  CPU  [████████░░]  80%
```

## Resizable Layout

```text
  Sidebar 32%  │  Main panel 68%  │  Active
  ┌──────────┐┃┌──────────────────────┐
  │ Files    │┃│ Content              │
  │ Projects │┃│ Details              │
  └──────────┘┃└──────────────────────┘
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

## Radio card

```text
╭─ ◉ Standard ───╮ ╭─ ○ High contrast ─╮
│ Balanced palette│ │ Stronger borders │
╰────────────────╯ ╰───────────────────╯
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

## Sparkline

```text
  Activity  ▁▃▂▅▄▇▆█
```

## Spinner

```text
  ◐ Working...
```

## Status bar

```text
  main  │  Ln 12, Col 4  │  UTF-8
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

## Tooltip

```text
╭───────────────────────────╮
│ Press Enter to continue   │
╰───────────────────────────╯
```
