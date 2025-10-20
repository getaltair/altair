# AI Service

AI-powered features for Altair Guidance, including task breakdown, smart prioritization, time estimates, and context suggestions.

## Features

- **Task Breakdown**: Break down complex tasks into manageable subtasks using AI
- **Smart Prioritization**: AI-powered task prioritization suggestions
- **Time Estimates**: Intelligent time estimation for tasks
- **Context Suggestions**: Contextual recommendations based on task content

## Supported AI Providers

1. **OpenAI** (GPT-4, GPT-3.5)
2. **Anthropic** (Claude 3.5 Sonnet, Claude 3 Opus)
3. **Ollama** (Local AI models)

## API Endpoints

- `POST /api/ai/breakdown` - Break down a task into subtasks
- `POST /api/ai/prioritize` - Get prioritization suggestions
- `POST /api/ai/estimate` - Get time estimates for tasks
- `POST /api/ai/suggest` - Get contextual suggestions

## Configuration

Set environment variables in `.env`:

```bash
# AI Provider Configuration
AI_PROVIDER=openai  # Options: openai, anthropic, ollama
OPENAI_API_KEY=your-key-here
ANTHROPIC_API_KEY=your-key-here
OLLAMA_BASE_URL=http://localhost:11434

# Model Configuration
OPENAI_MODEL=gpt-4-turbo-preview
ANTHROPIC_MODEL=claude-3-5-sonnet-20241022
OLLAMA_MODEL=llama3

# Service Configuration
AI_REQUEST_TIMEOUT=30
AI_MAX_RETRIES=3
```

## Running the Service

```bash
# Development
uvicorn app.main:app --reload --port 8001

# Production
uvicorn app.main:app --host 0.0.0.0 --port 8001
```

## Testing

```bash
# Run all tests
pytest

# Run with coverage
pytest --cov=app --cov-report=html
```

## License

AGPL-3.0-or-later
