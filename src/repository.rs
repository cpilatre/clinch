use crate::config::ServicesPlan;
use std::error::Error;

#[derive(Default)]
pub struct Params {
    pub plan: Option<String>,
    pub page: usize,
    pub page_size: usize,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub find: Option<String>,
}

pub struct SinchRepository {
    services_plan: ServicesPlan,
}

impl SinchRepository {
    pub fn new(services_plan: ServicesPlan) -> Self {
        Self { services_plan }
    }

    pub fn get_data(&self, params: &Params) -> Result<String, Box<dyn Error>> {
        let mut request_builder = reqwest::blocking::Client::new()
            .get(&format!(
                "https://eu.sms.api.sinch.com/xms/v1/{}/inbounds",
                self.services_plan.0
            ))
            .bearer_auth(&self.services_plan.1);

        request_builder =
            request_builder.query(&[("page", params.page), ("page_size", params.page_size)]);
        if let Some(sd) = &params.start_date {
            request_builder = request_builder.query(&[("start_date", &sd)]);
        }
        if let Some(ed) = &params.end_date {
            request_builder = request_builder.query(&[("end_date", &ed)]);
        }

        let result = request_builder.send()?.text()?;

        Ok(result)
    }
}
