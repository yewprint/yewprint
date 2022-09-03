# <a href="https://yewprint.rm.rs"><img src="./yewprint-doc/src/logo.svg" height="32" /></a> <a href="https://yewprint.rm.rs">Yewprint</a> ![Rust](https://github.com/yewprint/yewprint/workflows/Rust/badge.svg) [![Netlify Status](https://api.netlify.com/api/v1/badges/17f076ed-49e5-4185-921e-5c5759de2fdb/deploy-status)](https://app.netlify.com/sites/epic-poincare-f8adaa/deploys) [![Discord](https://img.shields.io/discord/701068342760570933)](https://discord.gg/NAFcwhp) [![API Docs](https://img.shields.io/badge/docs.rs-yewprint-green)](https://yewprint.rm.rs)


It's [Blueprint](https://blueprintjs.com), but for
[Yew](https://github.com/yewstack/yew) in [Rust](https://www.rust-lang.org/).

**Warning:** This is in early development and it is possible it won't be
finished if there is no interest. Don't use this in production! Please help or
leave a star to let me know you are interested in this project.

Installation
------------

## Usage

Check the
[template repository](https://github.com/yewprint/yewprint-template)
to see how things work. **The Blueprint CSS needs to be downloaded from your
build script.**

To easily get started, you can use
[cargo-generate](https://crates.io/crates/cargo-generate)
to get started very quickly:

```
cargo generate --git https://github.com/yewprint/yewprint-template.git
```

## Development Environment

```
cargo xtask start
```

You can now go to http://localhost:8000

### Blueprint CSS update

```
cargo xtask update-css
```

Roadmap
-------

 -  [ ] Doc web-site designed with the library itself (like https://blueprintjs.com) including link to the Yew library
 -  [ ] [FocusStyleManager](https://blueprintjs.com/docs/#core/accessibility.focus-management)
 -  [ ] [Classes & Typography (and bp3- prefix)](https://blueprintjs.com/docs/#core/classes)
     -  [HTML elements](https://blueprintjs.com/docs/#core/components/html)
     -  [HTML table](https://blueprintjs.com/docs/#core/components/html-table)
     -  [Non-ideal state](https://blueprintjs.com/docs/#core/components/non-ideal-state)
     -  [HTML select](https://blueprintjs.com/docs/#core/components/html-select)
     -  [HTML input](https://blueprintjs.com/docs/#core/components/text-inputs.html-input)
     -  [Search field](https://blueprintjs.com/docs/#core/components/text-inputs.search-field)
 -  [ ] [Breadcrumbs](https://blueprintjs.com/docs/#core/components/breadcrumbs)
 -  [x] [Button](https://blueprintjs.com/docs/#core/components/button)
     -  [ ] Complete Button API
     -  [ ] AnchorButton
 -  [x] [ButtonGroup](https://blueprintjs.com/docs/#core/components/button-group)
     -  depends on: Button
 -  [x] [Callout](https://blueprintjs.com/docs/#core/components/callout)
 -  [x] [Card](https://blueprintjs.com/docs/#core/components/card)
 -  [x] [Collapse](https://blueprintjs.com/docs/#core/components/collapse)
 -  [ ] [CollapsibleList](https://blueprintjs.com/docs/#core/components/collapsible-list)
 -  [x] [Divider](https://blueprintjs.com/docs/#core/components/divider)
     -  depends on: ButtonGroup
 -  [ ] [EditableText](https://blueprintjs.com/docs/#core/components/editable-text)
 -  [ ] [Hotkeys](https://blueprintjs.com/docs/#core/components/hotkeys)
 -  [x] [Icon](https://blueprintjs.com/docs/#core/components/icon)
 -  [x] [Menu](https://blueprintjs.com/docs/#core/components/menu)
     -  [ ] Complete Menu API
     -  depends on: Popover
 -  [ ] [Popover](https://blueprintjs.com/docs/#core/components/menu.dropdowns) (dropdowns)
     -  depends on: Overlay
 -  [ ] [Navbar](https://blueprintjs.com/docs/#core/components/navbar)
 -  [ ] [OverflowList](https://blueprintjs.com/docs/#core/components/overflow-list)
 -  [x] [PanelStack](https://blueprintjs.com/docs/#core/components/panel-stack)
 -  [x] [ProgressBar](https://blueprintjs.com/docs/#core/components/progress-bar)
 -  [ ] [ResizeSensor](https://blueprintjs.com/docs/#core/components/resize-sensor)
 -  [ ] [Skeleton](https://blueprintjs.com/docs/#core/components/skeleton)
 -  [x] [Spinner](https://blueprintjs.com/docs/#core/components/spinner)
 -  [x] [Tabs](https://blueprintjs.com/docs/#core/components/tabs)
 -  [x] [Tag](https://blueprintjs.com/docs/#core/components/tag)
     -  depends on: Text
 -  [x] [Text](https://blueprintjs.com/docs/#core/components/text)
 -  [x] [Tree](https://blueprintjs.com/docs/#core/components/tree)
     -  depends on: Collapse, Icon
 -  [ ] [FormGroup](https://blueprintjs.com/docs/#core/components/form-group)
 -  [x] [ControlGroup](https://blueprintjs.com/docs/#core/components/control-group)
 -  [ ] [Label](https://blueprintjs.com/docs/#core/components/label)
 -  [x] [Checkbox](https://blueprintjs.com/docs/#core/components/checkbox)
 -  [x] [RadioGroup](https://blueprintjs.com/docs/#core/components/radio)
 -  [x] [Sliders](https://blueprintjs.com/docs/#core/components/sliders)
 -  [ ] [RangeSlider](https://blueprintjs.com/docs/#core/components/sliders.range-slider)
 -  [ ] [MultiSlider](https://blueprintjs.com/docs/#core/components/sliders.multi-slider)
 -  [x] [Switch](https://blueprintjs.com/docs/#core/components/switch)
     -  [ ] Complete Switch API
 -  [ ] [FileInput](https://blueprintjs.com/docs/#core/components/file-input)
 -  [x] [NumericInput](https://blueprintjs.com/docs/#core/components/numeric-input)
 -  [x] [InputGroup](https://blueprintjs.com/docs/#core/components/text-inputs.input-group)
 -  [x] [TextArea](https://blueprintjs.com/docs/#core/components/text-inputs.text-area)
 -  [ ] [TagInput](https://blueprintjs.com/docs/#core/components/tag-input)
 -  [ ] [Overlay](https://blueprintjs.com/docs/#core/components/overlay)
     -  depends on: Portal
 -  [ ] [Portal](https://blueprintjs.com/docs/#core/components/portal)
 -  [ ] [Alert](https://blueprintjs.com/docs/#core/components/alert)
     -  depends on: Button, Dialog
 -  [ ] [Context menu](https://blueprintjs.com/docs/#core/components/context-menu)
     -  depends on: Popover
 -  [ ] [Dialog](https://blueprintjs.com/docs/#core/components/dialog)
     -  depends on: Icon, Overlay, Button
 -  [ ] [Drawer](https://blueprintjs.com/docs/#core/components/drawer)
     -  depends on: Icon, Overlay, Button
 -  [ ] [Popover](https://blueprintjs.com/docs/#core/components/popover)
     -  depends on: ResizeSensor, Tooltip, Overlay
 -  [ ] [Toast](https://blueprintjs.com/docs/#core/components/toast)
     -  depends on: Button, ButtonGroup, Icon, AnchorButton
 -  [ ] [Tooltip](https://blueprintjs.com/docs/#core/components/tooltip)
     -  depends on: Popover
 -  [ ] [DatePicker](https://blueprintjs.com/docs/#datetime/datepicker)
 -  [ ] [DateRangePicker](https://blueprintjs.com/docs/#datetime/daterangepicker)
 -  [ ] [TimePicker](https://blueprintjs.com/docs/#datetime/timepicker)
 -  [ ] [DateTimePicker](https://blueprintjs.com/docs/#datetime/datetimepicker)
 -  [ ] [DateInput](https://blueprintjs.com/docs/#datetime/dateinput)
 -  [ ] [DateRangeInput](https://blueprintjs.com/docs/#datetime/daterangeinput)
 -  [ ] [Select](https://blueprintjs.com/docs/#select/select-component)
 -  [ ] [Suggest](https://blueprintjs.com/docs/#select/suggest)
 -  [ ] [MultiSelect](https://blueprintjs.com/docs/#select/multi-select)
 -  [ ] [Omnibar](https://blueprintjs.com/docs/#select/omnibar)
 -  [ ] [QueryList](https://blueprintjs.com/docs/#select/query-list)
 -  [ ] [Table](https://blueprintjs.com/docs/#table)
 -  [ ] [TimezonePicker](https://blueprintjs.com/docs/#timezone)
