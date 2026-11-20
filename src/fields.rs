// Author: David M. Anderson
// Built with AI assistance (Claude, Anthropic)

use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
use eframe::egui::{Button, DragValue, Grid, Label, Popup, PopupCloseBehavior, RichText, Ui, vec2};

use crate::civil::{Clock, week_start};

pub fn date(ui: &mut Ui, id: &str, date: &mut NaiveDate, format: &str, first: Weekday) {
    ui.push_id(id, |ui| {
        let button = ui.button(date.format(format).to_string());
        let month_id = button.id.with("month");
        if button.clicked() {
            ui.data_mut(|data| data.insert_temp(month_id, first_of_month(*date)));
        }
        Popup::from_toggle_button_response(&button)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                let mut month = ui
                    .data(|data| data.get_temp(month_id))
                    .unwrap_or_else(|| first_of_month(*date));
                ui.horizontal(|ui| {
                    if ui.button("‹").on_hover_text("Previous month").clicked() {
                        month = month - Months::new(1);
                    }
                    ui.add_sized(
                        vec2(140.0, ui.spacing().interact_size.y),
                        Label::new(month.format("%B %Y").to_string()),
                    );
                    if ui.button("›").on_hover_text("Next month").clicked() {
                        month = month + Months::new(1);
                    }
                });
                ui.data_mut(|data| data.insert_temp(month_id, month));
                if let Some(picked) = month_grid(ui, month, *date, first) {
                    *date = picked;
                    ui.close();
                }
            });
    });
}

fn first_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap_or(date)
}

fn month_grid(
    ui: &mut Ui,
    month: NaiveDate,
    selected: NaiveDate,
    first: Weekday,
) -> Option<NaiveDate> {
    let mut picked = None;
    let cell = vec2(28.0, ui.spacing().interact_size.y);
    Grid::new("days").spacing(vec2(2.0, 2.0)).show(ui, |ui| {
        let mut weekday = first;
        for _ in 0..7 {
            let name = weekday.to_string();
            ui.add_sized(cell, Label::new(RichText::new(&name[..2]).weak()));
            weekday = weekday.succ();
        }
        ui.end_row();
        let mut day = week_start(month, first);
        while day <= month + Months::new(1) - Days::new(1) {
            for _ in 0..7 {
                let mut text = RichText::new(day.day().to_string());
                if day.month() != month.month() {
                    text = text.weak();
                }
                if ui
                    .add_sized(cell, Button::new(text).selected(day == selected))
                    .clicked()
                {
                    picked = Some(day);
                }
                day = day.succ_opt().unwrap_or(day);
            }
            ui.end_row();
        }
    });
    picked
}

pub fn time(ui: &mut Ui, clock: Clock, hour: &mut u32, minute: &mut u32) -> bool {
    let mut changed = false;
    match clock {
        Clock::TwentyFourHour => {
            changed |= ui
                .add(
                    DragValue::new(hour)
                        .range(0..=23)
                        .custom_formatter(|n, _| format!("{n:02}")),
                )
                .changed();
        }
        Clock::TwelveHour => {
            let mut shown = (*hour + 11) % 12 + 1;
            if ui.add(DragValue::new(&mut shown).range(1..=12)).changed() {
                *hour = shown % 12 + if *hour >= 12 { 12 } else { 0 };
                changed = true;
            }
        }
    }
    ui.label(":");
    changed |= ui
        .add(
            DragValue::new(minute)
                .range(0..=59)
                .custom_formatter(|n, _| format!("{n:02}")),
        )
        .changed();
    if clock == Clock::TwelveHour {
        let meridiem = if *hour >= 12 { "PM" } else { "AM" };
        if ui
            .add(Button::new(meridiem))
            .on_hover_text("Switch AM and PM")
            .clicked()
        {
            *hour = (*hour + 12) % 24;
            changed = true;
        }
    }
    changed
}
