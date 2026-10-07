//! The sign-in screen.

use egui::{Align, Color32, CornerRadius, Frame, Layout, Margin, Rect, Stroke, Vec2, pos2};

use crate::app::App;
use crate::backend::AuthStatus;
use crate::i18n::gettext;
use crate::model::Action;
use crate::settings::ProxyMode;
use crate::theme;

const SPOTY_ACCENT: Color32 = Color32::from_rgb(0xe0, 0x79, 0x8b);
const SPOTY_ACCENT_HOVER: Color32 = Color32::from_rgb(0xec, 0x91, 0xa0);
const SPOTY_ACCENT_ACTIVE: Color32 = Color32::from_rgb(0xc9, 0x63, 0x76);
const SPOTY_BUTTON_TEXT: Color32 = Color32::from_rgb(0x28, 0x14, 0x19);

pub fn show(app: &mut App, ui: &mut egui::Ui, connecting: bool) {
    let locale = app.locale;
    let ctx = ui.ctx().clone();
    egui::CentralPanel::default()
        .frame(Frame::new().fill(Color32::from_rgb(0x12, 0x10, 0x13)))
        .show(ui, |ui| {
            let rect = ui.max_rect();
            // The native double-click action belongs to the top bar, not the empty login background.
            let drag_rect = if cfg!(target_os = "macos") {
                Rect::from_min_size(
                    rect.min,
                    Vec2::new(rect.width(), theme::TOP_BAR_HEIGHT + theme::titlebar_inset(ui.ctx())),
                )
            } else {
                rect
            };
            super::titlebar_drag(ui, drag_rect);

            // Deep plum-black gradient with a quiet rose tint
            let top_bg = Color32::from_rgb(0x1a, 0x11, 0x16);
            let bot_bg = Color32::from_rgb(0x0f, 0x0d, 0x10);
            super::widgets::paint_vertical_gradient(ui, rect, top_bg, bot_bg);

            let card_width = 460.0;
            let proxy_id = egui::Id::new("login-proxy-open");
            let proxy_open = ui
                .ctx()
                .data(|data| data.get_temp::<bool>(proxy_id))
                .unwrap_or(false);
            let card_height: f32 = if !proxy_open {
                485.0
            } else if app.settings.proxy_mode.is_manual() {
                730.0
            } else {
                550.0
            };
            let card_height = card_height.min((rect.height() - 64.0).max(0.0));
            let card = Rect::from_center_size(
                rect.center() - Vec2::new(0.0, 16.0),
                Vec2::new(card_width, card_height),
            );

            // Soft ambient illumination behind the sign-in card
            let card_center = card.center();
            for (radius, alpha) in [(300.0, 10), (180.0, 16), (90.0, 24)] {
                ui.painter().circle_filled(
                    card_center,
                    radius,
                    Color32::from_rgba_unmultiplied(224, 121, 139, alpha),
                );
            }

            let mut card_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(card)
                    .layout(Layout::top_down(Align::Center)),
            );
            Frame::new()
                .fill(Color32::from_rgba_premultiplied(18, 20, 25, 245))
                .stroke(Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 26)))
                .corner_radius(CornerRadius::same(24))
                .inner_margin(Margin::symmetric(36, 32))
                .shadow(egui::epaint::Shadow {
                    offset: [0, 24],
                    blur: 64,
                    spread: 0,
                    color: Color32::from_black_alpha(200),
                })
                .show(&mut card_ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("login-card-scroll")
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .max_height((card.height() - 64.0).max(0.0))
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.set_width(card_width - 72.0);
                            ui.spacing_mut().item_spacing.y = 8.0;

                            // Elevated Spoty mark in a quiet halo
                            let (logo_rect, _) = ui.allocate_exact_size(Vec2::splat(84.0), egui::Sense::hover());
                            let logo_center = logo_rect.center();
                            ui.painter().circle_filled(
                                logo_center,
                                42.0,
                                Color32::from_rgba_premultiplied(26, 30, 37, 230),
                            );
                            ui.painter().circle_stroke(
                                logo_center,
                                42.0,
                                Stroke::new(
                                    1.0,
                                    Color32::from_rgba_unmultiplied(224, 121, 139, 75),
                                ),
                            );
                            theme::logo(ui, logo_center, 66.0);

                            ui.add_space(8.0);
                            theme::text(ui, "Spoty", theme::bold(32.0), Color32::WHITE);
                            ui.add_space(2.0);
                            theme::text(
                                ui,
                                gettext(locale, "A native Spotify client."),
                                theme::regular(14.0),
                                Color32::from_rgb(156, 163, 175),
                            );
                            ui.add_space(22.0);

                            match &app.auth {
                                AuthStatus::WaitingForBrowser { url } => {
                                    let url = url.clone();
                                    ui.horizontal(|ui| {
                                        ui.add_space((ui.available_width() - 250.0).max(0.0) / 2.0);
                                        theme::spinner(ui, 20.0, SPOTY_ACCENT);
                                        theme::text(
                                            ui,
                                            gettext(locale, "Waiting for Spotify in your browser…"),
                                            theme::medium(14.0),
                                            Color32::WHITE,
                                        );
                                    });
                                    ui.add_space(8.0);
                                    if theme::link(
                                        ui,
                                        gettext(locale, "Didn't open? Open the sign-in page again"),
                                        theme::regular(13.0),
                                        SPOTY_ACCENT,
                                    )
                                    .clicked()
                                    {
                                        ctx.open_url(egui::OpenUrl::new_tab(url));
                                    }
                                    ui.add_space(14.0);
                                    if secondary_button(ui, &gettext(locale, "Cancel")).clicked() {
                                        app.actions.push(Action::CancelSignIn);
                                    }
                                }
                                _ if connecting => {
                                    ui.horizontal(|ui| {
                                        ui.add_space((ui.available_width() - 200.0).max(0.0) / 2.0);
                                        theme::spinner(ui, 20.0, SPOTY_ACCENT);
                                        theme::text(
                                            ui,
                                            gettext(locale, "Connecting to Spotify…"),
                                            theme::medium(14.0),
                                            Color32::WHITE,
                                        );
                                    });
                                }
                                AuthStatus::Failed(message) => {
                                    let message = message.clone();
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(message)
                                                .font(theme::regular(13.0))
                                                .color(Color32::from_rgb(248, 113, 113)),
                                        )
                                        .wrap(),
                                    );
                                    ui.add_space(12.0);
                                    if big_button(ui, app, &gettext(locale, "Try again")) {
                                        app.actions.push(Action::SignIn);
                                    }
                                    if app.settings.web_client_id.is_some() {
                                        ui.add_space(10.0);
                                        if secondary_button(
                                            ui,
                                            &gettext(locale, "Use the shared Spotify app instead"),
                                        )
                                        .clicked()
                                        {
                                            app.settings.web_client_id = None;
                                            app.mark_settings_dirty();
                                            app.actions.push(Action::ConfigurePersonalWebApp);
                                        }
                                    }
                                }
                                _ => {
                                    if big_button(ui, app, &gettext(locale, "Sign in with Spotify")) {
                                        app.actions.push(Action::SignIn);
                                    }
                                    ui.add_space(12.0);
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(gettext(
                                                locale,
                                                "Sign in through your browser. Spoty never sees your password. Local playback needs Spotify Premium.",
                                            ))
                                            .font(theme::regular(12.5))
                                            .color(Color32::from_rgb(148, 154, 166)),
                                        )
                                        .wrap(),
                                    );
                                    if app.settings.web_client_id.is_some() {
                                        ui.add_space(12.0);
                                        if secondary_button(
                                            ui,
                                            &gettext(locale, "Use the shared Spotify app instead"),
                                        )
                                        .clicked()
                                        {
                                            app.settings.web_client_id = None;
                                            app.mark_settings_dirty();
                                            app.actions.push(Action::ConfigurePersonalWebApp);
                                        }
                                    }
                                }
                            }
                            let mut proxy_open = ui.data(|data| data.get_temp::<bool>(proxy_id)).unwrap_or(false);
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                                    if theme::link(
                                        ui,
                                        gettext(locale, "Proxy Settings"),
                                        theme::regular(13.0),
                                        Color32::from_rgb(156, 163, 175),
                                    )
                                    .clicked()
                                    {
                                        proxy_open = !proxy_open;
                                    }
                                });
                            });
                            ui.add_space(6.0);
                            if proxy_open {
                                proxy_fields(ui, app);
                            }
                            ui.data_mut(|data| data.insert_temp(proxy_id, proxy_open));
                        });
                });
            ui.painter().text(
                pos2(rect.center().x, rect.bottom() - 24.0),
                egui::Align2::CENTER_BOTTOM,
                // Translators: {version} is the app's version number, such as 1.2.0.
                gettext(locale, "Spoty {version} • not affiliated with Spotify")
                    .replace("{version}", env!("CARGO_PKG_VERSION")),
                theme::regular(11.5),
                Color32::from_rgb(113, 113, 122),
            );
        });
}

