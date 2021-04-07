use std::{cell::RefCell, collections::HashMap, fmt::Display, hash::Hash, mem, rc::Rc};

pub trait GetMemoKey<Key> {
    fn memo_key(&self) -> Key;
}

impl<T> GetMemoKey<T> for T
where
    T: Hash + Eq + Clone,
{
    fn memo_key(&self) -> T {
        self.clone()
    }
}

pub struct MemoFrames<Input, Output> {
    previous: HashMap<Input, Vec<Output>>,
    next: HashMap<Input, Vec<Output>>,
}

impl<Input, Output> Default for MemoFrames<Input, Output> {
    fn default() -> Self {
        Self {
            previous: HashMap::default(),
            next: HashMap::default(),
        }
    }
}

impl<Input, Output> MemoFrames<Input, Output> {
    fn next_frame(&mut self) {
        self.previous = mem::take(&mut self.next);
    }
}

#[derive(Clone)]
pub struct Memo<Input, Output> {
    frames: Rc<RefCell<MemoFrames<Input, Output>>>,
}

impl<Input, Output> Memo<Input, Output>
where
    Input: 'static + Hash + Eq + Clone + Display,
    Output: 'static + Clone,
{
    pub fn initialize() -> Self {
        let new = Self {
            frames: Rc::new(RefCell::new(MemoFrames::default())),
        };

        FRAME_LISTENERS.with(|frame_listeners| {
            frame_listeners.borrow_mut().push(Box::new(new.clone()));
        });

        new
    }

    pub fn memo<Args>(&self, f: impl Fn(Args) -> Output, input: Args) -> Output
    where
        Args: GetMemoKey<Input>,
    {
        let memo_key = input.memo_key();

        {
            let mut frames = self.frames.borrow_mut();

            if let Some(outputs) = frames.previous.get_mut(&memo_key) {
                if let Some(output) = outputs.pop() {
                    frames
                        .next
                        .entry(memo_key)
                        .or_default()
                        .push(output.clone());

                    return output;
                }
            }
        }

        let output = f(input);

        // TODO: Reset all new states to initial value.

        self.frames
            .borrow_mut()
            .next
            .entry(memo_key)
            .or_default()
            .push(output.clone());

        output
    }
}

impl<Input, Output> FrameListener for Memo<Input, Output> {
    fn next_frame(&self) {
        self.frames.borrow_mut().next_frame();
    }
}

pub fn next_frame() {
    FRAME_LISTENERS.with(|frame_listeners| {
        for frame_listener in frame_listeners.borrow_mut().iter() {
            frame_listener.next_frame();
        }
    });
}

pub trait FrameListener {
    fn next_frame(&self);
}

thread_local!(
    static FRAME_LISTENERS: Rc<RefCell<Vec<Box<dyn FrameListener>>>> =
        Rc::new(RefCell::new(Vec::new()));
);
