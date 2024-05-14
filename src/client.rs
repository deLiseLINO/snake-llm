use crate::client::models::InputContent;

use self::models::OutputContent;

pub mod groq;
pub mod models;
pub mod ollama;

pub trait ApiClient {
    fn snake_commands(&mut self, input: InputContent) -> Result<OutputContent, String>;
}

#[cfg(test)]
mod tests {

    use rstest::*;
    use rstest_reuse::{self, *};

    use crate::{
        client::models::{Commands, InputContent, OutputContent},
        config,
        models::Direction,
    };

    #[template]
    #[rstest]
    #[case(
    InputContent {
        snake_head_x: 10,
        snake_head_y: 20,
        food_x: 10,
        food_y: 53,
    },
    OutputContent {
        commands: vec![Commands {
            command: Direction::Up,
            repeat: 33,
        }],
    },
)]
    #[case(
    InputContent {
        snake_head_x: 20,
        snake_head_y: 121,
        food_x: 20,
        food_y: 60,
    },
    OutputContent {
        commands: vec![Commands {
            command: Direction::Down,
            repeat: 61,
        }],
    },
)]
    #[case(
    InputContent {
        snake_head_x: 50,
        snake_head_y: 30,
        food_x: 1,
        food_y: 30,
    },
    OutputContent {
        commands: vec![Commands {
            command: Direction::Left,
            repeat: 49,
        }],
    },
)]
    #[case(
    InputContent {
        snake_head_x: 100,
        snake_head_y: 20,
        food_x: 1,
        food_y: 1,
    },
    OutputContent {
        commands: vec![
        Commands {
            command: Direction::Down,
            repeat: 19
        },
        Commands {
            command: Direction::Left,
            repeat: 99,
        },
        ],
    },
)]
    #[case(
    InputContent {
        snake_head_x: 13,
        snake_head_y: 2,
        food_x: 71,
        food_y: 11,
    },
    OutputContent {
        commands: vec![
            Commands {
                command: Direction::Right,
                repeat: 58,
            },
            Commands {
                command: Direction::Up,
                repeat: 9,
            },
        ],
    },
)]
    #[case(
    InputContent {
        snake_head_x: 50,
        snake_head_y: 50,
        food_x: 10,
        food_y: 10,
    },
    OutputContent {
        commands: vec![
            Commands {
                command: Direction::Down,
                repeat: 40,
            },
            Commands {
                command: Direction::Left,
                repeat: 40,
            },
        ],
    },
)]
    #[case(
    InputContent {
        snake_head_x: 20,
        snake_head_y: 20,
        food_x: 40,
        food_y: 41,
    },
    OutputContent {
        commands: vec![
            Commands {
                command: Direction::Right,
                repeat: 20,
            },
            Commands {
                command: Direction::Up,
                repeat: 21,
            },
        ],
    },
)]
    #[case(
    InputContent {
        snake_head_x: 1000,
        snake_head_y: 500,
        food_x: 2000,
        food_y: 1500,
    },
    OutputContent {
        commands: vec![
            Commands {
                command: Direction::Right,
                repeat: 1000,
            },
            Commands {
                command: Direction::Down,
                repeat: 1000,
            },
        ],
    },
)]
    #[case(
    InputContent {
        snake_head_x: 45,
        snake_head_y: 23,
        food_x: 67,
        food_y: 91,
    },
    OutputContent {
        commands: vec![
            Commands {
                command: Direction::Right,
                repeat: 22,
            },
            Commands {
                command: Direction::Up,
                repeat: 68,
            },
        ],
    },
)]
    #[case(
    InputContent {
        snake_head_x: 91,
        snake_head_y: 45,
        food_x: 13,
        food_y: 19,
    },
    OutputContent {
        commands: vec![
            Commands {
                command: Direction::Down,
                repeat: 26,
            },
            Commands {
                command: Direction::Left,
                repeat: 78,
            },
        ],
    },
)]

    fn test_client(#[case] input: InputContent, #[case] expected_output: OutputContent) {}

    #[apply(test_client)]
    fn test_groq(#[case] input: InputContent, #[case] expected_output: OutputContent) {
        use super::groq::GroqClient;

        let groq_cfg = get_groqclient_cfg();

        let mut client = GroqClient::new(groq_cfg.url, groq_cfg.token);
        let res = client.snake_commands(input);

        if let Ok(res) = res {
            let mut expected_commands = expected_output.commands;
            let mut res_commands = res.commands;
            expected_commands.sort();
            res_commands.sort();
            assert_eq!(res_commands, expected_commands);
        } else {
            panic!("Error: {:?}", res);
        }
    }

    fn get_groqclient_cfg() -> config::TokenClient {
        let config = crate::config::parse();
        if let Some(client_cfg) = config.groq_client {
            client_cfg
        } else {
            panic!("Failed to get groq client config");
        }
    }

    #[apply(test_client)]
    fn test_ollama(#[case] input: InputContent, #[case] expected_output: OutputContent) {
        use super::ollama::OllamaClient;

        let ollama_cfg = get_ollama_cfg();

        let mut client = OllamaClient::new(ollama_cfg.url, ollama_cfg.model);
        let res = client.snake_commands(input);

        if let Ok(res) = res {
            let mut expected_commands = expected_output.commands;
            let mut res_commands = res.commands;
            expected_commands.sort();
            res_commands.sort();
            assert_eq!(res_commands, expected_commands);
        } else {
            panic!("Error: {:?}", res);
        }
    }

    fn get_ollama_cfg() -> config::Client {
        let config = crate::config::parse();
        if let Some(client_cfg) = config.ollama_client {
            client_cfg
        } else {
            panic!("Failed to get ollama client config");
        }
    }
}
