#[macro_export]
macro_rules! message_handler {
    ($rx:expr, $($pattern:pat => $body:expr)*) => {{
        match $rx.recv() {
            Ok(recv) => {
                if let Ok(msg) = bincode::deserialize::<Message>(&recv.0) {
                    match msg.op() {
                        $(
                            $pattern => {
                                let msg = msg;
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
