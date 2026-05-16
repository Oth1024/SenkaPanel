#[cfg(test)]
mod test {
    use std::{path::Path, time::Duration};

    use directory_monitoring::{monitor_fs, show_fs_on_dir, subscribe_fs_changed_event};
    use notify::Event;
    use tokio::{fs::{self, File}, time};

    // Test consts
    const TEST_DIR: &str = "C:\\Users\\Oth1024\\Downloads";
    const TEST_FILE: &str = "TEST_FILE.TEST";

    #[test]
    fn test_show_fs() {
        let file_infos = show_fs_on_dir(TEST_DIR);
        for file_info in file_infos {
            print!("{}\n", file_info)
        }
    }

    #[tokio::test]
    async fn test_monitoring() {
        // 添加一个任务，每隔5s创建、删除一个文件
        tokio::spawn(read_write_file());
        // 注册文件监控以及回调
        subscribe_fs_changed_event(TEST_DIR, "echo", echo_event_handled).unwrap();
        // 启动文件监控（异步，不阻塞）
        tokio::spawn(monitor_fs(TEST_DIR));
        
        // 保持测试运行足够长的时间来观察效果
        tokio::time::sleep(Duration::from_secs(30)).await;
    }

    async fn read_write_file() {
        print!("test enter read_write_file\n");
        let test_file_path = Path::new(TEST_DIR).join(TEST_FILE);
        let mut can_write = true;
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            // 等待下一个命中的tick
            interval.tick().await;

            // 执行删除、创建文件
            if can_write {
                can_write = false;
                if !test_file_path.exists() {
                    File::create_new(&test_file_path).await.expect("Create failed");
                }
            }
            else {
                can_write = true;
                if test_file_path.exists() {
                    fs::remove_file(&test_file_path).await.expect("Remove file failed");
                }
            }
        }
    }

    static mut COUNT: i64 = 0;
    // 仅做测试
    fn echo_event_handled(_: &Event) {
        unsafe {
            let current_count = COUNT.abs();
            print!("On fs event handled, current count:{}\n", current_count);// 实际没有打印出来
            COUNT += 1;
        }
    }
}