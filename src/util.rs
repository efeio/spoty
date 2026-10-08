//! Small helpers shared across the application.

use std::borrow::Cow;

use crate::i18n::{Locale, gettext, ngettext, pgettext};

/// `3:45` for track lengths, `1:02:03` past an hour.
pub fn format_duration_ms(ms: u32) -> String {
    let total = ms / 1000;
    let hours = total / 3600;
    let minutes = (total / 60) % 60;
    let seconds = total % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

/// `2 hr 13 min` for playlist totals, `45 min 12 sec` under an hour.
pub fn format_total_ms(locale: Locale, ms: u64) -> String {
    let total = ms / 1000;
    let hours = total / 3600;
    let minutes = (total / 60) % 60;
    let seconds = total % 60;
    if hours > 0 {
        // Translators: A length of time, abbreviated. Keep {hours} and {minutes}.
        gettext(locale, "{hours} hr {minutes} min")
            .replace("{hours}", &hours.to_string())
            .replace("{minutes}", &minutes.to_string())
    } else if minutes > 0 {
        // Translators: A length of time, abbreviated. Keep {minutes} and {seconds}.
        gettext(locale, "{minutes} min {seconds} sec")
            .replace("{minutes}", &minutes.to_string())
            .replace("{seconds}", &seconds.to_string())
    } else {
        // Translators: A length of time, abbreviated. Keep {seconds}.
        gettext(locale, "{seconds} sec").replace("{seconds}", &seconds.to_string())
    }
}

/// Episode lengths read as `1 hr 12 min` or `38 min`.
pub fn format_episode_ms(locale: Locale, ms: u32) -> String {
    let minutes = ms / 60_000;
    let hours = minutes / 60;
    if hours > 0 {
        gettext(locale, "{hours} hr {minutes} min")
            .replace("{hours}", &hours.to_string())
            .replace("{minutes}", &(minutes % 60).to_string())
    } else {
        // Translators: A length of time, abbreviated. Keep {minutes}.
        gettext(locale, "{minutes} min").replace("{minutes}", &minutes.max(1).to_string())
    }
}

pub fn format_count(count: u64) -> String {
    let digits = count.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(',');
        }
        out.push(character);
    }
    out
}

/// `Jan 5, 2024` from an ISO-8601 timestamp or a bare date.
pub fn format_date(locale: Locale, iso: &str) -> String {
    let date = iso.get(..10).unwrap_or(iso);
    let mut parts = date.split('-');
    let (Some(year), Some(month)) = (parts.next(), parts.next()) else {
        return iso.to_string();
    };
    let day = parts.next();
    let month_name = match month {
        "01" => pgettext(locale, "month", "Jan"),
        "02" => pgettext(locale, "month", "Feb"),
        "03" => pgettext(locale, "month", "Mar"),
        "04" => pgettext(locale, "month", "Apr"),
        "05" => pgettext(locale, "month", "May"),
        "06" => pgettext(locale, "month", "Jun"),
        "07" => pgettext(locale, "month", "Jul"),
        "08" => pgettext(locale, "month", "Aug"),
        "09" => pgettext(locale, "month", "Sep"),
        "10" => pgettext(locale, "month", "Oct"),
        "11" => pgettext(locale, "month", "Nov"),
        "12" => pgettext(locale, "month", "Dec"),
        _ => return iso.to_string(),
    };
    let dated = match day.and_then(|day| day.trim_start_matches('0').parse::<u8>().ok()) {
        Some(day) => {
            // Translators: A date. {month} is an abbreviated month name; reorder as your language writes dates.
            gettext(locale, "{month} {day}, {year}").replace("{day}", &day.to_string())
        }
        // Translators: A month and year. {month} is an abbreviated month name.
        None => gettext(locale, "{month} {year}").into_owned(),
    };
    dated
        .replace("{month}", &month_name)
        .replace("{year}", year)
}

