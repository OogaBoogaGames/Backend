#[macro_export]
macro_rules! message_handler {
    ($rx:expr, $($pattern:pat => $body:expr),*) => {{
        match $rx.recv() {
            Ok(recv) => {
                if let Ok(msg) = bincode::deserialize::<Message>(&recv.0) {
                    let op = msg.op().clone();

                    match op {
                        $(
                            $pattern => {
                                $body(msg);
                            },
                        )*
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }}
}
