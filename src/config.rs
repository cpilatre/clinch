use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub type ServicesPlan = (String, String);
pub type ServicesMap = Vec<Map>;

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    services_plan: Vec<Plan>,
    services_map: Vec<Map>,
}

#[derive(Debug, Deserialize)]
pub struct Plan {
    plan_name: String,
    plan_id: String,
    bearer: String,
    default: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Map {
    pub number: String,
    pub display: String,
}

impl<'a> Config {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let config: Config = serde_json::from_reader(file)?;
        Ok(config)
    }

    pub fn get_plan(&self, plan_name: &Option<String>) -> Result<ServicesPlan, String> {
        if plan_name.is_none() {
            for plan in &self.services_plan {
                if plan.default {
                    return Ok((plan.plan_id.to_string(), plan.bearer.to_string()));
                }
            }
        } else {
            let plan_name = plan_name.as_ref().unwrap();
            for plan in &self.services_plan {
                if plan.plan_name == *plan_name {
                    return Ok((plan.plan_id.to_string(), plan.bearer.to_string()));
                }
            }
        }
        Err("Service plan unknown".to_string())
    }

    pub fn get_map(&self) -> ServicesMap {
        self.services_map.clone()
    }
}
