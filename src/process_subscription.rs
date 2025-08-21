use iced::futures::sink::SinkExt;
use iced::{futures::channel::mpsc, Subscription};

use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::SplitStream;
use tokio_stream::StreamExt;

pub enum Event {
    Ready(usize, mpsc::Sender<PSubInput>),
    GotLogs(usize, String),
    ProcessEnded(usize),
}

enum PSubState {
    Starting,
    Ready(mpsc::Receiver<PSubInput>),
}

#[derive(Debug)]
pub enum PSubInput {
    Terminate,
    ReadInput,
}

pub fn get_psub(idx: usize, cmd_builder: Option<crate::games::Command>) -> Subscription<Event> {
    iced::Subscription::run_with_id(idx, {
        iced::stream::channel(100, move |mut output: mpsc::Sender<Event>| {
            async move {
                let mut state = PSubState::Starting;
                let mut proc = cmd_builder.clone().unwrap().run().unwrap();
                let stdout = proc.stdout.take().unwrap();
                let stderr = proc.stderr.take().unwrap();

                // Wrap them up and merge them.
                let stdout = SplitStream::new(tokio::io::BufReader::new(stdout).split(b'\n'));
                let stderr = SplitStream::new(tokio::io::BufReader::new(stderr).split(b'\n'));
                let merged = StreamExt::merge(stdout, stderr).map(|a| {
                    a.map(|b| {
                        let mut o = std::collections::VecDeque::from(b);
                        o.push_back(b'\n');
                        o
                    })
                });
                // let proc_out = proc.stdout.take().unwrap();
                // let mut inner_proc_out = tokio::fs::File::from_std(proc_out);
                let mut inner_proc_out = tokio_util::io::StreamReader::new(merged);
                loop {
                    match &mut state {
                        PSubState::Starting => {
                            let (sender, receiver) = mpsc::channel(100);
                            if let Err(e) = output.send(Event::Ready(idx, sender)).await {
                                log::error!(
                                "Couldn't send back the sender : {e}. This will most likely panic"
                            );
                            }
                            state = PSubState::Ready(receiver);
                        }
                        PSubState::Ready(receiver) => {
                            use tokio::io::AsyncRead;

                            use iced::futures::StreamExt;
                            let input = receiver.select_next_some().await;
                            match input {
                                PSubInput::Terminate => {
                                    loop {
                                        let mut ubuf: [u8; 1024] = [0; 1024];
                                        let mut buf = tokio::io::ReadBuf::new(&mut ubuf);
                                        let status = std::pin::Pin::new(&mut inner_proc_out)
                                            .poll_read(
                                                &mut std::task::Context::from_waker(
                                                    &noop_waker::noop_waker(),
                                                ),
                                                &mut buf,
                                            );
                                        match status {
                                            std::task::Poll::Ready(Ok(())) => {
                                                let ct = buf.filled();
                                                if ct.is_empty() {
                                                    break;
                                                }
                                                if let Err(e) = output
                                                    .send(Event::GotLogs(
                                                        idx,
                                                        String::from_utf8_lossy(ct).to_string(),
                                                    ))
                                                    .await
                                                {
                                                    log::error!(
                                                        "Unable to send data from psub : {e}"
                                                    );
                                                }
                                            }
                                            std::task::Poll::Ready(Err(e)) => {
                                                log::error!("unmanaged process read error : {e}");
                                                break;
                                            }
                                            std::task::Poll::Pending => break,
                                        }
                                    }
                                    if let Err(e) = proc.kill().await {
                                        log::error!("Unable to kill process : {e}");
                                    }
                                    if let Err(e) = output.send(Event::ProcessEnded(idx)).await {
                                        log::error!("Unable to send data from psub : {e}");
                                    }
                                }
                                PSubInput::ReadInput => {
                                    let mut logs = String::new();
                                    loop {
                                        let mut ubuf: [u8; 1024] = [0; 1024];
                                        let mut buf = tokio::io::ReadBuf::new(&mut ubuf);
                                        let status = std::pin::Pin::new(&mut inner_proc_out)
                                            .poll_read(
                                                &mut std::task::Context::from_waker(
                                                    &noop_waker::noop_waker(),
                                                ),
                                                &mut buf,
                                            );
                                        match status {
                                            std::task::Poll::Ready(Ok(())) => {
                                                let ct = buf.filled();
                                                if ct.is_empty() {
                                                    break;
                                                }
                                                logs += &String::from_utf8_lossy(ct);
                                            }
                                            std::task::Poll::Ready(Err(e)) => {
                                                log::error!("unmanaged process read error : {e}");
                                                break;
                                            }
                                            std::task::Poll::Pending => break,
                                        }
                                    }
                                    if !logs.is_empty() {
                                        if let Err(e) = output.send(Event::GotLogs(idx, logs)).await
                                        {
                                            log::error!("Unable to send data from psub : {e}");
                                        }
                                    }
                                    if let Ok(Some(_exit_status)) = proc.try_wait() {
                                        if let Err(e) = output.send(Event::ProcessEnded(idx)).await
                                        {
                                            log::error!("Unable to send data from psub : {e}");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    })
}

struct Runner<I, F, S, T>
where
    F: FnOnce(iced::advanced::subscription::EventStream) -> S,
    S: tokio_stream::Stream<Item = T>,
{
    id: I,
    spawn: F,
}

impl<I, F, S, T> iced::advanced::subscription::Recipe for Runner<I, F, S, T>
where
    I: std::hash::Hash + 'static,
    F: FnOnce(iced::advanced::subscription::EventStream) -> S,
    S: tokio_stream::Stream<Item = T> + iced::advanced::graphics::futures::MaybeSend + 'static,
{
    type Output = T;

    fn hash(&self, state: &mut iced::advanced::subscription::Hasher) {
        std::hash::Hash::hash(&std::any::TypeId::of::<I>(), state);
        self.id.hash(state);
    }

    fn stream(
        self: Box<Self>,
        input: iced::advanced::subscription::EventStream,
    ) -> iced::futures::stream::BoxStream<'static, Self::Output> {
        iced::futures::stream::StreamExt::boxed((self.spawn)(input))
    }
}

pub trait RunWithId<T> {
    fn run_with_id<I, S>(id: I, stream: S) -> Self
    where
        I: std::hash::Hash + 'static,
        S: tokio_stream::Stream<Item = T> + iced::advanced::graphics::futures::MaybeSend + 'static,
        T: 'static;
}

impl<T> RunWithId<T> for Subscription<T> {
    fn run_with_id<I, S>(id: I, stream: S) -> Subscription<T>
    where
        I: std::hash::Hash + 'static,
        S: tokio_stream::Stream<Item = T> + iced::advanced::graphics::futures::MaybeSend + 'static,
        T: 'static,
    {
        iced::advanced::subscription::from_recipe(Runner {
            id,
            spawn: move |_| stream,
        })
    }
}
