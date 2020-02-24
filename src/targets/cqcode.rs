use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
    io::{Error, ErrorKind},
};

use async_std::{
    fs::{copy, File},
    io::prelude::WriteExt,
    path::Path,
};
use hex::ToHex;
use md5::Digest;
use regex::Regex;

use crate::api::get_app_directory;

lazy_static! {
    static ref tag_regex: Regex = Regex::new(r"\[CQ:([A-Za-z]*)(?:(,[^\[\]]+))?]").unwrap();
    static ref args_regex: Regex = Regex::new(r",([A-Za-z]+)=([^,\[\]]+)").unwrap();
}

#[derive(Debug, Clone)]
pub enum CQCode {
    Face(i32),
    Emoji(i32),
    Bface(i32),
    Sface(i32),
    Image(String),
    Record(String, bool),
    At(i64),
    AtAll(),
    Rps(i32),
    Dice(i32),
    Shake(),
    Anonymous(bool),
    Sign(String, String, String),
    Location(f32, f32, String, String),
    Music(String, i32, i32),
    MusicCustom(String, String, String, String, String),
    Share(String, String, String, String),
    Unknown(String),
}

impl Display for CQCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = match self {
            CQCode::Face(id) => format!("[CQ:face,id={}]", id),
            CQCode::Emoji(id) => format!("[CQ:emoji,id={}]", id),
            CQCode::Bface(id) => format!("[CQ:bface,id={}]", id),
            CQCode::Sface(id) => format!("[CQ:sface,id={}]", id),
            CQCode::Image(img) => format!("[CQ:image,file={}]", img),
            CQCode::Record(file, magic) => {
                format!("[CQ:record,file={},magic={}]", file, magic.to_string())
            },
            CQCode::At(qq) => format!("[CQ:at,qq={}]", qq),
            CQCode::AtAll() => "[CQ:at,qq=all]".to_owned(),
            CQCode::Rps(t) => format!("[CQ:rps,type={}]", t),
            CQCode::Dice(t) => format!("[CQ:dice,type={}]", t),
            CQCode::Shake() => "[CQ:shake]".to_owned(),
            CQCode::Anonymous(ignore) => format!("[CQ:anonymous,ignore={}]", ignore.to_string()),
            CQCode::Location(latitude, longitude, title, content) => format!(
                "[CQ:location,lat={},lon={},title={},content={}]",
                latitude, longitude, title, content
            ),
            CQCode::Music(t, id, style) => {
                format!("[CQ:music,type={},id={},style={}]", t, id, style)
            },
            CQCode::MusicCustom(url, audio, title, content, image) => format!(
                "[CQ:music,type=custom,url={},audio={},title={},content={},image={}]",
                url, audio, title, content, image
            ),
            CQCode::Share(url, title, content, image) => format!(
                "[CQ:share,url={},title={},content={},image={}]",
                url, title, content, image
            ),
            _ => String::new(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum CQImage {
    /// 默认发送data\image\{image}。"xx.jpg"
    Default(String),
    /// 发送指定目录下的{image}。该目录必须可读。"/home/me/xx.jpg"
    File(String),
    /// 发送base64编码的图片。"JXU2MThCJXU4QkY0JXU4QkREJXVGRjBDJXU1NDNCJXU2MjEx"
    Base64(String),
    /// 发送二进制图片。"很明显，这个没办法演示给你看"
    Binary(Vec<u8>),
}

impl CQImage {
    /// 有阻塞版本[`to_file_name_blocking`]
    pub async fn to_file_name(&self) -> std::io::Result<String> {
        // 插件数据目录在data\app\appid，借此来获取data目录。
        let image_dir = Path::new(&get_app_directory().unwrap().to::<String>())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("image");
        Ok(match self {
            CQImage::Default(img) => img.clone(),
            CQImage::File(path) => {
                let path = Path::new(path);
                if !path.exists().await && !path.is_file().await {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "image file not found or not a file",
                    ));
                }
                let name = path.file_name().unwrap();
                let from = image_dir.join(name);
                let to = Path::new(&from);
                if !to.exists().await {
                    copy(path, to).await?;
                }
                to.to_str().unwrap().to_owned()
            },
            CQImage::Binary(bytes) => {
                let name = md5::Md5::digest(bytes).encode_hex::<String>();
                let to = image_dir.join(name);
                if !to.exists().await {
                    self.save_file(bytes, &to).await?;
                }
                to.to_str().unwrap().to_owned()
            },
            CQImage::Base64(b64) => {
                let bytes = base64::decode(b64).expect("Invalid base64 - CQImage");
                let name = md5::Md5::digest(&bytes).encode_hex::<String>();
                let to = image_dir.join(name);
                if !to.exists().await {
                    self.save_file(&bytes, &to).await?;
                }
                to.to_str().unwrap().to_owned()
            },
        })
    }

