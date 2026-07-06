#[cfg(test)]
mod tests {
    use std::time::Duration;

use tokio::sync::broadcast;

    #[tokio::test]
    /// 测试通过 broadcast 管道实时捕获子进程 stdout 输出
    async fn test_raw_task() {
        use process_manager::get_process_manager;

        let manager = get_process_manager();
        manager.start_monitor().await;

        let id = manager.start(
            "cmd".to_string(),
            vec!["/c".to_string(), "echo hello world".to_string()],
            true,
            "test".to_string(),
        )
        .unwrap();

        // 订阅 stdout，无需关心进程状态，管道自动关闭时 recv 返回 Closed
        let rx = {
            let info = manager.process_infos.get(&id).unwrap();
            info.get_stdout_recv().unwrap()
        };
        tokio::task::spawn(print_output(rx));

        tokio::time::sleep(Duration::from_secs(60)).await;
    }

    async fn print_output(mut rx: broadcast::Receiver<String>) {
        loop {
            let mut output = String::new();
            match rx.recv().await {
                Ok(line) => {
                    output.push_str(&line);
                    output.push('\n');
                }
                _ => continue,
            }
            println!("子进程输出: {:?}", output);
        }
    }
}
