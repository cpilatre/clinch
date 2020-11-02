use crate::config::ServicesMap;
use crate::domain::{Message, SinchData};
use chrono::Utc;
use chrono::{DateTime, Local};
use console::{style, truncate_str, Term};
use std::error::Error;
use std::str::FromStr;

pub struct Display {
    term: Term,
    services_map: ServicesMap,
}

impl<'a> Display {
    pub fn init(term: Term, services_map: ServicesMap) -> Self {
        Self { term, services_map }
    }

    pub fn display_head(&self, sinch_data: &SinchData) -> Result<(), Box<dyn Error>> {
        let head = format!(
            "Total messages: {} - Page ({}): {}/{}",
            style(sinch_data.count).red().bold(),
            style(sinch_data.page_size),
            style(sinch_data.page).cyan().bold(),
            style(sinch_data.count / sinch_data.page_size + 1)
                .cyan()
                .bold(),
        );
        self.term.write_line(&head)?;
        Ok(())
    }

    pub fn display_messages(
        &self,
        messages: &[Message],
        display_sms: bool,
        utc_time_zone: bool,
    ) -> Result<(), Box<dyn Error>> {
        let (_, columns) = self.term.size();

        for msg in messages {
            let dt = if utc_time_zone {
                DateTime::<Utc>::from_str(&msg.received_at)?.naive_utc()
            } else {
                DateTime::<Local>::from_str(&msg.received_at)?.naive_local()
            };

            let details = if display_sms {
                msg.body.to_string()
            } else {
                format!("({})", &msg.r#type).to_string()
            };

            let format = format!(
                "{} \u{260E}  {} \u{21DD} {} {} {}",
                style(&msg.id).italic(),
                style(&msg.from),
                style(self.map_service(&msg.to)),
                style(dt.format("%b. %d %X")),
                style(&details),
            );
            let line = truncate_str(&format, columns as usize, "...");
            self.term.write_line(&line)?;
        }

        Ok(())
    }

    pub fn map_service(&'a self, number: &'a str) -> &'a str {
        for map in &self.services_map {
            if map.number == *number {
                return &map.display;
            }
        }
        number
    }
}