    async fn save_file(&self, data: &[u8], path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path).await?;
        file.write_all(data).await?;
        file.sync_all().await
    }

    /// 有异步版本[`to_file_name`]
    pub fn to_file_name_blocking(&self) -> std::io::Result<String> {
        async_std::task::block_on(async { self.to_file_name().await })
    }
}

pub fn clean(s: &str) -> String {
    tag_regex.replace_all(s, "").to_string()
}

pub trait CQStr {
    fn has_cq_code(&self) -> bool;
    fn no_cq_code(&self) -> String;
}
impl CQStr for str {
    fn has_cq_code(&self) -> bool {
        tag_regex.is_match(self)
    }

    fn no_cq_code(&self) -> String {
        let mut s = String::new();
        for c in self.chars() {
            match c {
                '&' => s.push_str("&amp;"),
                '[' => s.push_str("&#91;"),
                ']' => s.push_str("&#93;"),
                ',' => s.push_str("&#44;"),
                _ => s.push(c),
            };
        }
        s
    }
}

pub fn parse(msg: &str) -> Vec<CQCode> {
    tag_regex
        .captures_iter(msg)
        .map(|codes| {
            let tag = codes.get(1).unwrap();
            let args: HashMap<String, String> = if let Some(arg) = codes.get(2) {
                args_regex
                    .captures_iter(arg.as_str())
                    .map(|a| {
                        (
                            a.get(1).unwrap().as_str().to_string(),
                            a.get(2).unwrap().as_str().to_string(),
                        )
                    })
                    .collect()
            } else {
                HashMap::new()
            };
            let get_arg =
                |name: &str| -> String { args.get(name).unwrap_or(&String::new()).clone() };
            match tag.as_str() {
                "face" => CQCode::Face(get_arg("id").parse::<i32>().unwrap_or(-1)),
                "emoji" => CQCode::Emoji(get_arg("id").parse::<i32>().unwrap_or(-1)),
                "bface" => CQCode::Bface(get_arg("id").parse::<i32>().unwrap_or(-1)),
                "sface" => CQCode::Sface(get_arg("id").parse::<i32>().unwrap_or(-1)),
                "image" => CQCode::Image(get_arg("file").to_owned()),
                "record" => CQCode::Record(get_arg("file").to_owned(), get_arg("magic") == "true"),
                "at" => {
                    if get_arg("qq") == "all" {
                        CQCode::AtAll()
                    } else {
                        CQCode::At(get_arg("qq").parse::<i64>().unwrap_or(-1))
                    }
                },
                "rps" => CQCode::Sface(get_arg("type").parse::<i32>().unwrap_or(-1)),
                "shake" => CQCode::Shake(),
                "location" => CQCode::Location(
                    get_arg("lat").parse::<f32>().unwrap_or(-1f32),
                    get_arg("lon").parse::<f32>().unwrap_or(-1f32),
                    get_arg("title").to_owned(),
                    get_arg("content").to_owned(),
                ),
                "sign" => CQCode::Sign(
                    get_arg("location").to_owned(),
                    get_arg("title").to_owned(),
                    get_arg("image").to_owned(),
                ),
                "share" => CQCode::Share(
                    get_arg("url").to_owned(),
                    get_arg("title").to_owned(),
                    get_arg("content").to_owned(),
                    get_arg("image").to_owned(),
                ),
                _ => CQCode::Unknown(codes.get(0).unwrap().as_str().to_owned()),
            }
        })
        .collect()
}
