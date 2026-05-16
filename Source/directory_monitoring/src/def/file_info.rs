use std::fmt::Display;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct FileInfo{
    /// Common file definition for windows and linux
    // 文件名称
    pub file_name: String,
    // 文件类型
    pub file_type: FileType,
    // 最近的修改时间
    pub last_modified_time: DateTime<Utc>,
    // 文件大小
    pub file_size: u64,
    // 文件权限
    /// Specified for linux
    pub file_mod: Option<FileMode>,
    // 拥有者
    pub owner: Option<String>,
    // 组
    pub group: Option<String>
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:10} {:20} {:10} {:5}",
            self.file_type,
            self.file_name,
            self.last_modified_time,
            self.file_size).as_str())
    }
}

#[derive(Debug)]
pub struct FileMode {
    pub file_type: FileType,

    pub user_read: bool,

    pub user_write: bool,

    pub user_exec: bool,

    pub group_read: bool,

    pub group_write: bool,

    pub group_exec: bool,

    pub other_read: bool,

    pub other_write: bool,

    pub other_exec: bool
}

impl FileMode {
    /// Get File Mode from formatted string like "-rwx-wx--x"
    pub fn from_formatted_str(formatted: &str) -> Option<Self> {
        // Validation
        if formatted.len() != 10 {
            return None
        };
        let mut  chars = formatted.chars();
        let file_type = match &chars.nth(0).unwrap() {
            'd' => FileType::Directory,
            '-' => FileType::File,
            'l' => FileType::Link,
            _ => FileType::Unspecified
        };
        let user_read = chars.nth(1).unwrap() == 'r';
        let user_write = chars.nth(2).unwrap() == 'w';
        let user_exec = chars.nth(3).unwrap() == 'x';
        let group_read = chars.nth(4).unwrap() == 'r';
        let group_write = chars.nth(5).unwrap() == 'w';
        let group_exec = chars.nth(6).unwrap() == 'x';
        let other_read = chars.nth(7).unwrap() == 'r';
        let other_write = chars.nth(8).unwrap() == 'w';
        let other_exec = chars.nth(9).unwrap() == 'x';
        Some(FileMode {
            file_type,
            user_read,
            user_write,
            user_exec,
            group_read,
            group_write,
            group_exec,
            other_read,
            other_write,
            other_exec
        })
    }

    pub fn from_u32(file_type: FileType, mode: u32) -> Self {
        let user_read = mode & 0o400 == 0o400;
        let user_write = mode & 0o200 == 0o200;
        let user_exec = mode & 0o100 == 0o100;
        let group_read = mode & 0o040 == 0o040;
        let group_write = mode & 0o020 == 0o020;
        let group_exec = mode & 0o010 == 0o010;
        let other_read = mode & 0o004 == 0o004;
        let other_write = mode & 0o002 == 0o002;
        let other_exec = mode & 0o001 == 0o001;
        return FileMode {
            file_type,
            user_read,
            user_write,
            user_exec,
            group_read,
            group_write,
            group_exec,
            other_read,
            other_write,
            other_exec
        };
    }

    pub fn as_formatted_string(&self) -> String {
        let mut result = vec!['-','-','-','-','-','-','-','-','-'];

        let file_type = match self.file_type {
            FileType::Unspecified => '-',
            FileType::Directory => 'd',
            FileType::File => '-',
            FileType::Link => 'l',
        };
        result[0] = file_type;

        // user
        if self.user_read { result[1] = 'r' };
        if self.user_write { result[2] = 'w' };
        if self.user_exec { result[3] = 'x' };
        // group
        if self.group_read { result[4] = 'r' };
        if self.group_write { result[5] = 'w' };
        if self.group_exec { result[6] = 'x' };
        // others
        if self.other_read { result[7] = 'r' };
        if self.other_write { result[8] = 'w' };
        if self.other_exec { result[9] = 'x' };

        return String::from_iter(result);
    }
}

#[derive(Clone, Debug)]
pub enum FileType {
    Unspecified,
    Directory,
    File,
    Link
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            FileType::Unspecified => "Unspecified",
            FileType::Directory => "Directory",
            FileType::File => "File",
            FileType::Link => "Link"
        };
        f.write_str(result)
    }
}