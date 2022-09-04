# Notes

- Based on egui_demo_lib inside egui source repo.
- Only add easy_mark_parser.rs and easy_mark_viewer.rs

# Changes

## mod.rs

- Make the modules public

## easy_mark_viewer.rs

- Make rich_text_from_style() public

### Support link event

- add imports

```
use bevy::prelude::EventWriter;
use crate::egui::EasyLinkEvent;
```

- add `, link_evts: &mut EventWriter<EasyLinkEvent>` to easy_mark(), easy_mark_it(), and item_ui()

- Use `EasyLink` in item_ui()

```
        easy_mark::Item::Hyperlink(style, text, url) => {
            /*
            let label = rich_text_from_style(text, &style);
            ui.add(Hyperlink::from_label_and_url(label, url));
             */
            crate::egui::EasyLink::new(url, text, style).ui(ui, link_evts);
        }
```