fn secondary_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let galley = ui.painter().layout_no_wrap(
        label.to_string(),
        theme::semibold(13.0),
        Color32::from_rgb(228, 231, 236),
    );
    let padding = Vec2::new(20.0, 9.0);
    let size = Vec2::new(
        (galley.size().x + padding.x * 2.0).min(ui.available_width()),
        galley.size().y + padding.y * 2.0,
    );
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    let hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();
    let fill = if is_down {
        Color32::from_rgba_premultiplied(255, 255, 255, 25)
    } else if hovered {
        Color32::from_rgba_premultiplied(255, 255, 255, 18)
    } else {
        Color32::from_rgba_premultiplied(255, 255, 255, 8)
    };
    let stroke = if hovered {
        Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 60))
    } else {
        Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 25))
    };
    let radius = rect.height() / 2.0;
    ui.painter().rect_filled(rect, radius, fill);
    ui.painter()
        .rect_stroke(rect, radius, stroke, egui::StrokeKind::Inside);
    let text_color = if hovered {
        Color32::WHITE
    } else {
        Color32::from_rgb(220, 224, 230)
    };
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, text_color);
    response
}

fn proxy_fields(ui: &mut egui::Ui, app: &mut App) {
    let palette = app.palette;
    let mut changed = false;
    let mut apply = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let row_width: f32 = ProxyMode::ALL
            .iter()
            .map(|choice| {
                let galley = ui.painter().layout_no_wrap(
                    choice.label(app.locale).into_owned(),
                    theme::medium(13.0),
                    palette.text,
                );
                galley.size().x + 24.0
            })
            .sum::<f32>()
            + 6.0 * (ProxyMode::ALL.len() - 1) as f32;
        ui.add_space((ui.available_width() - row_width).max(0.0) / 2.0);
        for choice in ProxyMode::ALL {
            if theme::soft_button(
                ui,
                &palette,
                None,
                &choice.label(app.locale),
                app.settings.proxy_mode == choice,
            )
            .clicked()
                && app.settings.proxy_mode != choice
            {
                app.settings.proxy_mode = choice;
                changed = true;
                apply = !choice.is_manual();
            }
        }
    });
    if app.settings.proxy_mode.is_manual() {
        ui.add_space(10.0);
        if super::widgets::proxy_manual_form(
            ui,
            &palette,
            app.locale,
            &mut app.settings.proxy_host,
            &mut app.settings.proxy_port,
            &mut app.settings.proxy_username,
            &mut app.settings.proxy_password,
        ) {
            changed = true;
        }
        ui.add_space(6.0);
        super::widgets::proxy_scope_note(ui, &palette, app.locale, app.settings.proxy_mode);
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if theme::pill_button(ui, &palette, &gettext(app.locale, "Apply settings"), true)
                .clicked()
            {
                apply = true;
            }
        });
    }
    if changed {
        app.actions.push(Action::ProxyEdited);
        app.mark_settings_dirty();
    }
    if apply {
        app.actions.push(Action::ApplyProxy);
    }
}

