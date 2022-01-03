# Patch easy_mark

## easy_mark_viewer.rs

Update use
```
use bevy::prelude::EventWriter;
use bevy_egui::egui::*;
use crate::egui::EasyLinkEvent;
```

Add extra params to `easy_mark, easy_mark_it, item_ui`
```
, link_evts: &mut EventWriter<EasyLinkEvent>
```

change `item_ui`
```
            let label = rich_text_from_style(text, &style);
            ui.add(Hyperlink::from_label_and_url(label, url));
```
to
```
            /*
            let label = rich_text_from_style(text, &style);
            ui.add(Hyperlink::from_label_and_url(label, url));
             */
            crate::egui::EasyLink::new(url, text, style).ui(ui, link_evts);
```

## easy_mark_editor.rs

Update use
```
use bevy::prelude::EventWriter;
use bevy_egui::egui::{text_edit::CCursorRange, *};
use crate::egui::EasyLinkEvent;
```

Add extra params to `ui`
```
, link_evts: &mut EventWriter<EasyLinkEvent>
```

comment out the `epi::Api` related lines

## mode.rs

Make all the modules public