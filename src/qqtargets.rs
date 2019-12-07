use std::io::{Cursor, Read, Error};
use std::default::Default;
use std::ops::Add;

use byteorder::{BigEndian, ReadBytesExt};

use encoding::{DecoderTrap, Encoding};
use encoding::all::GB18030;

use crate::api::{send_private_msg, get_stranger_info, get_group_member_list, get_group_member_info_v2, get_group_info, send_group_msg, set_group_anonymous_ban, Flag, set_group_card, set_group_anonymous, set_group_whole_ban, set_group_ban, set_group_kick};

macro_rules! utf8 {
    ($b:expr) => {
        GB18030
            .decode($b, DecoderTrap::Ignore).unwrap()[..]
            .to_string()
    }
}

pub(crate) fn read_multi_object(b: Vec<u8>) -> Vec<Vec<u8>> {
    let mut b = Cursor::new(base64::decode(&b).unwrap());
    let count = b.read_i32::<BigEndian>().unwrap();
    let mut vs = Vec::new();
    for _ in 0..count {
        let mut v = vec![0u8; b.read_i16::<BigEndian>().unwrap() as usize];
        b.read_exact(&mut v).unwrap();
        vs.push(v);
    }
    vs
}

pub trait ReadString: Read {
    fn read_string(&mut self) -> Result<String, Error> {
        let len = self.read_i16::<BigEndian>()?;
        if len > 0 {
            let mut v = vec![0u8; len as usize];
            self.read_exact(&mut v).unwrap();
            Ok(unsafe { utf8!(v.as_slice()) })
        } else { Ok(String::new()) }
    }
}

impl<R: Read + ?Sized> ReadString for R {}

#[derive(Debug, Clone)]
pub struct File {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub busid: i64,
}