/// `5 minutes ago` for recent ISO-8601 timestamps, otherwise the usual date.
///
/// Dates are shown relatively for their first 30 days, matching the playlist
/// table's compact, time-aware presentation. `now` is an argument so callers
/// can render against one instant and the boundary behaviour stays testable.
pub fn format_relative_date(locale: Locale, iso: &str, now: jiff::Timestamp) -> String {
    let Ok(added) = iso.parse::<jiff::Timestamp>() else {
        return format_date(locale, iso);
    };
    let seconds = added.duration_until(now).as_secs_f64().floor() as i64;
    if !(0..30 * 24 * 60 * 60).contains(&seconds) {
        return format_date(locale, iso);
    }

    let (count, text) = if seconds < 60 {
        let count = seconds;
        let text = ngettext(
            locale,
            // Translators: How long ago a song was added. Keep {count}.
            "{count} second ago",
            "{count} seconds ago",
            count as u32,
        );
        (count, text)
    } else if seconds < 60 * 60 {
        let count = seconds / 60;
        let text = ngettext(
            locale,
            // Translators: How long ago a song was added. Keep {count}.
            "{count} minute ago",
            "{count} minutes ago",
            count as u32,
        );
        (count, text)
    } else if seconds < 24 * 60 * 60 {
        let count = seconds / (60 * 60);
        let text = ngettext(
            locale,
            // Translators: How long ago a song was added. Keep {count}.
            "{count} hour ago",
            "{count} hours ago",
            count as u32,
        );
        (count, text)
    } else if seconds < 7 * 24 * 60 * 60 {
        let count = seconds / (24 * 60 * 60);
        // Translators: How long ago a song was added. Keep {count}.
        let text = ngettext(locale, "{count} day ago", "{count} days ago", count as u32);
        (count, text)
    } else {
        let count = seconds / (7 * 24 * 60 * 60);
        let text = ngettext(
            locale,
            // Translators: How long ago a song was added. Keep {count}.
            "{count} week ago",
            "{count} weeks ago",
            count as u32,
        );
        (count, text)
    };
    text.replace("{count}", &count.to_string())
}

/// Tears the id out of `spotify:track:abc` and friends.
pub fn uri_id(uri: &str) -> Option<&str> {
    uri.rsplit(':').next().filter(|id| !id.is_empty())
}

pub fn uri_kind(uri: &str) -> Option<&str> {
    let mut parts = uri.split(':');
    parts.next()?;
    parts.next()
}

/// Spotify's radio station seeded by a song, playlist, album, or artist.
pub fn station_uri(seed: &str) -> Option<String> {
    let kind = uri_kind(seed)?;
    let id = uri_id(seed)?;
    (seed == format!("spotify:{kind}:{id}")
        && matches!(kind, "track" | "playlist" | "album" | "artist"))
    .then(|| format!("spotify:station:{kind}:{id}"))
}

/// The song, playlist, album, or artist a radio station is seeded by.
pub fn station_seed(station: &str) -> Option<String> {
    let seed = format!("spotify:{}", station.strip_prefix("spotify:station:")?);
    station_uri(&seed).is_some().then_some(seed)
}

pub fn open_spotify_url(uri: &str) -> Option<String> {
    let kind = uri_kind(uri)?;
    let id = uri_id(uri)?;
    Some(format!("https://open.spotify.com/{kind}/{id}"))
}

/// The Spoty monogram for macOS. Template images use alpha only, letting
/// macOS paint the mark for the current menu-bar appearance.
pub fn tray_template_rgba(size: usize) -> Vec<u8> {
    let scale = size as f32 / 128.0;
    let mut rgba = vec![0u8; size * size * 4];
    for y in 0..size {
        for x in 0..size {
            let u = (x as f32 + 0.5) / scale;
            let v = (y as f32 + 0.5) / scale;
            let coverage = polygon_coverage((u, v), scale);
            let index = (y * size + x) * 4;
            rgba[index..index + 3].fill(0);
            rgba[index + 3] = (coverage * 255.0).round() as u8;
        }
    }
    rgba
}

