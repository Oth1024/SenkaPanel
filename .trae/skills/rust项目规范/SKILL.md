---
name: rust项目规范
description: 用户要求智能体审查rust项目时，除了审查官方推荐的编码规范性问题，还要根据此文件审查项目规范性问题
---

# Rust项目编码规范性文件

## 文件目录、
- 不推荐通过在工作目录下直接创建crate的方式构建项目，而是在工作目录下创建Source文件夹，并在Source文件夹中创建workspace，并且在workspace中创建crate;
- 需求文档、操作手册、使用手册等文档性文件需要存放在工作目录下的Documents文件夹中，Documents中的结构视实际项目情况而定;
- .git/.vscode文件夹、.gitignore/README.md文件需要放在文件目录的根目录下;
- Source中的文件及文件夹以rust推荐文件命名为准;

示例:
+ (工作目录)
    + .git
    + .vscode
    + .gitignore
    + README.md
    + Documents
        + 需求文档
        + 其他文档
    + Source
        + Cargo.lock
        + Cargo.toml
        + crate_1
        + crate_2

## Cargo.toml
- workspace的Cargo.toml中仅允许给出crate的版本信息，即形如:

        [workspace.dependenices]
        xxx = "1.1.1"
- workspace的Cargo.toml允许酌情根据crate的用途添加分组和注释，注释在每个分组的前面，分组间用空行隔离，形如:

        [workspace.dependencies]
        # framework
        notify = "8.2.0"
        tokio = "1.51.1"

        # utilities
        os_info = "3.14.0"
        libc = "0.2.184"
- crate中仅允许引用workspace的Cargo.toml中定义的在线crate以及本地crate;使用在线crate仅允许引用workspace中定义的版本;允许指定feature;
- crate的Cargo.toml中如果同时存在workspace中的本地crate和在线crate的情况下，需要按照以下方式添加注释:

        1. 本地crate为通过 xxx = { path = "paht/of/crate" }的方式添加的crate,
        2. 所有本地crate放在一起，并在第一个本地crate的前一行添加 # local crate注释,
        3. 本地crate的组和在线crate的组中需要空行
## use与mod
- 所有mod需要写在use前，并且mod与use间需要空行;
- 避免无用的crate use，如果某个crate中的方法或者struct的引用在当前文件中完全没有使用，则删除该use;
- 如果仅引用了crate或者其名下mod中的一个成员，则不允许使用{}包含该成员，而是直接跟随该成员，即:
        
        禁止: crate1::mod1::{member};
        采用: crate1::mod2::member;
- 同个命名空间下存在多个use的情况下，每个成员单独存在一行，即:
    
        use crate1::mod1:: {
            member1,
            member2
        };
## 代码规范
- 代码规范以rust官方推荐为准;