impl File {
    pub(crate) fn decode(b: Vec<u8>) -> File {
        let mut b = Cursor::new(base64::decode(&b).unwrap());
        File {
            id: b.read_string().unwrap(),
            name: b.read_string().unwrap(),
            size: b.read_i64::<BigEndian>().unwrap(),
            busid: b.read_i64::<BigEndian>().unwrap(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Anonymous {
    pub group_id: i64,
    pub user_id: i64,
    pub name: String,
    pub flag: Flag,
}

impl Anonymous {
    pub fn ban(&self, time: i64) {
        set_group_anonymous_ban(self.group_id, self.flag.clone(), time);
    }

    pub(crate) fn decode(b: Vec<u8>, group_id: i64) -> Anonymous {
        let mut c = Cursor::new(base64::decode(&b).unwrap());
        Anonymous {
            group_id: group_id,
            user_id: c.read_i64::<BigEndian>().unwrap(),
            name: c.read_string().unwrap(),
            flag: unsafe { String::from_utf8_unchecked(b) },
        }
    }
}


#[derive(Debug, Clone)]
pub enum GroupRole {
    Member,
    Admin,
    Owner,
}

impl From<i32> for GroupRole {
    fn from(i: i32) -> Self {
        match i {
            1 => GroupRole::Member,
            2 => GroupRole::Admin,
            3 => GroupRole::Owner,
            _ => GroupRole::Member
        }
    }
}

impl Default for GroupRole {
    fn default() -> Self {
        GroupRole::Member
    }
}

#[derive(Debug, Default, Clone)]
pub struct GroupMember {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub card: String,
    pub sex: UserSex,
    pub age: i32,
    pub area: String,
    pub join_time: i32,
    pub last_sent_time: i32,
    pub level: String,
    pub role: GroupRole,
    pub unfriendly: bool,
    pub title: String,
    pub title_expire_time: i32,
    pub card_changeable: bool,
}

impl SendMessage for GroupMember {
    fn send_message(&self, msg: &str) {
        send_private_msg(self.user_id, msg);
    }
}

impl GroupMember {
    pub(crate) fn decode(b: Vec<u8>) -> GroupMember {
        let mut b = Cursor::new(b);
        GroupMember {
            group_id: b.read_i64::<BigEndian>().unwrap(),
            user_id: b.read_i64::<BigEndian>().unwrap(),
            nickname: b.read_string().unwrap(),
            card: b.read_string().unwrap(),
            sex: UserSex::from(b.read_i32::<BigEndian>().unwrap()),
            age: b.read_i32::<BigEndian>().unwrap(),
            area: b.read_string().unwrap(),
            join_time: b.read_i32::<BigEndian>().unwrap(),
            last_sent_time: b.read_i32::<BigEndian>().unwrap(),
            level: b.read_string().unwrap(),
            role: GroupRole::from(b.read_i32::<BigEndian>().unwrap()),
            unfriendly: if b.read_i32::<BigEndian>().unwrap() > 0 {
                true
            } else {
                false
            },
            title: b.read_string().unwrap(),
            title_expire_time: b.read_i32::<BigEndian>().unwrap(),
            card_changeable: if b.read_i32::<BigEndian>().unwrap() > 0 {
                true
            } else {
                false
            },
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Group {
    pub group_id: i64,
    pub group_name: String,
    pub member_count: i32,
    pub max_member_count: i32,
}

impl SendMessage for Group {
    fn send_message(&self, msg: &str) {
        send_group_msg(self.group_id, msg);
    }
}

impl Group {
    pub fn new(group_id: i64) -> Group {
        get_group_info(group_id, false)
    }

    //部分参数如 area、title 等等无法获取到（为空）。要获取全部参数请使用 get_member。
    pub fn get_members(&self) -> Vec<GroupMember> {
        get_group_member_list(self.group_id)
    }

    pub fn get_member(&self, user_id: i64) -> GroupMember {
        get_group_member_info_v2(self.group_id, user_id, false)
    }

    pub fn get_member_no_cache(&self, user_id: i64) -> GroupMember {
        get_group_member_info_v2(self.group_id, user_id, true)
    }

    pub fn set_card(&self, user_id: i64, card: &str) {
        set_group_card(self.group_id, user_id, card);
    }

    pub fn set_anonymous(&self, enable: bool) {
        set_group_anonymous(self.group_id, enable);
    }

    pub fn set_whole_ban(&self, enable: bool) {
        set_group_whole_ban(self.group_id, enable);
    }

    pub fn set_ban(&self, user_id: i64, time: i64) {
        set_group_ban(self.group_id, user_id, time);
    }

    pub fn set_kick(&self, user_id: i64, refuse_rejoin: bool) {
        set_group_kick(self.group_id, user_id, refuse_rejoin);
    }

    pub fn update(&mut self) {
        *self = get_group_info(self.group_id, true)
    }

    //用于get_group_list
    //只有基础信息
    pub(crate) fn decode_base(b: Vec<u8>) -> Group {
        let mut b = Cursor::new(b);
        Group {
            group_id: b.read_i64::<BigEndian>().unwrap(),
            group_name: b.read_string().unwrap(),
            ..Default::default()
        }
    }

    //含有基础和群人数信息
    pub(crate) fn decode(b: Vec<u8>) -> Group {
        let mut b = Cursor::new(base64::decode(&b).unwrap());
        Group {
            group_id: b.read_i64::<BigEndian>().unwrap(),
            group_name: b.read_string().unwrap(),
            member_count: b.read_i32::<BigEndian>().unwrap(),
            max_member_count: b.read_i32::<BigEndian>().unwrap(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserSex {
    Male,
    Female,
    Unknown,
}

impl From<i32> for UserSex {
    fn from(i: i32) -> Self {
        match i {
            0 => UserSex::Male,
            1 => UserSex::Female,
            _ => UserSex::Unknown
        }
    }
}

impl Default for UserSex {
    fn default() -> Self {
        UserSex::Unknown
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: i64,
    pub nickname: String,
    pub sex: UserSex,
    pub age: i32,
}

impl SendMessage for User {
    fn send_message(&self, msg: &str) {
        send_private_msg(self.user_id, msg);
    }
}

impl User {
    //为了防止获取频率过大，所有从事件获取到的User皆是从缓存取的。
    //如果想获得最新信息，请使用update。
    pub(crate) fn new(user_id: i64) -> User {
        get_stranger_info(user_id, false)
    }

    pub fn update(&mut self) {
        *self = get_stranger_info(self.user_id, true);
    }

    pub(crate) fn decode(b: Vec<u8>) -> User {
        let mut b = Cursor::new(base64::decode(&b).unwrap());
        User {
            user_id: b.read_i64::<BigEndian>().unwrap(),
            nickname: b.read_string().unwrap(),
            sex: UserSex::from(b.read_i32::<BigEndian>().unwrap()),
            age: b.read_i32::<BigEndian>().unwrap(),
        }
    }
}

pub trait SendMessage {
    fn send_message(&self, msg: &str);

    fn send_image_message(&self, msg: &str, images: Vec<cqcode::Image>) {
        let mut s = String::new();
        for i in images {
            s.push_str(cqcode::image(i).as_str());
        }
        self.send_message(s.add(msg).as_str());
    }

    //type参数暂不支持
    fn send_rps(&self) {
        self.send_message(cqcode::rps(1).as_str())
    }

    //type参数暂不支持
    fn send_dice(&self) {
        self.send_message(cqcode::dice(1).as_str())
    }

    fn send_shake(&self) {
        self.send_message(cqcode::shake().as_str())
    }

    fn send_anonymous(&self, ignore: bool, msg: &str) {
        self.send_message(cqcode::anonymous(ignore).add(msg).as_str())
    }

    fn send_location(&self, latitude: f32, longitude: f32, title: &str, content: &str) {
        self.send_message(cqcode::location(latitude, longitude, title, content).as_str())
    }

    fn send_music(&self, _type: &str, id: i32, style: i32) {
        self.send_message(cqcode::music(_type, id, style).as_str())
    }

    fn send_music_custom(&self, url: &str, audio: &str, title: &str, content: &str, image: &str) {
        self.send_message(cqcode::music_custom(url, audio, title, content, image).as_str())
    }

    fn send_share(&self, url: &str, title: &str, content: &str, image: &str) {
        self.send_message(cqcode::share(url, title, content, image).as_str())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Message {
    pub msg: String,
    pub raw_msg: String,
    //如果没有cq吗，则此成员为空。如果未知/未支持（如CQ:hb,CQ:rich），则为Unknown。如果解析错误，则返回默认值。
    pub cqcodes: Vec<cqcode::CQCode>,
}

impl Message {
    pub fn new(s: String) -> Message {
        if cqcode::has_cq_code(s.as_str()) {
            Message {
                msg: Message::escape(cqcode::clean(s.clone())),
                cqcodes: cqcode::parser(s.clone()),
                raw_msg: s,
            }
        } else {
            Message {
                msg: Message::escape(s.clone()),
                raw_msg: s,
                cqcodes: Vec::new(),
            }
        }
    }

    //将因为防止与cq码混淆而转义的字符还原
    fn escape(s: String) -> String {
        s.replace("&amp;", "&")
            .replace("&#91;", "[")
            .replace("&#93;", "]")
            .replace("&#44;", ",")
    }
}

pub mod cqcode {
    use std::path::Path;
    use std::fs;
    use std::io::Write;

    use crate::api::{get_app_directory, add_log, CQLogLevel};

    use regex::Regex;
    use std::collections::HashMap;
    use reqwest::RedirectPolicy;
    use std::time::Duration;

    lazy_static! {
            static ref tag: Regex = Regex::new(r"\[CQ:([A-Za-z]*)(?:(,[^\[\]]+))?]").unwrap();
            static ref args: Regex = Regex::new(r",([A-Za-z]+)=([^,\[\]]+)").unwrap();
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
        Rps(i32),
        //Dice(i32), 接收的骰子表情是一个sface。猜拳未试，估计也是。
        Shake(),
        Sign(String, String, String),
        Location(f32, f32, String, String),
        Share(String, String, String, String),//
        Unknown(),
    }


    pub(crate) fn parser(s: String) -> Vec<CQCode> {
        if has_cq_code(s.as_str()) {
            tag.captures_iter(s.as_str()).map(|c| {
                match c.get(1) {
                    Some(m) => {
                        let cargs: HashMap<String, String> = match c.get(2) {
                            Some(arg) => {
                                args.captures_iter(arg.as_str()).map(|c| {
                                    (c.get(1).unwrap().as_str().to_string(), c.get(2).unwrap().as_str().to_string())
                                }).collect()
                            }
                            None => HashMap::new()
                        };
                        let get_arg = |arg: &str| -> String {
                            cargs.get(arg).unwrap_or(&std::default::Default::default()).clone()
                        };

                        match m.as_str() {
                            "face" => CQCode::Face(get_arg("id").parse::<i32>().unwrap()),
                            "emoji" => CQCode::Emoji(get_arg("id").parse::<i32>().unwrap()),
                            "bface" => CQCode::Bface(get_arg("id").parse::<i32>().unwrap()),
                            "sface" => CQCode::Sface(get_arg("id").parse::<i32>().unwrap()),
                            "image" => CQCode::Image(get_arg("file").clone()),
                            "record" => CQCode::Record(get_arg("file").clone(), if get_arg("magic") == "true" { true } else { false }),
                            "at" => CQCode::At(get_arg("qq").parse::<i64>().unwrap()),
                            "rps" => CQCode::Sface(get_arg("type").parse::<i32>().unwrap()),
                            "shake" => CQCode::Shake(),
                            "location" => CQCode::Location(get_arg("lat").parse::<f32>().unwrap(), get_arg("lon").parse::<f32>().unwrap(), get_arg("title").clone(), get_arg("content").clone()),
                            "sign" => CQCode::Sign(get_arg("location").clone(), get_arg("title").clone(), get_arg("image").clone()),
                            "share" => CQCode::Share(get_arg("url").clone(), get_arg("title").clone(), get_arg("content").clone(), get_arg("image").clone()),
                            _ => CQCode::Unknown()
                        }
                    }
                    None => {
                        CQCode::Unknown()
                    }
                }
            }).collect()
        } else {
            Vec::new()
        }
    }

    pub fn clean(s: String) -> String {
        tag.replace_all(s.as_str(), "").to_string()
    }

    pub fn has_cq_code(s: &str) -> bool {
        tag.is_match(s)
    }

    pub enum Image {
        //默认发送data\image\{image}。"xx.jpg"
        Default(String),
        //发送指定目录下的{image}。该目录必须可读。"/home/me/xx.jpg"
        File(String),
        //从网络下载图片发送。"http://domain.com/xx.jpg"
        Http(String),
        //发送base64编码的图片。"JXU2MThCJXU4QkY0JXU4QkREJXVGRjBDJXU1NDNCJXU2MjEx"
        Base64(String),
        //发送二进制图片。"很明显，这个没办法演示给你看"
        Binary(Vec<u8>),
    }

    impl Image {
        pub(crate) fn to_default(&self) -> String {
            let data_dir = get_app_directory();
            //get_app_directory获取到的路径为 酷q目录\data\app\{appid} ，父目录的父目录则为 酷q目录\data。
            let data_dir = Path::new(data_dir.as_str()).parent().unwrap().parent().unwrap().join(Path::new("image"));
            match self {
                Image::Default(s) => s.clone(),
                Image::File(s) => {
                    let filename = Path::new(s).file_name().unwrap();
                    fs::copy(s, data_dir.join(Path::new(filename))).unwrap();
                    filename.to_str().unwrap().to_string()
                },
                Image::Binary(b) => {
                    let filename = format!("{}.jpg", uuid::Uuid::new_v4());
                    let mut f = fs::File::create(data_dir.join(&filename)).unwrap();
                    f.write_all(b).unwrap();
                    f.flush().unwrap();
                    filename
                },
                Image::Base64(s) => {
                    let filename = format!("{}.jpg", uuid::Uuid::new_v4());
                    let mut f = fs::File::create(data_dir.join(&filename)).unwrap();
                    f.write_all(base64::decode(s.as_bytes()).unwrap().as_slice()).unwrap();
                    f.flush().unwrap();
                    filename
                },
                Image::Http(s) => {
                    let filename = format!("{}.jpg", uuid::Uuid::new_v4());
                    let mut f = fs::File::create(data_dir.join(&filename)).unwrap();
                    let mut b: Vec<u8> = Vec::new();
                    //最多只能重定向10次，并在5秒之后超时
                    match reqwest::Client::builder().redirect(RedirectPolicy::limited(10)).timeout(Duration::from_secs(5)).build().unwrap().get(s.as_str()).send() {
                        Ok(mut r) => {
                            r.copy_to(&mut b).unwrap_or_default();
                        },
                        Err(e) => {
                            add_log(CQLogLevel::ERROR, "error", e.to_string().as_str());
                        }
                    };
                    f.write_all(b.as_slice()).unwrap();
                    f.flush().unwrap();
                    filename
                }
            }
        }
    }

    // 不解析cp码
    pub fn no_code(s: &str) -> String {
        let mut vc = Vec::new();
        for c in s.chars() {
            match c {
                '&' => vc.extend_from_slice(&['&', 'a', 'm', 'p', ';']),
                '[' => vc.extend_from_slice(&['&', '#', '9', '1', ';']),
                ']' => vc.extend_from_slice(&['&', '#', '9', '3', ';']),
                ',' => vc.extend_from_slice(&['&', '#', '4', '4', ';']),
                _ => vc.push(c)
            };
        }
        vc.into_iter().collect()
    }

    pub fn face(id: i32) -> String {
        format!("[CQ:face,id={id}]", id = id)
    }

    pub fn emoji(id: i32) -> String {
        format!("[CQ:emoji,id={id}]", id = id)
    }

    pub fn bface(id: i32) -> String {
        format!("[CQ:bface,id={id}]", id = id)
    }

    pub fn sface(id: i32) -> String {
        format!("[CQ:sface,id={id}]", id = id)
    }

    pub fn image(image: Image) -> String {
        format!("[CQ:image,file={filename}]", filename = image.to_default())
    }

    pub fn record(filename: &str, magic: bool) -> String {
        format!("[CQ:record,file={filename},magic={magic}]", filename = filename, magic = if magic { "true" } else { "false" })
    }

    pub fn at(user_id: i64) -> String {
        format!("[CQ:at,qq={qq}]", qq = user_id)
    }

    pub fn rps(_type: i32) -> String {
        format!("[CQ:rps,type={t}]", t = _type)
    }

    pub fn dice(_type: i32) -> String {
        format!("[CQ:dice,type={t}]", t = _type)
    }

    pub fn shake() -> String {
        "[CQ:shake]".to_string()
    }

    pub fn anonymous(ignore: bool) -> String {
        format!("[CQ:anonymous,ignore={ignore}]", ignore = if ignore { "true" } else { "false" })
    }

    pub fn location(latitude: f32, longitude: f32, title: &str, content: &str) -> String {
        format!("[CQ:location,lat={lat},lon={lon},title={title},content={content}]", lat = latitude, lon = longitude, title = title, content = content)
    }

    pub fn music(_type: &str, id: i32, style: i32) -> String {
        format!("[CQ:music,type={t},id={id},style={style}]", t = _type, id = id, style = style)
    }

    pub fn music_custom(url: &str, audio: &str, title: &str, content: &str, image: &str) -> String {
        format!("[CQ:music,type=custom,url={url},audio={audio},title={title},content={content},image={image}]", url = url, audio = audio, title = title, content = content, image = image)
    }

    pub fn share(url: &str, title: &str, content: &str, image: &str) -> String {
        format!("[CQ:share,url={url},title={title},content={content},image={image}]", url = url, title = title, content = content, image = image)
    }
}