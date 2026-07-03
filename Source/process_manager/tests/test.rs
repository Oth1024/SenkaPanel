#[cfg(test)]
mod tests {
    #[tokio::test]
    /// 测试下发单次执行的任务
    async fn test_raw_task() {
        use process_manager::{get_process_manager, ProcessHandle};
        use tokio::io::AsyncReadExt;

        let manager = get_process_manager();

        // 启动一个打印 hello world 的子进程
        let id = manager.start(
            "cmd".to_string(),
            vec!["/c".to_string(), "echo hello world".to_string()],
            false,
            "test".to_string(),
        )
        .unwrap();

        // 等待子进程退出
        let child = {
            let mut info = manager.process_infos.get_mut(&id).unwrap();
            std::mem::replace(&mut info.process, ProcessHandle::None)
        };
        if let ProcessHandle::Managed(mut child) = child {
            let _ = child.wait().await;
        }

        // 读取子进程的 stdout
        let mut output = Vec::new();
        if let Some(mut info) = manager.process_infos.get_mut(&id) {
            if let Some(ref mut stdout) = info.stdout {
                tokio::time::timeout(
                    std::time::Duration::from_secs(3),
                    stdout.read_to_end(&mut output),
                )
                .await
                .unwrap()
                .unwrap();
            }
        }

        println!("子进程 stdout: {:?}", String::from_utf8_lossy(&output));
    }

    #[tokio::test]
    /// 测试进程监控的开启和关闭
    /// 测试子进程的自动重启
    async fn test_monitor_thread() {

    }
}