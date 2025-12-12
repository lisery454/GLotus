use crate::event::event::AppEvent;

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
}