/// Spoty's original monogram: the window icon, trays and in-app logo share it.
pub fn app_icon_rgba(size: usize) -> Vec<u8> {
    const BACK_TOP: [f32; 3] = [63.0, 53.0, 65.0];
    const BACK_BOTTOM: [f32; 3] = [37.0, 32.0, 40.0];
    const ROSE: [f32; 3] = [215.0, 168.0, 181.0];
    let scale = size as f32 / 128.0;
    let mut rgba = vec![0u8; size * size * 4];
    for y in 0..size {
        for x in 0..size {
            let u = (x as f32 + 0.5) / scale;
            let v = (y as f32 + 0.5) / scale;
            let rectangle = rounded_rect_coverage((u, v), scale);
            let glyph = polygon_coverage((u, v), scale);
            if rectangle == 0.0 {
                continue;
            }
            let mut colour = mix(BACK_TOP, BACK_BOTTOM, (v - 4.0) / 120.0);
            colour = mix(colour, ROSE, glyph);
            let index = (y * size + x) * 4;
            rgba[index] = colour[0].round() as u8;
            rgba[index + 1] = colour[1].round() as u8;
            rgba[index + 2] = colour[2].round() as u8;
            rgba[index + 3] = (rectangle * 255.0).round() as u8;
        }
    }
    rgba
}

/// Mixes two colours, `t` of the way from `a` to `b`.
fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

const SPOTY_GLYPH: &[(f32, f32)] = &[
    (82.0, 42.0),
    (78.0, 53.0),
    (68.0, 49.0),
    (60.0, 49.0),
    (55.0, 52.0),
    (55.0, 57.0),
    (58.0, 60.0),
    (72.0, 63.0),
    (80.0, 66.0),
    (85.0, 71.0),
    (87.0, 77.0),
    (85.0, 84.0),
    (80.0, 90.0),
    (73.0, 94.0),
    (65.0, 96.0),
    (57.0, 95.0),
    (49.0, 92.0),
    (42.0, 88.0),
    (49.0, 78.0),
    (56.0, 82.0),
    (63.0, 85.0),
    (70.0, 84.0),
    (74.0, 82.0),
    (75.0, 79.0),
    (72.0, 77.0),
    (68.0, 76.0),
    (58.0, 73.0),
    (51.0, 70.0),
    (47.0, 65.0),
    (45.0, 59.0),
    (47.0, 53.0),
    (52.0, 47.0),
    (59.0, 43.0),
    (67.0, 41.0),
    (76.0, 41.0),
];

fn polygon_coverage(point: (f32, f32), scale: f32) -> f32 {
    let mut inside = false;
    let mut distance_squared = f32::INFINITY;
    for index in 0..SPOTY_GLYPH.len() {
        let a = SPOTY_GLYPH[index];
        let b = SPOTY_GLYPH[(index + 1) % SPOTY_GLYPH.len()];
        let (ex, ey) = (b.0 - a.0, b.1 - a.1);
        let (px, py) = (point.0 - a.0, point.1 - a.1);
        let along = ((px * ex + py * ey) / (ex * ex + ey * ey)).clamp(0.0, 1.0);
        let (dx, dy) = (px - ex * along, py - ey * along);
        distance_squared = distance_squared.min(dx * dx + dy * dy);
        if (a.1 > point.1) != (b.1 > point.1)
            && point.0 < (b.0 - a.0) * (point.1 - a.1) / (b.1 - a.1) + a.0
        {
            inside = !inside;
        }
    }
    let distance = distance_squared.sqrt() * scale;
    let signed = if inside { distance } else { -distance };
    (signed + 0.5).clamp(0.0, 1.0)
}

fn rounded_rect_coverage(point: (f32, f32), scale: f32) -> f32 {
    let qx = (point.0 - 64.0).abs() - 26.0;
    let qy = (point.1 - 64.0).abs() - 26.0;
    let outside = qx.max(0.0).hypot(qy.max(0.0));
    let distance = outside + qx.max(qy).min(0.0) - 34.0;
    (0.5 - distance * scale).clamp(0.0, 1.0)
}

pub fn greeting(locale: Locale) -> Cow<'static, str> {
    match local_hour() {
        5..=11 => gettext(locale, "Good morning"),
        12..=17 => gettext(locale, "Good afternoon"),
        _ => gettext(locale, "Good evening"),
    }
}

fn local_hour() -> u8 {
    jiff::Zoned::now().hour() as u8
}