fn big_button(ui: &mut egui::Ui, _app: &App, label: &str) -> bool {
    let galley =
        ui.painter()
            .layout_no_wrap(label.to_string(), theme::bold(15.5), SPOTY_BUTTON_TEXT);
    let size = Vec2::new(ui.available_width().min(320.0), 48.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    let hovered = response.hovered();
    let is_down = response.is_pointer_button_down_on();

    // Subtle tactile ambient glow when hovered
    if hovered {
        ui.painter().rect_filled(
            rect.expand(3.0),
            25.0,
            Color32::from_rgba_unmultiplied(224, 121, 139, 40),
        );
    }

    let fill = if is_down {
        SPOTY_ACCENT_ACTIVE
    } else if hovered {
        SPOTY_ACCENT_HOVER
    } else {
        SPOTY_ACCENT
    };

    ui.painter().rect_filled(rect, 24.0, fill);
    ui.painter().galley(
        rect.center() - galley.size() / 2.0,
        galley,
        SPOTY_BUTTON_TEXT,
    );
    response.clicked()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app::AppOptions, backend::Waker, paths::AppDirs, settings::Settings};

    #[test]
    fn expanded_proxy_settings_remain_reachable_in_a_short_window() {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        theme::install(&ctx);
        let root =
            std::env::temp_dir().join(format!("spoty-proxy-short-login-{}", std::process::id()));
        let mut app = App::new(
            &Waker::default(),
            AppDirs {
                config: root.join("config"),
                state: root.join("state"),
                cache: root.join("cache"),
            },
            Settings {
                proxy_mode: ProxyMode::Http,
                ..Default::default()
            },
            AppOptions {
                media_controls: false,
                restore_sign_in: false,
                tray: false,
            },
        );
        app.auth = AuthStatus::SignedOut;
        ctx.data_mut(|data| data.insert_temp(egui::Id::new("login-proxy-open"), true));
        let screen = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(760.0, 520.0));
        let mut frame = |events| {
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(screen),
                    events,
                    ..Default::default()
                },
                |ui| show(&mut app, ui, false),
            );
            output.textures_delta.clear();
            output.platform_output.accesskit_update.unwrap()
        };
        let visible_button = |tree: &egui::accesskit::TreeUpdate, label: &str| {
            let bounds = tree
                .nodes
                .iter()
                .find(|(_, node)| {
                    node.label() == Some(label) && node.role() == egui::accesskit::Role::Button
                })
                .unwrap_or_else(|| panic!("missing accessible {label}"))
                .1
                .bounds()
                .unwrap();
            bounds.y0 >= 0.0 && bounds.y1 <= 480.0
        };
        let first = frame(vec![]);
        assert!(visible_button(&first, "Sign in with Spotify"));
        let center = |label: &str| {
            let bounds = first
                .nodes
                .iter()
                .find(|(_, node)| node.label() == Some(label))
                .unwrap_or_else(|| panic!("missing accessible {label}"))
                .1
                .bounds()
                .unwrap();
            (bounds.x0 + bounds.x1) / 2.0
        };
        assert!(
            (center("Proxy Settings") - center("Sign in with Spotify")).abs() < 1.0,
            "the proxy link stays centered under sign-in"
        );
        frame(vec![
            egui::Event::PointerMoved(egui::pos2(400.0, 250.0)),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                phase: egui::TouchPhase::Move,
                delta: egui::vec2(0.0, -1000.0),
                modifiers: egui::Modifiers::NONE,
            },
        ]);
        for _ in 0..30 {
            frame(vec![]);
        }
        assert!(
            visible_button(&frame(vec![]), "Apply settings"),
            "scrolling must reach Apply above the footer"
        );
    }
}
