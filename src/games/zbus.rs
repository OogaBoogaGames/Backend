use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "games.oogabooga.JsHost.JsInterface",
    default_service = "games.oogabooga.JsHost",
    default_path = "/games/oogabooga/JsHost"
)]
trait JsInterface {
    async fn create_game(&self, id: u64) -> zbus::Result<()>;
}
