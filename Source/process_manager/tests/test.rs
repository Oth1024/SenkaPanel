#[cfg(test)]
mod tests {
    #[tokio::test]
    /// 测试下发单次执行的任务——等待进程结束后读取全部输出
    async fn test_raw_task() {
        use process_manager::{get_process_manager, ProcessHandle};
        use tokio::io::AsyncReadExt;

        let manager = get_process_manager();

        let id = manager.start(
            "cmd".to_string(),
            vec!["/c".to_string(), "echo hello world".to_string()],
            false,
            "test".to_string(),
        )
        .unwrap();

        // RefMut 不能跨 .await，所以先移出 Child，wait 完再放回去
        let child = {
            let mut info = manager.process_infos.get_mut(&id).unwrap();
            std::mem::replace(&mut info.process, ProcessHandle::None)
        };
        if let ProcessHandle::Managed(mut child) = child {
            let _ = child.wait().await;
            // wait 完放回 ProcessInfo，保持结构完整性
            if let Some(mut info) = manager.process_infos.get_mut(&id) {
                info.process = ProcessHandle::Managed(child);
            }
        }

        // 进程已退出，管道 EOF，read_to_end 正常返回
        let mut output = Vec::new();
        if let Some(mut info) = manager.process_infos.get_mut(&id) {
            if let Some(ref mut stdout) = info.stdout {
                stdout.read_to_end(&mut output).await.unwrap();
            }
        }

        println!("子进程输出: {:?}", String::from_utf8_lossy(&output));
        assert!(String::from_utf8_lossy(&output).contains("hello world"));
    }

    #[tokio::test]
    /// 测试进程运行中实时捕获输出——进程循环输出，不等待退出
    async fn test_live_output() {
        use process_manager::{get_process_manager, ProcessHandle};
        use tokio::io::AsyncBufReadExt;

        let manager = get_process_manager();

        // 启动一个循环输出进程（Windows 每 2 秒输出一行，共 3 次）
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

        // 取出 stdout 的所有权（BufReader 需要所有权）
        let stdout = {
            let mut info = manager.process_infos.get_mut(&id).unwrap();
            info.stdout.take()
        };

        if let Some(stdout) = stdout {
            let mut reader = tokio::io::BufReader::new(stdout).lines();
            let mut count = 0;

            // 边运行边逐行读，读到 3 行就停
            while let Ok(Some(line)) =
                tokio::time::timeout(std::time::Duration::from_secs(10), reader.next_line())
                    .await
                    .unwrap_or(Ok(None))
            {
                println!("实时捕获: {}", line);
                count += 1;
                if count >= 3 {
                    break;
                }
            }

            assert_eq!(count, 3);
        }

        // 清理进程，同样移出、kill、放回
        let child = {
            let mut info = manager.process_infos.get_mut(&id).unwrap();
            std::mem::replace(&mut info.process, ProcessHandle::None)
        };
        if let ProcessHandle::Managed(mut child) = child {
            let _ = child.kill().await;
            if let Some(mut info) = manager.process_infos.get_mut(&id) {
                info.process = ProcessHandle::Managed(child);
                info.stdin = None;
                info.stdout = None;
                info.stderr = None;
            }
        }
    }

    #[tokio::test]
    /// 测试进程监控的开启和关闭
    /// 测试子进程的自动重启
    async fn test_monitor_thread() {

    }
}