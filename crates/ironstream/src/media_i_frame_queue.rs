// FILE: media_i_frame_queue.rs
// occt: Media_IFrameQueue

/// Frame queue interface.
pub trait IFrameQueue {
    fn enqueue(&mut self, frame_id: u32) -> bool;
    fn dequeue(&mut self) -> Option<u32>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn clear(&mut self);
}

/// Simple frame queue implementation.
#[derive(Clone, Debug, Default)]
pub struct FrameQueue {
    frames: Vec<u32>,
}

impl FrameQueue {
    pub fn new() -> Self { Self { frames: Vec::new() } }
}

impl IFrameQueue for FrameQueue {
    fn enqueue(&mut self, frame_id: u32) -> bool {
        self.frames.push(frame_id);
        true
    }
    fn dequeue(&mut self) -> Option<u32> {
        if self.frames.is_empty() { None } else { Some(self.frames.remove(0)) }
    }
    fn is_empty(&self) -> bool { self.frames.is_empty() }
    fn size(&self) -> usize { self.frames.len() }
    fn clear(&mut self) { self.frames.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_queue() {
        let mut q = FrameQueue::new();
        assert!(q.is_empty());
        q.enqueue(1);
        assert_eq!(q.dequeue(), Some(1));
    }
}
