pub use crate::repository::Params;
use crate::repository::SinchRepository;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct Message {
    pub r#type: String,
    pub id: String,
    pub from: String,
    pub to: String,
    pub body: String,
    pub operator_id: Option<String>,
    pub sent_at: Option<String>,
    pub received_at: String,
}

#[derive(Default, Deserialize, Debug)]
pub struct SinchData {
    pub count: usize,
    pub page: usize,
    pub inbounds: Vec<Message>,
    pub page_size: usize,
}

pub struct Sinch {
    repository: SinchRepository,
}

impl Sinch {
    pub fn new(repository: SinchRepository) -> Self {
        Self { repository }
    }

    pub fn get_data(&self, params: &Params) -> Result<SinchData, serde_json::Error> {
        let raw_data = self.repository.get_data(params).unwrap();
        serde_json::from_str(&raw_data)
    }
}

pub fn expand_messages(messages: Vec<Message>) -> Vec<Message> {
    messages
        .into_iter()
        .map(|mut msg| {
            if msg.r#type == "mo_binary" {
                let bin_sms = base64::decode(&msg.body).unwrap_or_default();
                let raw_sms = decode_7to8(&bin_sms);

                let body = std::str::from_utf8(&raw_sms)
                    .unwrap_or_default()
                    .to_string();
                msg.body = body;
            }
            msg
        })
        .collect()
}

fn decode_7to8(raw_bytes: &[u8]) -> Vec<u8> {
    let (mut bits_len, mut bits) = (0_u8, 0_u8);
    let mut out = Vec::<u8>::with_capacity(raw_bytes.len() << 1);

    for byte in raw_bytes {
        out.push(((byte << bits_len) | bits) & 0x7F);
        bits = byte >> (7 - bits_len);
        bits_len += 1;

        if bits_len == 7 {
            out.push(bits);
            bits = 0;
            bits_len = 0;
        }
    }

    out
}
