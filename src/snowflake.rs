use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Eq)]
pub struct Snowflake {
    /// The worker ID of the snowflake.
    /// This is a unique identifier for the host or thread that created the snowflake.
    pub worker_id: u64,
    /// The sequence number of the snowflake.
    /// This increments every time the snowflake is created within the same second.
    /// This will automatically reset to 0 when the timestamp changes or
    /// when the sequence overflows (2^64 - 1).
    pub sequence: u64,
    /// The timestamp of the last snowflake creation in seconds since the epoch (1970-01-01 00:00:00 UTC).
    pub last_timestamp: u64,
}

impl Snowflake {
    pub fn new(worker_id: u64) -> Snowflake {
        Snowflake {
            worker_id,
            sequence: 0,
            last_timestamp: Snowflake::get_timestamp(),
        }
    }

    /// Generate a new snowflake ID
    /// This function will return a new snowflake ID every time it is called
    /// with the same worker ID.
    /// If the timestamp is the same as the previous call, the sequence number will be incremented.
    ///
    /// # Example
    /// ```rust
    /// use rusty_snowflake::Snowflake;
    ///
    /// let mut snowflake = Snowflake::new(1);
    /// println!("{}", snowflake.next());
    /// println!("{}", snowflake.next());
    /// ```
    ///
    pub fn next(&mut self) -> u64 {
        let mut timestamp = Snowflake::get_timestamp();

        if timestamp < self.last_timestamp {
            timestamp = self.last_timestamp; // Reset timestamp
        } else if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & 0xFFFF; // Increment sequence
            if self.sequence == 0 {
                timestamp = self.wait_next_sec(timestamp);
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        self.to_id()
    }

    /// Convert a snowflake ID into a u64
    ///
    /// # Example
    ///
    /// ```rust
    /// use rusty_snowflake::Snowflake;
    ///
    /// let snowflake = Snowflake::new(1); // Create
    ///
    /// let id = snowflake.to_id();
    ///
    /// let parsed = Snowflake::parse(id);
    ///
    /// assert_eq!(snowflake, parsed);
    /// ```
    pub fn to_id(&self) -> u64 {
        (self.last_timestamp << 22) | (self.worker_id << 12) | self.sequence
    }

    /// Parse a snowflake ID into a `Snowflake`
    /// # Example
    /// ```rust
    /// use rusty_snowflake::Snowflake;
    ///
    /// let mut snowflake = Snowflake::new(1);
    ///
    /// let id = snowflake.next();
    /// let parsed = Snowflake::parse(id);
    ///
    /// assert_eq!(snowflake, parsed);
    /// ```
    pub fn parse(id: u64) -> Snowflake {
        let timestamp = (id >> 22) & 0x1FFFFFFFFFF;
        let worker_id = (id >> 12) & 0x3FF;
        let sequence = id & 0xFFF;

        Snowflake {
            worker_id,
            sequence,
            last_timestamp: timestamp,
        }
    }

    /// Get the current timestamp in seconds since the epoch (1970-01-01 00:00:00 UTC).
    ///
    /// # Returns
    /// The current timestamp in seconds
    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
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
    fn wait_next_sec(&self, current_timestamp: u64) -> u64 {
        let mut timestamp = Snowflake::get_timestamp();
        while timestamp <= current_timestamp {
            timestamp = Snowflake::get_timestamp();
        }
        timestamp
    }
}

impl std::fmt::Display for Snowflake {
    /// Display the Snowflake
    /// # Example
    /// ```rust
    /// use rusty_snowflake::Snowflake;
    ///
    /// let mut snowflake = Snowflake::new(1);
    /// println!("{}", snowflake); // u64 ID
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_id())
    }
}

impl From<u64> for Snowflake {
    fn from(id: u64) -> Snowflake {
        Snowflake::parse(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_id() {
        const IDS: [u64; 6] = [42, 69, 420, 123, 777, 1000];

        for i in 0..IDS.len() {
            let snowflake = Snowflake::new(IDS[i]);
            assert_eq!(snowflake.worker_id, IDS[i]);
        }
    }

    #[test]
    fn test_sequence() {
        let mut snowflake = Snowflake::new(1);

        for i in 1..10 {
            let id = snowflake.next();
            assert_eq!(Snowflake::parse(id).sequence, i);
        }
    }

    #[test]
    fn test_timestamp() {
        let snowflake = Snowflake::new(1);
        assert_eq!(snowflake.last_timestamp, Snowflake::get_timestamp());
    }

    #[test]
    fn test_snowflake_parse() {
        let snowflake = Snowflake::new(1);
        let id = snowflake.to_id();
        let parsed = Snowflake::parse(id);
        assert_eq!(snowflake, parsed, "Snowflake ID didn't parse correctly");
    }
    #[test]
    fn test_next_timestamp_change() {
        let mut snowflake = Snowflake {
            worker_id: 1,
            last_timestamp: 100,
            sequence: 0,
        };

        snowflake.next();

        // Assert that timestamp is updated to last_timestamp
        assert_eq!(
            snowflake.last_timestamp,
            Snowflake::get_timestamp(),
            "Timestamp didn't update correctly"
        );
    }

    #[test]
    fn test_next_sequence_change() {
        let mut snowflake = Snowflake {
            worker_id: 1,
            last_timestamp: Snowflake::get_timestamp(),
            sequence: 0,
        };

        snowflake.next();

        // Assert that sequence is incremented
        assert_eq!(snowflake.sequence, 1);
    }

    #[test]
    fn test_next_when_sequence_overflows() {
        let time = Snowflake::get_timestamp();

        let mut snowflake = Snowflake {
            worker_id: 1,
            sequence: 0xFFFF, // Maximum sequence value
            last_timestamp: time,
        };

        snowflake.next();

        // Assert that sequence is reset to 0
        assert_eq!(snowflake.sequence, 0);
        assert!(snowflake.last_timestamp > time);

        // Assert that wait_next_sec method is called
        // Add appropriate assertions based on your implementation
    }

    #[test]
    fn test_next_when_timestamp_is_greater_than_last_timestamp() {
        let mut snowflake = Snowflake {
            worker_id: 1,
            last_timestamp: Snowflake::get_timestamp() + 100,
            sequence: 0,
        };

        snowflake.next();

        // Assert that sequence is reset to 0
        assert_eq!(snowflake.sequence, 0);

        // Add appropriate assertions based on your implementation
    }

    #[test]
    fn test_snowflake_to_id() {
        let snowflake = Snowflake::new(1);
        let id = snowflake.to_id();

        assert_eq!(
            snowflake,
            Snowflake::parse(id),
            "Snowflake ID didn't parse correctly"
        );
    }

    #[test]
    fn test_snowflake_wait_next_sec() {
        let snowflake = Snowflake::new(1);
        let id = snowflake.last_timestamp;
        let next = snowflake.wait_next_sec(Snowflake::get_timestamp());
        assert!(
            next > id,
            "Snowflake.wait_next_sec didn't return a new timestamp"
        );
    }

    #[test]
    fn test_snowflake_to_string() {
        let snowflake = Snowflake::new(1);
        assert_eq!(snowflake.to_string(), format!("{}", snowflake.to_id()));
    }

    #[test]
    fn test_from_u64() {
        let snowflake: Snowflake = Snowflake::from(1);
        let id = snowflake.to_id();
        let from_u64: Snowflake = id.into();

        assert_eq!(snowflake, from_u64);

        assert_eq!(Snowflake::from(1), Snowflake::parse(id));
    }
}
