use std::path::PathBuf;

use notify::Config;
use notify::Event;
use notify::EventKind;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Result;
use notify::Watcher;

use iced::futures::SinkExt;
use iced::subscription::channel;
use iced::Subscription;

use flume::bounded;
use flume::Receiver;

use crate::StyleSheet;

/// Utility method to create a recommended watcher that sends messages on an async channel.
fn async_watcher() -> Result<(RecommendedWatcher, Receiver<Result<Event>>)> {
    let (tx, rx) = bounded(1);

    Ok((
        RecommendedWatcher::new(move |res| tx.send(res).unwrap(), Config::default())?,
        rx,
    ))
}

/// An iced subscription that triggers when the stylesheet at the given path has been modified.
///
/// It will send the `on_reload` message with the new stylesheet information if parsed successfully.
pub fn hot_reload<
    P: Into<PathBuf>,
    Message: Send + 'static,
    M: (Fn(StyleSheet) -> Message) + Send + 'static,
>(
    path: P,
    on_reload: M,
) -> Subscription<Message> {
    let path = path.into();

    channel(
        "icee::subscription::hot_reload",
        1,
        |mut output| async move {
            let (mut watcher, rx) = async_watcher().unwrap();

            watcher
                .watch(path.as_ref(), RecursiveMode::NonRecursive)
                .expect("Failed to setup watcher for hot reload!");

            loop {
                while let Ok(result) = rx.recv_async().await {
                    if let Ok(result) = result {
                        if matches!(result.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                            for _ in 0..5 {
                                match StyleSheet::load(&path) {
                                    Ok(stylesheet) => {
                                        let _ = output.send(on_reload(stylesheet)).await;
                                        break;
                                    }
                                    Err(e) => {
                                        #[cfg(feature = "debug")]
                                        println!("Failed to hot reload stylesheet: {:?}", e);
                                        #[cfg(not(feature = "debug"))]
                                        let _ = e;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}
