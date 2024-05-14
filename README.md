
# Snake LLM
A snake game written in rust using [ratatui](https://github.com/ratatui-org/ratatui) library.

![1715450155679](image/README/1715450155679.png)

## Game modes

### 1. Player control mode
In this mode the player controls the snake and can move it in any direction. 

### 2. LLM control mode
In this mode one of the LL Models takes control over the snake. 

## Api providers
### 1. Groq 
Llama3 70b model
### 2. Ollama
Llama3 8b model

## Controls
- arrow keys - change the direction of the snake

- q - quit from the game.
  
- m key - select game mode and provider.

## Config file
In order to use LLM control mode config.yaml file needs to be placed in the root folder.
The following is an example config.yaml file:
```yaml
groq_client:
  url: https://api.groq.com/openai/v1/chat/completions
  token: your-api-key-here

ollama_client:
  url: http://localhost:11434/api/chat
  model: llama3
```

Groq api key can be found [here](https://console.groq.com/keys)

## Build and run
```bash
# run the game
cargo run

# or build a binary and execute it
cargo build --release
./target/release/snake
```