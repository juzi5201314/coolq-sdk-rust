use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};

use encoding::{EncoderTrap, DecoderTrap, Encoding};
use encoding::all::GB18030;

use crate::api::{send_private_msg, get_stranger_info};

macro_rules! utf8 {
    ($b:expr) => {
        unsafe {
            GB18030
                .decode($b, DecoderTrap::Ignore)
                .unwrap()[..]
                .to_string()
        }
    }
}

pub enum UserSex {
    Male,
    Female,
    Unknown
}

pub struct User {
    pub user_id: i64,
    pub nickname: String,
    pub sex: UserSex,
    pub age: i32
}

impl User {

    //为了防止获取频率过大，所有从事件获取到的User皆是从缓存取的。
    //如果想获得最新信息，请使用update。
    pub(crate) fn new(user_id: i64) -> User {
        get_stranger_info(user_id, true)
    }

    pub fn send_message(&self, msg: &str) {
        send_private_msg(self.user_id, msg);
    }

    pub fn update(&mut self) {
        *self = get_stranger_info(user_id, false);
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
            sex: match b.read_i32::<BigEndian>().unwrap() {
                0 => UserSex::Male,
                1 => UserSex::Female,
                _ => UserSex::Unknown
            },
            age: b.read_i32::<BigEndian>().unwrap()
        }
    }
}