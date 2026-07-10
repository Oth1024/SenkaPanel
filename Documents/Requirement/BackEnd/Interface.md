# 接口设计

本文档定义各业务模块对外暴露的操控接口。每个模块仅暴露直接操控需求，不包含网络通信逻辑。

---

## 1. HTTP服务基础框架模块

> 该模块由 Rocket 框架原生能力承接，路由、中间件、请求守卫等能力无需额外定义接口。
> 各业务模块的路由 Handler 函数定义见各模块自身。

（暂无额外接口）

---

## 2. 服务器状态监控模块

> 待实现

---

## 3. 用户管理模块

> 待实现

---

## 4. 用户权限管理模块

> 待实现

---

## 5. 子进程管理模块 (ProcessManager)

**模块路径**: `Source/process_manager`

### 5.1 公共类型定义

#### ProcessStatus — 进程状态枚举

```rust
pub enum ProcessStatus {
    None,            // 进程未创建或创建失败，此时接受线程创建
    Running,         // 线程创建并正在运行，此时仅接受进程状态更新
    Exited(i32),     // 进程正常或异常退出，携带退出码，此时接受进程重启
    Stopped,         // 进程被手动中止，此时不接受进程重启
}
```

#### ProcessHandle — 进程句柄枚举

```rust
pub enum ProcessHandle {
    None,                              // 无子进程
    Managed(Child),                    // 由本进程创建的子进程句柄，支持输入输出控制、状态管理
    Dued(Pid),                         // 通过其他方式捕获的进程句柄，仅支持状态管理
}
```

#### ProcessInfo — 进程信息结构体

```rust
pub struct ProcessInfo {
    pub status: ProcessStatus,         // 进程当前状态
    pub command: String,               // 启动命令
    pub args: Vec<String>,             // 启动参数
    pub auto_restart: bool,            // 是否启用异常自动重启
    pub creator: String,               // 创建者标识
    pub created_time: DateTime<Utc>,   // 创建时间
    // 以下为私有字段，仅通过方法访问
    // process: ProcessHandle,         // 进程句柄
    // stdin_tx, stdout_rx, stderr_rx // 输入输出管道
}
```

#### ProcessManager — 进程管理器结构体

```rust
pub struct ProcessManager {
    pub process_infos: DashMap<u32, ProcessInfo>,  // 进程映射表，key为进程ID
    // 以下为私有字段
    // check_interval: u32,            // 监控轮询间隔(秒)
    // process_index: AtomicU32,       // 进程ID自增计数器
    // monitor_thread, stop_signal     // 监控线程控制
}
```

### 5.2 ProcessInfo 方法

|方法签名|说明|
|----|----|
|`pub fn new(command: String, args: Vec<String>, auto_restart: bool, creator: String) -> Self`|构造一个新的 ProcessInfo。创建时初始化 stdin/stdout/stderr 消息通道，状态为 None。此方法不会启动进程，需由 ProcessManager 的监控线程在轮询时自动创建|
|`pub fn from_dued(pid: u32, command: String, args: Vec<String>, auto_restart: bool, creator: String) -> Self`|通过外部已存在的进程PID构建 ProcessInfo。适用于管理已在运行的进程，不支持输入输出控制。通过 sysinfo 检查进程是否存在|
|`pub fn get_stdout_recv(&self) -> Option<broadcast::Receiver<String>>`|获取标准输出的接收通道。返回一个新的 broadcast::Receiver 订阅，通过该通道可实时读取进程的 stdout 输出行|
|`pub fn get_stderr_recv(&self) -> Option<broadcast::Receiver<String>>`|获取标准错误的接收通道。返回一个新的 broadcast::Receiver 订阅，通过该通道可实时读取进程的 stderr 输出行|
|`pub fn get_stdin_sender(&self) -> Option<mpsc::Sender<String>>`|获取标准输入的发送通道。通过该通道可以向进程的 stdin 写入数据。仅在由本管理器创建的子进程(Managed)中可用|

### 5.3 ProcessManager 方法

#### 获取全局实例

```rust
pub fn get_process_manager() -> &'static ProcessManager
```
返回全局唯一的 ProcessManager 单例。

#### 监控线程管理

|方法签名|说明|
|----|----|
|`pub async fn start_monitor(&self)`|启动后台监控线程。若监控线程已存在且未结束则不重复创建。监控线程以 check_interval 为间隔轮询所有进程，执行状态检查、IO 分发、自动重启等逻辑|
|`pub fn stop_monitor(&self)`|停止监控线程。设置停止信号并 abort 线程|

#### 进程生命周期管理

|方法签名|说明|
|----|----|
|`pub fn start(&self, command: String, args: Vec<String>, auto_restart: bool, creator: String) -> Result<u32, SenkaError>`|注册一个新进程。返回唯一的进程ID。进程不会立即启动，而是先以 None 状态存入映射表，由监控线程在下一轮轮询时自动创建并启动。若进程创建失败，监控线程会持续重试|
|`pub fn kill(&self, id: u32)`|中止指定ID的进程。将状态设为 Stopped（阻止自动重启），并通过 start_kill() 发送终止信号|
|`pub fn restart_by_id(&self, id: u32) -> Result<(), SenkaError>`|通过进程ID重启进程。先异步杀掉旧进程，再重新 spawn。若ID不存在则返回错误|
|`pub fn restart_entry(&self, entry: &mut ProcessInfo) -> Result<(), SenkaError>`|直接操作 ProcessInfo 重启进程。供监控轮询等已持有引用的场景调用，避免二次查表|
|`pub fn remove(&self, id: u32) -> Result<(), SenkaError>`|从映射表中移除进程。仅允许移除非 Running 状态的进程，否则返回错误|

---

## 6. 子任务管理模块

> 待实现

---

## 7. 网络管理模块（Frpc管理）

> 待实现
