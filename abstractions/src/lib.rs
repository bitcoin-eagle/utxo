trait Service {
    fn start() -> Result<(), Error>
    fn stop() -> Result<(), Error>
}