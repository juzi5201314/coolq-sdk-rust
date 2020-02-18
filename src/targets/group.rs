use std::convert::TryInto;
use std::io::{Cursor, Result};

use byteorder::{BigEndian, ReadBytesExt};

use crate::api::{
    Convert, get_group_info, get_group_member_info_v2, get_group_member_list,
    send_group_msg, set_group_anonymous, set_group_ban,
    set_group_kick, set_group_whole_ban,
};
use crate::targets::message::SendMessage;
use crate::targets::ReadString;
use crate::targets::user::UserSex;

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
            _ => GroupRole::Member,
        }
    }
}

#[derive(Debug, Clone)]
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
    fn send(&self, msg: &str) -> crate::api::Result<Convert<i32>> {
        send_group_msg(self.group_id, msg)
    }
}

impl GroupMember {
    pub(crate) fn decode(b: &[u8]) -> std::io::Result<GroupMember> {
        let mut b = Cursor::new(base64::decode(&b).expect("Invalid base64 - decode GroupMember"));
        Ok(GroupMember {
            group_id: b.read_i64::<BigEndian>()?,
            user_id: b.read_i64::<BigEndian>()?,
            nickname: b.read_string()?,
            card: b.read_string()?,
            sex: UserSex::from(b.read_i32::<BigEndian>()?),
            age: b.read_i32::<BigEndian>()?,
            area: b.read_string()?,
            join_time: b.read_i32::<BigEndian>()?,
            last_sent_time: b.read_i32::<BigEndian>()?,
            level: b.read_string()?,
            role: GroupRole::from(b.read_i32::<BigEndian>()?),
            unfriendly: b.read_i32::<BigEndian>()? > 0,
            title: b.read_string()?,
            title_expire_time: b.read_i32::<BigEndian>()?,
            card_changeable: b.read_i32::<BigEndian>()? > 0,
        })
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
    fn send(&self, msg: &str) -> crate::api::Result<Convert<i32>> {
        send_group_msg(self.group_id, msg)
    }
}

impl Group {
    pub fn new(group_id: i64) -> Group {
        get_group_info(group_id, false)
            .expect("cannot get group info.")
            .try_into()
            .expect("cannot decode group")
    }

    /// 部分参数如 area、title 等等无法获取到（为空）。要获取全部参数请使用 get_member。
    pub fn get_members(&self) -> crate::api::Result<Vec<GroupMember>> {
        Ok(get_group_member_list(self.group_id)?
            .try_into()
            .expect("cannot decode GroupMember list"))
    }

    pub fn get_member(&self, user_id: i64) -> crate::api::Result<GroupMember> {
        Ok(get_group_member_info_v2(self.group_id, user_id, false)?
            .try_into()
            .expect("cannot decode GroupMember"))
    }

    pub fn set_can_anonymous(&self, enable: bool) -> crate::api::Result<Convert<i32>> {
        set_group_anonymous(self.group_id, enable)
    }

    pub fn set_whole_ban(&self, enable: bool) -> crate::api::Result<Convert<i32>> {
        set_group_whole_ban(self.group_id, enable)
    }

    pub fn set_ban(&self, user_id: i64, time: i64) -> crate::api::Result<Convert<i32>> {
        set_group_ban(self.group_id, user_id, time)
    }

    pub fn set_kick(&self, user_id: i64, refuse_rejoin: bool) -> crate::api::Result<Convert<i32>> {
        set_group_kick(self.group_id, user_id, refuse_rejoin)
    }

    pub fn update(&mut self) -> crate::api::Result<Group> {
        Ok(get_group_info(self.group_id, true)?
            .try_into()
            .expect("cannot decode Group"))
    }

    /// 用于get_group_list
    /// 没有群人数信息
    pub(crate) fn decode_small(b: &[u8]) -> Result<Group> {
        let mut b = Cursor::new(b);
        Ok(Group {
            group_id: b.read_i64::<BigEndian>()?,
            group_name: b.read_string()?,
            ..Default::default()
        })
    }

    pub(crate) fn decode(b: &[u8]) -> Result<Group> {
        let mut b = Cursor::new(base64::decode(&b).unwrap());
        Ok(Group {
            group_id: b.read_i64::<BigEndian>()?,
            group_name: b.read_string()?,
            member_count: b.read_i32::<BigEndian>()?,
            max_member_count: b.read_i32::<BigEndian>()?,
        })
    }
}
