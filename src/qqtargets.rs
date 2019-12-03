use std::io::{Cursor, Read, Error};
use std::default::Default;
use std::ops::Add;

use byteorder::{BigEndian, ReadBytesExt};

use encoding::{EncoderTrap, DecoderTrap, Encoding};
use encoding::all::GB18030;

use crate::api::{send_private_msg, get_stranger_info, add_log, CQLogLevel, get_group_member_list, get_group_member_info_v2, get_group_info, send_group_msg, set_group_anonymous_ban, Flag};

macro_rules! utf8 {
    ($b:expr) => {
        unsafe {
            GB18030
                .decode($b, DecoderTrap::Ignore).unwrap()[..]
                .to_string()
        }
    }
}

pub(crate) fn read_multi_object(b: Vec<u8>) -> Vec<Vec<u8>> {
    let mut b = Cursor::new(base64::decode(&b).unwrap());
    let count = b.read_i32::<BigEndian>().unwrap();
    let mut vs = Vec::new();
    for _ in 0..count {
        let mut v = vec![0u8; b.read_i16::<BigEndian>().unwrap() as usize];
        b.read_exact(&mut v);
        vs.push(v);
    }
    vs
}

pub trait ReadString: Read {
    fn read_string(&mut self) -> Result<String, Error> {
        let len = self.read_i16::<BigEndian>()?;
        if len > 0 {
            let mut v = vec![0u8; len as usize];
            self.read_exact(&mut v);
            Ok(utf8!(v.as_slice()))
        } else { Ok(String::new()) }
    }
}

impl<R: Read + ?Sized> ReadString for R {}

#[derive(Debug)]
pub struct File {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub busid: i64
}

impl File {
    pub(crate) fn decode(b: Vec<u8>) -> File {
        let mut b = Cursor::new(base64::decode(&b).unwrap());
        File {
            id: b.read_string().unwrap(),
            name: b.read_string().unwrap(),
            size: b.read_i64::<BigEndian>().unwrap(),
            busid: b.read_i64::<BigEndian>().unwrap()
        }
    }
}

#[derive(Debug, Default)]
pub struct Anonymous {
    pub group_id: i64,
    pub user_id: i64,
    pub name: String,
    pub flag: Flag
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
            flag: unsafe { String::from_utf8_unchecked(b)}
        }
    }
}


#[derive(Debug)]
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

#[derive(Debug, Default)]
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

impl Message for GroupMember {
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

#[derive(Debug, Default)]
pub struct Group {
    pub group_id: i64,
    pub group_name: String,
    pub member_count: i32,
    pub max_member_count: i32,
}

impl Message for Group {
    fn send_message(&self, msg: &str) {
        send_group_msg(self.group_id, msg);
    }
}

impl Group {
    //只有群号信息
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct User {
    pub user_id: i64,
    pub nickname: String,
    pub sex: UserSex,
    pub age: i32,
}

impl Message for User {
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
            nickname: {
                let len = b.read_i16::<BigEndian>().unwrap();
                if len > 0 {
                    let mut v = vec![0; len as usize];
                    b.read_exact(&mut v);
                    utf8!(v.as_slice())
                } else { String::new() }
            },
            sex: UserSex::from(b.read_i32::<BigEndian>().unwrap()),
            age: b.read_i32::<BigEndian>().unwrap(),
        }
    }
}

pub trait Message {
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

    fn send_share(&self, url: &str,  title: &str, content: &str, image: &str) {
        self.send_message(cqcode::share(url, title, content, image).as_str())
    }

}

pub mod cqcode {
    use std::path::Path;
    use std::fs;

    use crate::api::get_app_directory;
    use std::io::Write;

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
        Binary(Vec<u8>)
    }

    impl Image {
        pub(crate) fn to_default(&self) -> String {
            let data_dir = get_app_directory();
            let data_dir = Path::new(data_dir.as_str()).parent().unwrap().parent().unwrap();
            match self {
                Image::Default(s) => s.clone(),
                Image::File(s) => {
                    let filename = Path::new(s).file_name().unwrap();
                    fs::copy(s, data_dir.join(Path::new(filename)));
                    filename.to_str().unwrap().to_string()
                },
                Image::Binary(b) => {
                    let filename = format!("{}.jpg", uuid::Uuid::new_v4());
                    let mut f = fs::File::create(Path::new(data_dir).join(&filename)).unwrap();
                    f.write_all(b);
                    f.flush();
                    filename
                },
                Image::Base64(s) => {
                    let filename = format!("{}.jpg", uuid::Uuid::new_v4());
                    let mut f = fs::File::create(Path::new(data_dir).join(&filename)).unwrap();
                    f.write_all(base64::decode(s.as_bytes()).unwrap().as_slice());
                    f.flush();
                    filename
                },
                Image::Http(s) => {
                    let filename = format!("{}.jpg", uuid::Uuid::new_v4());
                    let mut f = fs::File::create(Path::new(data_dir).join(&filename)).unwrap();
                    f.write_all(reqwest::get(s.as_str()).unwrap().text().unwrap().as_bytes());
                    f.flush();
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

    pub fn rps(_type: i64) -> String {
        format!("[CQ:rps,type={t}]", t = _type)
    }

    pub fn dice(_type: i64) -> String {
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

    pub fn share(url: &str,  title: &str, content: &str, image: &str) -> String {
        format!("[CQ:share,url={url},title={title},content={content},image={image}]", url= url, title = title, content = content, image = image)
    }

}