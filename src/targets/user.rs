use std::convert::TryInto;
use std::io::Cursor;
use std::sync::RwLock;

use byteorder::{BigEndian, ReadBytesExt};

use crate::api::{get_stranger_info, send_private_msg, Convert};
use crate::targets::message::SendMessage;
use crate::targets::ReadString;

lazy_static! {
    static ref MasterList: RwLock<Vec<i64>> = RwLock::new(Vec::new());
    static ref SuperAdminList: RwLock<Vec<i64>> = RwLock::new(Vec::new());
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
            _ => UserSex::Unknown,
        }
    }
}

impl Default for UserSex {
    fn default() -> Self {
        UserSex::Unknown
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
pub enum Authority {
    Master = 0,
    SuperAdmin = 1,
    GroupLord = 2,
    GroupAdmin = 3,
    User = 4,
}

impl Authority {
    pub fn check_authority(&self, authority: Authority) -> bool {
        self <= &authority
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: i64,
    pub nickname: String,
    pub sex: UserSex,
    pub age: i32,
    authority: Authority,
}

impl SendMessage for User {
    fn send(&self, msg: &str) -> crate::api::Result<Convert<i32>> {
        send_private_msg(self.user_id, msg)
    }
}

impl User {
    pub fn add_master(user_id: i64) {
        MasterList.write().unwrap().push(user_id);
    }

    pub fn add_super_admin(user_id: i64) {
        SuperAdminList.write().unwrap().push(user_id);
    }

    pub fn get_masters() -> Vec<i64> {
        MasterList.read().unwrap().clone()
    }

    pub fn get_super_admins() -> Vec<i64> {
        SuperAdminList.read().unwrap().clone()
    }

    //为了防止获取频率过大，所有从事件获取到的User皆是从缓存取的。
    //如果想获得最新信息，请使用update。
    pub(crate) fn new(user_id: i64) -> User {
        let mut user = get_stranger_info(user_id, false)
            .unwrap()
            .try_to::<User>()
            .unwrap();
        if !SuperAdminList
            .read()
            .unwrap()
            .iter()
            .all(|qq| *qq != user_id)
        {
            user.set_authority(Authority::SuperAdmin);
        } else if !MasterList.read().unwrap().iter().all(|qq| *qq != user_id) {
            user.set_authority(Authority::Master);
        }
        user
    }

    pub(crate) fn set_authority(&mut self, authority: Authority) {
        if !self.authority.check_authority(authority) {
            self.authority = authority
        }
    }

    pub fn update(&mut self) -> std::io::Result<User> {
        get_stranger_info(self.user_id, true).unwrap().try_into()
    }

    pub(crate) fn decode(b: &[u8]) -> std::io::Result<User> {
        let mut b = Cursor::new(base64::decode(&b).unwrap());
        Ok(User {
            user_id: b.read_i64::<BigEndian>()?,
            nickname: b.read_string()?,
            sex: UserSex::from(b.read_i32::<BigEndian>()?),
            age: b.read_i32::<BigEndian>()?,
            authority: Authority::User,
        })
    }
}
