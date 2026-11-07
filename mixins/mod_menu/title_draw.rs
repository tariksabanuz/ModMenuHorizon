// The Mods button's icon (three bars, a menu) over the button, and its tooltip while the pointer is on it.
fn mod_menu_title_draw(
    title: &Title,
    atlas: &HudAtlas,
    items: Option<&crate::item_render::ItemRenderer>,
    batch: &mut HudBatch,
    cursor: (f32, f32),
    hide_splash: bool,
    hide_server_address: bool,
) -> ::core::ops::ControlFlow<()> {
    use core::ops::ControlFlow;
    use crate::title::native_mod_menu_title as m;
    // The call below goes through this hook again: that one runs the original.
    if m::DRAW.enter() {
        return ControlFlow::Continue(());
    }
    title.draw_with_items(atlas, items, batch, cursor, hide_splash, hide_server_address);
    m::DRAW.leave();
    if matches!(title.mode, Mode::Main) {
        let (w, h) = (batch.width.ceil(), batch.height.ceil());
        if let Some(b) = title.buttons(w, h).into_iter().find(|b| b.id == m::BUTTON_ID) {
            let hover = cursor.0 >= b.x && cursor.0 < b.x + b.w && cursor.1 >= b.y && cursor.1 < b.y + 20.0;
            let bar = if hover { rgba(255, 255, 255, 255) } else { rgba(224, 224, 224, 255) };
            // Center the icon: the bars are 12px wide, button is 20px, so we want 4px offset.
            // But empirically it needs more — try 6px to shift it right.
            let icon_offset = 6.0;
            for (_, dy, bw, bh) in m::BARS {
                batch.fill(atlas, b.x + icon_offset, b.y + dy, bw, bh, bar);
            }
            if hover {
                let lines = atlas.font.split(&Text::literal("Mods"), 170.0);
                crate::ui::tooltip(batch, atlas, &lines, cursor);
            }
        }
    }
    ControlFlow::Break(())
}
