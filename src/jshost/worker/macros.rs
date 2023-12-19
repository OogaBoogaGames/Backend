#[macro_export]
macro_rules! message_handler {
    ($rx:expr, $($pattern:pat => $body:expr),*) => {{
        while let Ok(recv) = $rx.recv() {
            if let Ok(msg) = bincode::deserialize::<Message>(&recv.0) {
                let op = msg.op().clone();

                match op {
                    $(
                        $pattern => {
                            if !$body(msg) {
                                break;
                            }
                        },
                    )*
                    _ => {}
                }
            }
        }
    }}
}
