pub mod app {
    pub mod services {
        pub mod indeed_job;
        pub mod indeed_snapshot;
        pub mod job;
        pub mod linkedin_snapshot;
    }
    pub mod ports {
        pub mod driven;
        pub mod driving;
        pub mod types;
    }
}

pub mod actors {
    pub mod driven {
        pub mod brightdata {
            pub mod client;
            pub mod jobs;
            pub mod snapshots;
        }
        pub mod file_saver;
    }
    pub mod driving {
        pub mod cli {
            pub mod commands;
            pub mod handler;
            pub mod snapshot_table;
        }
    }
}

pub mod adapters {
    pub mod driven {
        pub mod indeed_brightdata;
        pub mod linkedin_brightdata;
    }
}
