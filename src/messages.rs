pub const BOT_ABOUT: &'static str =
    "*Hang it bot*\n\nHang your boss up\\!\n[Github](https://github.com/senseab/hangitbot) @hangitbot";

pub const BOT_TEXT_NO_TARGET: &'static str = "请在回复某一条信息时使用该指令";
pub const BOT_TEXT_IS_CHANNEL: &'static str = "这不是个人，吊不起来";
pub const BOT_TEXT_TOP_TITLE: &'static str = "**吊人排行榜**";
pub const BOT_TEXT_TOP_GROUP: &'static str = "群内";
pub const BOT_TEXT_TOP_GLOBAL: &'static str = "全球";
pub const BOT_TEXT_TOP_NONE: &'static str = "无人上榜";
pub const BOT_TEXT_TOP_TEMPLATE: &'static str = "{name} 被吊了 {count} 次";

const BOT_TEXT_HANGED_1: &'static str = "{name} 被吊路灯了，绳子是昨天他卖出去的……";
const BOT_TEXT_HANGED_2: &'static str = "因为 {name} 太过逆天，我们把 TA 吊在了路灯上……";
const BOT_TEXT_HANGED_3: &'static str = "{name} 吊在了路灯上，TA 兴风作浪的时代结束了……";
const BOT_TEXT_HANGED_4: &'static str = "吊在路灯上的 {name} 正在接受大家的鄙视……";
const BOT_TEXT_HANGED_5: &'static str = "对 {name} 来说，绳命来得快去得也快，只有路灯是永恒的……";
const BOT_TEXT_HANGED_6: &'static str = "被套上麻袋的 {name} 在经历了一顿胖揍之后，最后还是成了路灯的挂件……";

pub const BOT_TEXT_HANGED: [&str; 6] = [
    BOT_TEXT_HANGED_1,
    BOT_TEXT_HANGED_2,
    BOT_TEXT_HANGED_3,
    BOT_TEXT_HANGED_4,
    BOT_TEXT_HANGED_5,
    BOT_TEXT_HANGED_6,
];

const BOT_TEXT_HANGED_SELF_1: &'static str = "{name} 承受不了自己所做的一切，选择了自行了断……";
const BOT_TEXT_HANGED_SELF_2: &'static str = "对于 {name} 来说，把自己吊在路灯上可能是最好的选择了……";
const BOT_TEXT_HANGED_SELF_3: &'static str = "{name} 最终还是选择了逃避……";

pub const BOT_TEXT_HANGED_SELF: [&str; 3] = [
    BOT_TEXT_HANGED_SELF_1,
    BOT_TEXT_HANGED_SELF_2,
    BOT_TEXT_HANGED_SELF_3,
];

pub const BOT_TEXT_HANG_BOT: &'static str = "机器人是无法被吊死的……";
pub const BOT_TEXT_HANG_CHANNEL: &'static str = "这是个频道……";
pub const BOT_TEXT_HANG_ANONYMOUS: &'static str = "这是个幽灵……";