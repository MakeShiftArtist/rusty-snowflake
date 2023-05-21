use crate::SnowflakeGenerator;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Snowflake {
    /// The worker ID of the snowflake.
    /// This is a unique identifier for the host or thread that created the snowflake.
    pub worker_id: u64,
    /// The sequence number of the snowflake.
    /// This increments every time the snowflake is created within the same second.
    /// This will automatically reset to 0 when the timestamp changes or
    /// when the sequence overflows (2^16 - 1).
    pub sequence: u64,
    /// The timestamp of the snowflake creation in seconds since the epoch (1970-01-01 00:00:00 UTC).
    pub timestamp: u64,
}

impl Snowflake {
    /// Create a new snowflake with the given worker ID
    ///
    /// # Arguments
    /// * `worker_id` - The worker ID of the snowflake
    ///
    /// # Returns
    /// A new `Snowflake`
    ///
    /// # Example
    /// ```rust
    /// use rusty_snowflake::Snowflake;
    ///
    /// let snowflake = Snowflake::new(1);
    /// ```
    pub fn new(worker_id: u64) -> Snowflake {
        Snowflake {
            worker_id,
            sequence: 0,
            timestamp: SnowflakeGenerator::get_timestamp(),
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
    pub fn next(&self) -> Snowflake {
        let mut timestamp = SnowflakeGenerator::get_timestamp();
        let mut sequence = self.sequence;

        if timestamp < self.timestamp {
            timestamp = self.timestamp; // Reset timestamp
        } else if timestamp == self.timestamp {
            sequence = (sequence + 1) & 0xFFFF; // Increment sequence
            if sequence == 0 {
                timestamp = SnowflakeGenerator::wait_next_timestamp(timestamp); // Update timestamp when sequence overflows
            }
        } else {
            sequence = 0; // Reset sequence because timestamp changed
        }

        Snowflake {
            worker_id: self.worker_id,
            sequence,
            timestamp,
        }
    }

    /// Convert a Snowflake ID into a u64 id
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
        (self.timestamp << 22) | (self.worker_id << 12) | self.sequence
    }

    /// Parse a snowflake ID into a `Snowflake`
    /// # Example
    /// ```rust
    /// use rusty_snowflake::Snowflake;
    ///
    /// let snowflake = Snowflake::new(1);
    ///
    /// let id = snowflake.to_id();
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
            timestamp: timestamp,
        }
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

impl Ord for Snowflake {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_id().cmp(&other.to_id())
    }
}

impl PartialOrd for Snowflake {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
            snowflake = snowflake.next();
            let id = snowflake.to_id();
            assert_eq!(Snowflake::parse(id).sequence, i);
        }
    }

    #[test]
    fn test_timestamp() {
        let snowflake = Snowflake::new(1);
        assert_eq!(snowflake.timestamp, SnowflakeGenerator::get_timestamp());
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
        let snowflake = Snowflake {
            worker_id: 1,
            timestamp: 100,
            sequence: 0,
        };

        let snowflake = snowflake.next();

        // Assert that timestamp is updated to timestamp
        assert_eq!(
            snowflake.timestamp,
            SnowflakeGenerator::get_timestamp(),
            "Timestamp didn't update correctly"
        );
    }

    #[test]
    fn test_next_sequence_change() {
        let mut snowflake = Snowflake {
            worker_id: 1,
            timestamp: SnowflakeGenerator::get_timestamp(),
            sequence: 0,
        };

        snowflake = snowflake.next();

        // Assert that sequence is incremented
        assert_eq!(snowflake.sequence, 1);

        snowflake = snowflake.next();

        assert_eq!(snowflake.sequence, 2);
    }

    #[test]
    fn test_next_when_sequence_overflows() {
        let time = SnowflakeGenerator::get_timestamp();

        let snowflake = Snowflake {
            worker_id: 1,
            sequence: 0xFFFF, // Maximum sequence value
            timestamp: time,
        };

        let new_snowflake = snowflake.next();

        // Assert that sequence is reset to 0
        assert_eq!(new_snowflake.sequence, 0);

        assert!(new_snowflake.timestamp > time);

        // Assert that wait_next_timestamp method is called
        // Add appropriate assertions based on your implementation
    }

    #[test]
    fn test_next_when_timestamp_is_greater_than_timestamp() {
        let snowflake = Snowflake {
            worker_id: 1,
            timestamp: SnowflakeGenerator::get_timestamp() + 100,
            sequence: 0,
        };

        let snowflake = snowflake.next();

        // Assert that sequence is reset to 0
        assert_eq!(snowflake.sequence, 0);
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
    fn test_snowflake_wait_next_timestamp() {
        let snowflake = Snowflake::new(1);
        let id = snowflake.timestamp;
        let next = SnowflakeGenerator::wait_next_timestamp(SnowflakeGenerator::get_timestamp());
        assert!(
            next > id,
            "Snowflake.wait_next_timestamp didn't return a new timestamp"
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

    #[test]
    fn test_partial_ord() {}
}
