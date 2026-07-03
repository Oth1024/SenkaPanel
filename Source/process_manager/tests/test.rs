#[cfg(test)]
mod tests {
    #[tokio::test]
    /// 测试通过 broadcast 管道实时捕获子进程 stdout 输出
    async fn test_raw_task() {
        use process_manager::get_process_manager;
        use tokio::sync::broadcast::error::RecvError;

        let manager = get_process_manager();

        let id = manager.start(
            "cmd".to_string(),
            vec!["/c".to_string(), "echo hello world".to_string()],
            false,
            "test".to_string(),
        )
        .unwrap();

        // 订阅 stdout，无需关心进程状态，管道自动关闭时 recv 返回 Closed
        let mut rx = {
            let info = manager.process_infos.get(&id).unwrap();
            info.subscribe_stdout().unwrap()
        };

        let mut output = String::new();
        loop {
            match rx.recv().await {
                Ok(line) => {
                    output.push_str(&line);
                    output.push('\n');
                }
                Err(RecvError::Closed) => break,
                Err(RecvError::Lagged(_)) => continue,
            }
        }

        println!("子进程输出: {:?}", output);
        assert!(output.contains("hello world"));
    }

    #[tokio::test]
    /// 测试循环输出进程中实时逐行捕获
    async fn test_live_output() {
        use process_manager::get_process_manager;
        use tokio::sync::broadcast::error::RecvError;

        let manager = get_process_manager();

        let id = manager.start(
            "cmd".to_string(),
            vec![
                "/c".to_string(),
                "for /l %i in (1,1,3) do @(echo line %i & ping -n 2 127.0.0.1 >nul)".to_string(),
            ],
            false,
            "test".to_string(),
        )
        .unwrap();

        let mut rx = {
            let info = manager.process_infos.get(&id).unwrap();
            info.subscribe_stdout().unwrap()
        };

        let mut count = 0;
        while count < 3 {
            match tokio::time::timeout(std::time::Duration::from_secs(10), rx.recv()).await {
                Ok(Ok(line)) => {
                    println!("实时捕获: {}", line);
                    count += 1;
                }
                Ok(Err(RecvError::Closed)) => break,
                Ok(Err(RecvError::Lagged(_))) => continue,
                Err(_) => break,
            }
        }

        assert_eq!(count, 3);
    }

    #[tokio::test]
    /// 测试进程监控的开启和关闭
    /// 测试子进程的自动重启
    async fn test_monitor_thread() {

    }
}
