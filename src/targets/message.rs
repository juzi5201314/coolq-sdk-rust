use std::fmt::Display;

use serde::export::fmt::Error;
use serde::export::Formatter;

use crate::api::{delete_msg, Convert, Result};
use crate::targets::cqcode;
use crate::targets::cqcode::CQCode;

#[derive(Debug, Default, Clone)]
pub struct Message {
    pub msg: String,
    msg_id: i32,
    pub raw_msg: String,
    pub cqcodes: Vec<CQCode>,
}

impl Message {
    pub fn new(msg: impl Into<Convert<String>>, msg_id: i32) -> Self {
        let msg = msg.into().to_string();
        Message {
            msg: Self::escape(cqcode::clean(msg.as_ref())),
            cqcodes: cqcode::parse(msg.as_ref()),
            raw_msg: msg,
            msg_id,
        }
    }

    pub fn delete(&self) -> bool {
        delete_msg(self.msg_id).is_ok()
    }

    pub fn has_cqcode(&self) -> bool {
        self.cqcodes.iter().count() != 0
    }

    //将因为防止与cq码混淆而转义的字符还原
    fn escape(s: String) -> String {
        s.replace("&amp;", "&")
            .replace("&#91;", "[")
            .replace("&#93;", "]")
            .replace("&#44;", ",")
    }
}

pub struct MessageSegment(String);

impl MessageSegment {
    pub fn new() -> Self {
        MessageSegment(String::new())
    }

    pub fn add(&mut self, msg: impl ToString) -> &mut Self {
        self.0.push_str(msg.to_string().as_ref());
        self
    }
}

impl Display for MessageSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

pub(crate) trait SendMessage {

    /// `@return` msg id
    fn send_message(&self, msg: impl ToString) -> Result<i32> {
        match self.send(msg.to_string().as_ref()) {
            Ok(msg_id) => Ok(msg_id.into()),
            Err(err) => Err(err),
        }
    }

    fn send(&self, msg: &str) -> crate::api::Result<Convert<i32>>;

    /// type参数暂不支持
    fn send_rps(&self) -> Result<i32> {
        self.send_message(CQCode::Rps(0))
    }

    /// type参数暂不支持
    fn send_dice(&self) -> Result<i32> {
        self.send_message(CQCode::Dice(1))
    }

    fn send_shake(&self) -> Result<i32> {
        self.send_message(CQCode::Shake())
    }

    fn send_anonymous(&self, ignore: bool, msg: impl ToString) -> Result<i32> {
        self.send_message(
            MessageSegment::new()
                .add(CQCode::Anonymous(ignore))
                .add(msg),
        )
    }

    fn send_location(
        &self,
        latitude: f32,
        longitude: f32,
        title: &str,
        content: &str,
    ) -> Result<i32> {
        self.send_message(CQCode::Location(
            latitude,
            longitude,
            title.to_owned(),
            content.to_owned(),
        ))
    }

    fn send_music(&self, _type: &str, id: i32, style: i32) -> Result<i32> {
        self.send_message(CQCode::Music(_type.to_owned(), id, style))
    }

    fn send_music_custom(
        &self,
        url: &str,
        audio: &str,
        title: &str,
        content: &str,
        image: &str,
    ) -> Result<i32> {
        self.send_message(CQCode::MusicCustom(
            url.to_owned(),
            audio.to_owned(),
            title.to_owned(),
            content.to_owned(),
            image.to_owned(),
        ))
    }

    fn send_share(&self, url: &str, title: &str, content: &str, image: &str) -> Result<i32> {
        self.send_message(CQCode::Share(
            url.to_owned(),
            title.to_owned(),
            content.to_owned(),
            image.to_owned(),
        ))
    }

    fn at(&self, user_id: i64, msg: impl ToString) -> Result<i32> {
        self.send_message(MessageSegment::new().add(CQCode::At(user_id)).add(msg))
    }
}
