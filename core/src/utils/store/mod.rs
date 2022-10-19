type StoreMiddleware<T, M> = Box<dyn (Fn(T, M) -> M) + Send + Sync + 'static>;
type StoreReducer<T, M> = Box<dyn (Fn(T, M) -> T) + Send + Sync + 'static>;
type TerminalChecker<M> = Box<dyn (Fn(M) -> bool) + Send + Sync + 'static>;

pub struct Store<T, M> 
where T: Copy + Clone + Default + PartialEq, M: Copy + Clone + PartialEq {
    state: T,
    middleware: Vec<StoreMiddleware<T, M>>,
    history: Vec<T>,
    reducer: StoreReducer<T, M>,
    is_terminal: TerminalChecker<M>,
}

impl<T, M> Store<T, M>
where T: Copy + Clone + Default + PartialEq, M: Copy + Clone + PartialEq {
    pub fn new(reducer: StoreReducer<T, M>, is_terminal: TerminalChecker<M>) -> Self {
        Self {
            state: T::default(),
            middleware: Vec::new(),
            history: Vec::new(),
            reducer,
            is_terminal,
        }
    }

    pub fn dispatch(&mut self, msg: M) {
        if (self.is_terminal)(msg) {
            return
        }
        else {
            let last = self.state;
            self.state = (self.reducer)(self.state, msg);
            if last != self.state {
                self.history.push(last);
            }; 
            self.run_middlewares(msg)
        }
    }

    pub fn view(&mut self) -> T {
        self.state.clone()
    }

    pub fn backtrace(&self, steps: usize) -> Vec<T> {
        let mut acc = Vec::new();
        for t in self.history.iter().rev().take(steps) {
            acc.push(*t);
        }
        acc
    }

    pub fn use_middleware(
        &mut self,
        mw: StoreMiddleware<T, M>
    ) {
        self.middleware.push(mw);
    }

    fn run_middlewares(&mut self, msg: M) {
        let mut results = Vec::new();
        for mw in self.middleware.iter() {
            results.push((mw)(self.state, msg));
        }

        for m in results.iter() {
            self.dispatch(*m);
        }
    }
}