use super::Snowflake;

pub struct SnowflakeGenerator {
    last_snowflake: Snowflake,
}

impl SnowflakeGenerator {
    pub fn new(worker_id: u64) -> SnowflakeGenerator {
        SnowflakeGenerator {
            last_snowflake: Snowflake::new(worker_id),
        }
    }

    pub fn next(&mut self) -> &Snowflake {
        self.last_snowflake = self.last_snowflake.next();
        return &self.last_snowflake;
    }

    /// Get the current timestamp in seconds since the epoch (1970-01-01 00:00:00 UTC).
    ///
    /// # Returns
    /// The current timestamp in seconds
    pub fn get_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs()
    }

    /// Wait for the next second and return the timestamp
    ///
    /// # Arguments
    /// * `current_timestamp` - The current timestamp in seconds
    ///
    /// # Returns
    /// The timestamp of the next second
    pub fn wait_next_timestamp(last_timestamp: u64) -> u64 {
        let mut timestamp = SnowflakeGenerator::get_timestamp();
        while timestamp <= last_timestamp {
            std::thread::sleep(std::time::Duration::from_millis(1));
            timestamp = SnowflakeGenerator::get_timestamp();
        }
        timestamp
    }
}
