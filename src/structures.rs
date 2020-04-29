
    #[allow(dead_code)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Clone)]
    pub enum SearchCommands {
        Update,
        Search,
        Die
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct SearchCommand {
        pub command: SearchCommands,
        pub person_id: String,
        pub param: String,
        pub result: Option<String>,
        pub result_channel: std::sync::mpsc::Sender<SearchCommand>
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct AppState {
        pub send_channel: async_std::sync::Sender<SearchCommand>
    }