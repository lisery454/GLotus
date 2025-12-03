use crate::event::event::AppEvent;

pub struct AppEventQueue {
    queue: Vec<AppEvent>,
}

impl AppEventQueue {
    pub fn new() -> Self {
        AppEventQueue { queue: Vec::new() }
    }

    pub fn push(&mut self, e: AppEvent) {
        self.queue.push(e);
    }

    pub fn pop(&mut self) -> Option<AppEvent> {
        self.queue.pop()
    }

    pub fn drain(&mut self) -> Vec<AppEvent> {
        self.queue.drain(..).collect()
    }
}
