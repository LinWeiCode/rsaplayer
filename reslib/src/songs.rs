pub struct Songs
{
    pub path_name:String,
}

impl Songs {
    /// Returns the play of this [`Songs`].
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn play(&self)
    {
        println!("{}--播放中",self.path_name);
        let file=std::fs::File::open(&self.path_name).unwrap();
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink=rodio::Sink::try_new(&stream_handle).unwrap();
        let file_source=rodio::Decoder::new(file).unwrap();
        sink.append(file_source);
        sink.sleep_until_end();
    }
}