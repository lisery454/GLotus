use super::AppEvent;

/// 引用事件队列
pub struct AppEventQueue {
    queue: Vec<AppEvent>,
}

impl AppEventQueue {
    /// 创建空的队列
    pub fn new() -> Self {
        AppEventQueue { queue: Vec::new() }
    }

    /// 增加事件
    pub fn push(&mut self, e: AppEvent) {
        self.queue.push(e);
    }

    /// 读取事件
    pub fn pop(&mut self) -> Option<AppEvent> {
        self.queue.pop()
    }

    /// 取出所有的事件
    pub fn drain(&mut self) -> Vec<AppEvent> {
        self.queue.drain(..).collect()
    }

    /// 仅返回引用，不删除数据
    pub fn all_events(&self) -> &[AppEvent] {
        &self.queue
    }

    /// 在 清空所有事件
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}