/// Strips the HTML Spotify embeds in playlist descriptions.
pub fn strip_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_tag = false;
    for character in text.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(character),
            _ => {}
        }
    }
    out.replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&#x2F;", "/")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

/// Atomically replaces `path` with `temporary` on the current platform.
#[cfg(not(windows))]
pub(crate) fn replace_file(
    temporary: &std::path::Path,
    path: &std::path::Path,
) -> std::io::Result<()> {
    std::fs::rename(temporary, path)
}

/// Atomically replaces `path` with `temporary` on Windows.
#[cfg(windows)]
pub(crate) fn replace_file(
    temporary: &std::path::Path,
    path: &std::path::Path,
) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    let temporary: Vec<u16> = temporary.as_os_str().encode_wide().chain(Some(0)).collect();
    let path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let moved = unsafe {
        MoveFileExW(
            temporary.as_ptr(),
            path.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if moved == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixel(rgba: &[u8], size: usize, x: usize, y: usize) -> [u8; 4] {
        let index = (y * size + x) * 4;
        [
            rgba[index],
            rgba[index + 1],
            rgba[index + 2],
            rgba[index + 3],
        ]
    }

    /// App icons carry the Spoty tile and monogram. Menu-bar images use only
    /// the monogram silhouette so the operating system can theme its colour.
    #[test]
    fn the_icon_and_menu_bar_use_the_spoty_monogram_at_every_size() {
        // #given the icon at a dock size and at a tray size
        let (large, small) = (app_icon_rgba(128), app_icon_rgba(32));

        // #then the tile has a transparent rounded corner and a rose monogram
        assert_eq!(pixel(&large, 128, 1, 1)[3], 0);
        let monogram = pixel(&large, 128, 66, 45);
        assert!(monogram[0] > monogram[2] && monogram[2] > monogram[1]);
        assert!(pixel(&small, 32, 16, 12)[0] > pixel(&small, 32, 16, 12)[1]);

        // #and the menu-bar template is the S silhouette on transparency
        let template = tray_template_rgba(44);
        assert_eq!(pixel(&template, 44, 2, 2)[3], 0);
        assert_eq!(pixel(&template, 44, 20, 15)[3], 255);
    }

    #[test]
    fn radio_stations_map_to_their_seeds_and_back() {
        for kind in ["track", "playlist", "album", "artist"] {
            let seed = format!("spotify:{kind}:4uLU6hMCjMI75M1A2tKUQC");
            let station = station_uri(&seed).expect("a station");
            assert_eq!(
                station,
                format!("spotify:station:{kind}:4uLU6hMCjMI75M1A2tKUQC")
            );
            assert_eq!(station_seed(&station).as_deref(), Some(seed.as_str()));
        }
        for seed in [
            "spotify:show:abc",
            "spotify:episode:abc",
            "spotify:user:me:collection",
            "spotify:track:",
            "track:abc",
        ] {
            assert_eq!(station_uri(seed), None, "{seed}");
        }
        assert_eq!(station_seed("spotify:playlist:abc"), None);
    }

    #[test]
    fn durations() {
        assert_eq!(format_duration_ms(225_000), "3:45");
        assert_eq!(format_duration_ms(3_723_000), "1:02:03");
        assert_eq!(format_total_ms(Locale::English, 7_980_000), "2 hr 13 min");
        assert_eq!(format_total_ms(Locale::English, 2_712_000), "45 min 12 sec");
        assert_eq!(format_episode_ms(Locale::English, 4_320_000), "1 hr 12 min");
    }

    #[test]
    fn counts_and_dates() {
        assert_eq!(format_count(1_234_567), "1,234,567");
        assert_eq!(format_count(12), "12");
        assert_eq!(
            format_date(Locale::English, "2024-01-05T10:00:00Z"),
            "Jan 5, 2024"
        );
        assert_eq!(format_date(Locale::English, "2024-03"), "Mar 2024");
        assert_eq!(format_date(Locale::English, "2024"), "2024");
    }

    #[test]
    fn recent_dates_are_relative_for_the_first_month() {
        let now: jiff::Timestamp = "2026-08-31T12:00:00Z".parse().unwrap();
        assert_eq!(
            format_relative_date(Locale::English, "2026-08-31T11:59:30Z", now),
            "30 seconds ago"
        );
        assert_eq!(
            format_relative_date(Locale::English, "2026-08-31T11:59:00Z", now),
            "1 minute ago"
        );
        assert_eq!(
            format_relative_date(Locale::English, "2026-08-31T11:00:00Z", now),
            "1 hour ago"
        );
        assert_eq!(
            format_relative_date(Locale::English, "2026-08-30T12:00:00Z", now),
            "1 day ago"
        );
        assert_eq!(
            format_relative_date(Locale::English, "2026-08-17T12:00:00Z", now),
            "2 weeks ago"
        );
        assert_eq!(
            format_relative_date(Locale::English, "2026-08-01T12:00:00Z", now),
            "Aug 1, 2026"
        );
    }

    #[test]
    fn dates_and_lengths_follow_the_interface_language() {
        let now: jiff::Timestamp = "2026-08-31T12:00:00Z".parse().unwrap();
        assert_eq!(format_date(Locale::Spanish, "2024-01-05"), "5 ene 2024");
        assert_eq!(format_date(Locale::Spanish, "2024-09"), "sept 2024");
        assert_eq!(format_total_ms(Locale::Spanish, 7_980_000), "2 h 13 min");
        assert_eq!(format_episode_ms(Locale::Spanish, 2_280_000), "38 min");
        for (added, expected) in [
            ("2026-08-31T11:59:59Z", "hace 1 segundo"),
            ("2026-08-31T11:58:00Z", "hace 2 minutos"),
            ("2026-08-30T12:00:00Z", "hace 1 día"),
            ("2026-08-17T12:00:00Z", "hace 2 semanas"),
        ] {
            assert_eq!(format_relative_date(Locale::Spanish, added, now), expected);
        }
        // Each language orders the date its own way.
        assert_eq!(format_date(Locale::Swedish, "2024-01-05"), "5 jan. 2024");
        assert_eq!(format_date(Locale::English, "2024-01-05"), "Jan 5, 2024");
        assert_eq!(
            format_relative_date(Locale::Swedish, "2026-08-30T12:00:00Z", now),
            "för 1 dag sedan"
        );
        assert_eq!(format_date(Locale::Turkish, "2024-01-05"), "5 Oca 2024");
        assert_eq!(format_date(Locale::Turkish, "2024-09"), "Eyl 2024");
        assert_eq!(format_total_ms(Locale::Turkish, 7_980_000), "2 sa 13 dk");
        assert_eq!(format_episode_ms(Locale::Turkish, 2_280_000), "38 dk");
        for (added, expected) in [
            ("2026-08-31T11:59:59Z", "1 saniye önce"),
            ("2026-08-31T11:58:00Z", "2 dakika önce"),
            ("2026-08-30T12:00:00Z", "1 gün önce"),
            ("2026-08-17T12:00:00Z", "2 hafta önce"),
        ] {
            assert_eq!(format_relative_date(Locale::Turkish, added, now), expected);
        }
    }

    #[test]
    fn relative_dates_fall_back_for_future_and_invalid_timestamps() {
        let now: jiff::Timestamp = "2026-08-31T12:00:00Z".parse().unwrap();
        assert_eq!(
            format_relative_date(Locale::English, "2026-09-01T12:00:00Z", now),
            "Sep 1, 2026"
        );
        assert_eq!(
            format_relative_date(Locale::English, "not-a-date", now),
            "not-a-date"
        );
    }

    #[test]
    fn uris() {
        assert_eq!(uri_id("spotify:track:abc"), Some("abc"));
        assert_eq!(uri_kind("spotify:playlist:x"), Some("playlist"));
        assert_eq!(
            open_spotify_url("spotify:album:z").as_deref(),
            Some("https://open.spotify.com/album/z")
        );
    }

    #[test]
    fn html_is_stripped() {
        assert_eq!(
            strip_html("Hi <a href=\"x\">there</a> &amp; you"),
            "Hi there & you"
        );
        assert_eq!(strip_html("ONE&#x2F;TWO&#x2F;THREE"), "ONE/TWO/THREE");
    }
}
