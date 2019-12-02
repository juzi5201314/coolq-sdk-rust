//因为使用宏的话会导致ide没有代码提示而放弃。（2019.12.2 使用实验性宏展开）
//dirmod::all!(default file);

mod enable;
mod private_message;
mod group_message;
mod discuss_message;
mod group_upload;
mod group_admin;
mod group_member_decrease;
mod group_member_increase;
mod group_ban;
mod friend_add;
mod add_friend_request;
mod add_group_request;

pub use enable::*;
pub use private_message::*;
pub use group_message::*;
pub use discuss_message::*;
pub use group_upload::*;
pub use group_admin::*;
pub use group_member_decrease::*;
pub use group_member_increase::*;
pub use group_ban::*;
pub use friend_add::*;
pub use add_friend_request::*;
pub use add_group_request::*;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Events {
    Enable,
    Disable,
    Start,
    Exit,
    GroupMessage,
    DiscussMessage,
    PrivateMessage,
    GroupUpload,
    GroupAdmin,
    GroupBan,
    GroupMemberDecrease,
    GroupMemberIncrease,
    FriendAdd,
    AddFriendRequest,
    AddGroupRequest,
}

pub trait Event {
    fn get_type(&self) -> Events;
    fn is_cancel(&self) -> bool;
    fn cancel(&mut self);
}
