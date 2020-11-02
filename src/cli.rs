use std::error::Error;
use std::thread;

use crate::{
    config, 
    repository,
    display::Display,
    domain::{expand_messages, Message, Params, Sinch},
    Opt,
};

pub struct Cli {
    pub sinch: Sinch,
    pub opt: Opt,
    pub display: Display,
}

impl Cli {
    pub fn new(sinch: Sinch, opt: Opt, display: Display) -> Self {
        Self {
            sinch,
            opt,
            display,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error + '_>> {
        if self.opt.count {
            return self.count_message();
        }
        if self.opt.load_messages {
            return self.list_messages();
        }
        if self.opt.find.is_some() {
            return self.find_number();
        }

        Ok(())
    }

    fn count_message(&self) -> Result<(), Box<dyn Error>> {
        let sinch_data = self.sinch.get_data(&self.forge_params())?;
        self.display.display_head(&sinch_data)
    }

    fn list_messages(&self) -> Result<(), Box<dyn Error>> {
        let mut sinch_data = self.sinch.get_data(&self.forge_params())?;

        if self.opt.show_sms {
            sinch_data.inbounds = expand_messages(sinch_data.inbounds);
        }
        self.display.display_head(&sinch_data)?;
        self.display
            .display_messages(&sinch_data.inbounds, self.opt.show_sms, self.opt.utc)
    }

    fn find_number(&self) -> Result<(), Box<dyn Error + '_>> {
        let (mut page, mut active_threads) = (0_usize, 0_usize);
        let temp_number = self.opt.find.as_ref().unwrap();
        let max_threads = num_cpus::get();
        let mut results = vec![];

        let mut params = self.forge_params();
        params.page = page;
        let sinch_data = self.sinch.get_data(&params)?;
        self.display.display_head(&sinch_data)?;

        let number_of_pages = (sinch_data.count / sinch_data.page_size) + 1;

        while page < number_of_pages {
            while (active_threads < max_threads) && (page < number_of_pages) {
                let mut children = Vec::with_capacity(max_threads);
                let mut params = self.forge_params();
                params.page = page;

                let opt = self.opt.clone();
                let number = temp_number.clone();

                children.push(thread::spawn(move || {
                    let config = config::Config::from(&opt.config).unwrap();
                    let services_plan = config.get_plan(&opt.plan).unwrap();
                    let repository = repository::SinchRepository::new(services_plan);
                    Sinch::new(repository)
                        .get_data(&params)
                        .unwrap()
                        .inbounds
                        .into_iter()
                        .filter(|msg| msg.from == *number)
                        .collect::<Vec<Message>>()
                }));

                for child in children {
                    results.append(&mut child.join().unwrap());
                }

                page += 1;
                active_threads += 1;
            }
            active_threads = 0;
        }

        if self.opt.show_sms {
            results = expand_messages(results);
        }
        let _ = self
            .display
            .display_messages(&results, self.opt.show_sms, self.opt.utc);

        Ok(())
    }

    fn forge_params(&self) -> Params {
        Params {
            plan: self.opt.plan.clone(),
            page: self.opt.page,
            page_size: self.opt.page_size,
            start_date: self.opt.start_date.clone(),
            end_date: self.opt.end_date.clone(),
            find: self.opt.find.clone(),
        }
    }
}
