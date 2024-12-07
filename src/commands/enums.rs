#[derive(Debug)]
pub enum Command {
    Createdb,
    List,
    Rename,
    Drop,
    Count,
    Backup,
    Restore,
    Init,
    Migration,
    Update,
}

impl Command {
    pub fn from_str(name: &str) -> Option<Command> {
        match name {
            "createdb" => Some(Command::Createdb),
            "list" => Some(Command::List),
            "rename" => Some(Command::Rename),
            "drop" => Some(Command::Drop),
            "count" => Some(Command::Count),
            "backup" => Some(Command::Backup),
            "restore" => Some(Command::Restore),
            "init" => Some(Command::Init),
            "migration" => Some(Command::Migration),
            "update" => Some(Command::Update),
            _ => None,
        }
    }
}
