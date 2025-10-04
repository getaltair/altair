# Altair Full-Stack Integration Guide

## Overview

This guide shows how to integrate the Flutter frontend with the Python/FastAPI backend for a complete ADHD-friendly project management system.

## Architecture Overview

```
┌──────────────────────────────────────────────────────┐
│                    Flutter Apps                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│  │   iOS    │  │ Android  │  │   Web    │           │
│  └──────────┘  └──────────┘  └──────────┘           │
│        │             │             │                  │
│        └─────────────┼─────────────┘                  │
│                      ▼                                │
│              ┌──────────────┐                         │
│              │  Dio Client  │                         │
│              └──────────────┘                         │
│                      │                                │
│              ┌───────┴────────┐                       │
│              ▼                ▼                       │
│        HTTP Requests    WebSocket                     │
└──────────────┼───────────────┼───────────────────────┘
               │               │
               ▼               ▼
┌──────────────────────────────────────────────────────┐
│                  FastAPI Backend                      │
│  ┌────────────────────────────────────────────────┐  │
│  │            API Gateway (Uvicorn)               │  │
│  └────────────────────────────────────────────────┘  │
│              │                    │                   │
│     ┌────────┴──────┐    ┌───────┴────────┐         │
│     │  REST Routes  │    │ WebSocket Hub  │         │
│     └───────────────┘    └────────────────┘         │
│              │                    │                   │
│     ┌────────┴──────────────────┐│                   │
│     │    Business Logic         ││                   │
│     │  ┌─────────────────────┐  ││                   │
│     │  │  Task Service       │  ││                   │
│     │  │  Auth Service       │  ││                   │
│     │  │  Focus Service      │  ││                   │
│     │  └─────────────────────┘  ││                   │
│     └────────────────────────────┘│                   │
│                      │            │                   │
└──────────────────────┼────────────┼───────────────────┘
                       ▼            ▼
         ┌──────────────────┐  ┌──────────┐
         │   PostgreSQL     │  │  Redis   │
         └──────────────────┘  └──────────┘
```

## File Organization

```
altair/
├── backend/               # Python/FastAPI
│   ├── app/
│   ├── alembic/
│   ├── tests/
│   └── pyproject.toml
├── flutter/              # Flutter frontend
│   ├── lib/
│   ├── web/
│   ├── test/
│   └── pubspec.yaml
├── docker/               # Deployment
│   ├── docker-compose.yml
│   ├── backend.Dockerfile
│   └── flutter.Dockerfile
├── docs/                 # Documentation
│   ├── CLAUDE.md        # Claude Code instructions
│   ├── ALTAIR_PYTHON_CONTEXT.md
│   └── FLUTTER_FRONTEND.md
├── .env.example         # Environment variables
├── .gitignore
└── README.md
```

## Shared Configuration

### Environment Variables

Create `.env` files for both backend and frontend:

```bash
# backend/.env
DATABASE_URL=postgresql://altair:altair@localhost:5432/altair
REDIS_URL=redis://localhost:6379
SECRET_KEY=your-secret-key-change-this-in-production
API_URL=http://localhost:8000
FRONTEND_URL=http://localhost:3000
ENVIRONMENT=development
```

```bash
# flutter/.env
API_BASE_URL=http://localhost:8000
WS_URL=ws://localhost:8000/ws
SENTRY_DSN=optional-for-error-tracking
```

## Backend API Setup

### 1. CORS Configuration for Flutter

```python
# backend/app/main.py
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI(title="Altair API")

# Configure CORS for Flutter development
app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "http://localhost:3000",      # Flutter web dev
        "http://localhost:8080",      # Flutter web alternative
        "http://localhost:*",         # Any localhost port
        "altair://app",             # Mobile deep linking
    ],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
    expose_headers=["X-Total-Count"], # For pagination
)
```

### 2. Authentication Endpoints

```python
# backend/app/api/auth.py
from fastapi import APIRouter, Depends, HTTPException, status
from fastapi.security import OAuth2PasswordBearer, OAuth2PasswordRequestForm
from jose import JWTError, jwt
from passlib.context import CryptContext
from datetime import datetime, timedelta
from typing import Optional

router = APIRouter()
pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")
oauth2_scheme = OAuth2PasswordBearer(tokenUrl="/api/auth/token")

@router.post("/register")
async def register(
    username: str,
    email: str,
    password: str,
    db: Session = Depends(get_db)
):
    # Check if user exists
    if db.query(User).filter(User.email == email).first():
        raise HTTPException(
            status_code=400,
            detail="Email already registered"
        )
    
    # Create user
    hashed_password = pwd_context.hash(password)
    user = User(
        username=username,
        email=email,
        password_hash=hashed_password,
    )
    db.add(user)
    db.commit()
    
    # Generate tokens
    access_token = create_access_token(user.id)
    refresh_token = create_refresh_token(user.id)
    
    return {
        "access_token": access_token,
        "refresh_token": refresh_token,
        "token_type": "bearer",
        "user": {
            "id": str(user.id),
            "username": user.username,
            "email": user.email,
        }
    }

@router.post("/token")
async def login(
    form_data: OAuth2PasswordRequestForm = Depends(),
    db: Session = Depends(get_db)
):
    user = authenticate_user(db, form_data.username, form_data.password)
    if not user:
        raise HTTPException(
            status_code=401,
            detail="Incorrect username or password"
        )
    
    access_token = create_access_token(user.id)
    refresh_token = create_refresh_token(user.id)
    
    return {
        "access_token": access_token,
        "refresh_token": refresh_token,
        "token_type": "bearer"
    }

@router.post("/refresh")
async def refresh_token(
    refresh_token: str,
    db: Session = Depends(get_db)
):
    try:
        payload = jwt.decode(
            refresh_token, 
            settings.SECRET_KEY, 
            algorithms=[settings.ALGORITHM]
        )
        user_id = payload.get("sub")
        if user_id is None:
            raise HTTPException(status_code=401)
    except JWTError:
        raise HTTPException(status_code=401)
    
    new_access_token = create_access_token(user_id)
    return {
        "access_token": new_access_token,
        "token_type": "bearer"
    }
```

### 3. WebSocket Manager

```python
# backend/app/websocket/manager.py
from typing import Dict, Set
from fastapi import WebSocket
import json
import asyncio

class ConnectionManager:
    def __init__(self):
        self.active_connections: Dict[str, Set[WebSocket]] = {}
        self.user_tasks: Dict[str, asyncio.Task] = {}
    
    async def connect(self, websocket: WebSocket, user_id: str):
        await websocket.accept()
        if user_id not in self.active_connections:
            self.active_connections[user_id] = set()
        self.active_connections[user_id].add(websocket)
        
        # Start heartbeat for this connection
        task = asyncio.create_task(self._heartbeat(websocket, user_id))
        self.user_tasks[f"{user_id}:{id(websocket)}"] = task
    
    def disconnect(self, websocket: WebSocket, user_id: str):
        self.active_connections[user_id].discard(websocket)
        if not self.active_connections[user_id]:
            del self.active_connections[user_id]
        
        # Cancel heartbeat task
        task_key = f"{user_id}:{id(websocket)}"
        if task_key in self.user_tasks:
            self.user_tasks[task_key].cancel()
            del self.user_tasks[task_key]
    
    async def _heartbeat(self, websocket: WebSocket, user_id: str):
        try:
            while True:
                await asyncio.sleep(30)
                await websocket.send_json({"type": "ping"})
        except Exception:
            self.disconnect(websocket, user_id)
    
    async def send_personal_message(self, message: dict, user_id: str):
        if user_id in self.active_connections:
            disconnected = set()
            for connection in self.active_connections[user_id]:
                try:
                    await connection.send_json(message)
                except:
                    disconnected.add(connection)
            
            # Clean up disconnected websockets
            for ws in disconnected:
                self.disconnect(ws, user_id)
    
    async def broadcast_task_update(self, task: dict, user_id: str):
        await self.send_personal_message({
            "type": "task_update",
            "data": task
        }, user_id)
    
    async def broadcast_focus_update(self, session: dict, user_id: str):
        await self.send_personal_message({
            "type": "focus_update",
            "data": session
        }, user_id)

manager = ConnectionManager()

# WebSocket endpoint
@app.websocket("/ws/{token}")
async def websocket_endpoint(
    websocket: WebSocket,
    token: str
):
    try:
        # Verify JWT token
        payload = jwt.decode(token, settings.SECRET_KEY, algorithms=[settings.ALGORITHM])
        user_id = payload.get("sub")
        if not user_id:
            await websocket.close(code=1008)
            return
    except JWTError:
        await websocket.close(code=1008)
        return
    
    await manager.connect(websocket, user_id)
    try:
        while True:
            data = await websocket.receive_json()
            
            # Handle different message types
            if data["type"] == "pong":
                continue  # Heartbeat response
            elif data["type"] == "focus_start":
                await handle_focus_start(user_id, data["task_id"])
            elif data["type"] == "task_update":
                await handle_task_update(user_id, data["task"])
            
    except WebSocketDisconnect:
        manager.disconnect(websocket, user_id)
```

## Flutter Frontend Integration

### 1. API Client Setup

```dart
// flutter/lib/data/providers/api_provider.dart
import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

final dioProvider = Provider<Dio>((ref) {
  final dio = Dio(BaseOptions(
    baseUrl: const String.fromEnvironment(
      'API_URL',
      defaultValue: 'http://localhost:8000',
    ),
    connectTimeout: const Duration(seconds: 5),
    receiveTimeout: const Duration(seconds: 10),
  ));
  
  // Add auth interceptor
  dio.interceptors.add(AuthInterceptor(ref));
  
  // Add logging in debug mode
  if (kDebugMode) {
    dio.interceptors.add(LogInterceptor(
      requestBody: true,
      responseBody: true,
    ));
  }
  
  return dio;
});

class AuthInterceptor extends Interceptor {
  final Ref ref;
  final _storage = const FlutterSecureStorage();
  
  AuthInterceptor(this.ref);
  
  @override
  void onRequest(RequestOptions options, RequestInterceptorHandler handler) async {
    // Add auth token to headers
    final token = await _storage.read(key: 'access_token');
    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    handler.next(options);
  }
  
  @override
  void onError(DioException err, ErrorInterceptorHandler handler) async {
    if (err.response?.statusCode == 401) {
      // Token expired, try to refresh
      final refreshToken = await _storage.read(key: 'refresh_token');
      if (refreshToken != null) {
        try {
          final response = await Dio().post(
            '${err.requestOptions.baseUrl}/api/auth/refresh',
            data: {'refresh_token': refreshToken},
          );
          
          // Save new token
          await _storage.write(
            key: 'access_token',
            value: response.data['access_token'],
          );
          
          // Retry original request
          final clonedRequest = await _retry(err.requestOptions);
          return handler.resolve(clonedRequest);
        } catch (e) {
          // Refresh failed, logout user
          ref.read(authProvider.notifier).logout();
        }
      }
    }
    handler.next(err);
  }
  
  Future<Response<dynamic>> _retry(RequestOptions requestOptions) async {
    final options = Options(
      method: requestOptions.method,
      headers: requestOptions.headers,
    );
    
    return Dio().request<dynamic>(
      requestOptions.path,
      data: requestOptions.data,
      queryParameters: requestOptions.queryParameters,
      options: options,
    );
  }
}
```

### 2. WebSocket Connection

```dart
// flutter/lib/data/providers/websocket_provider.dart
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dart:async';
import 'dart:convert';

final websocketProvider = Provider<WebSocketService>((ref) {
  return WebSocketService(ref);
});

class WebSocketService {
  final Ref ref;
  WebSocketChannel? _channel;
  StreamController<Map<String, dynamic>> _messageController = StreamController.broadcast();
  Timer? _heartbeatTimer;
  
  WebSocketService(this.ref);
  
  Stream<Map<String, dynamic>> get messages => _messageController.stream;
  
  Future<void> connect(String token) async {
    final wsUrl = const String.fromEnvironment(
      'WS_URL',
      defaultValue: 'ws://localhost:8000',
    );
    
    _channel = WebSocketChannel.connect(
      Uri.parse('$wsUrl/ws/$token'),
    );
    
    _channel!.stream.listen(
      (message) {
        final data = jsonDecode(message);
        _handleMessage(data);
      },
      onError: (error) {
        print('WebSocket error: $error');
        _reconnect(token);
      },
      onDone: () {
        print('WebSocket closed');
        _reconnect(token);
      },
    );
    
    // Start heartbeat
    _startHeartbeat();
  }
  
  void _handleMessage(Map<String, dynamic> message) {
    if (message['type'] == 'ping') {
      send({'type': 'pong'});
    } else {
      _messageController.add(message);
      
      // Handle specific message types
      switch (message['type']) {
        case 'task_update':
          ref.read(taskProvider.notifier).updateTask(message['data']);
          break;
        case 'focus_update':
          ref.read(focusProvider.notifier).updateSession(message['data']);
          break;
        case 'celebration':
          _showCelebration(message['data']);
          break;
      }
    }
  }
  
  void _startHeartbeat() {
    _heartbeatTimer?.cancel();
    _heartbeatTimer = Timer.periodic(
      const Duration(seconds: 30),
      (_) => send({'type': 'heartbeat'}),
    );
  }
  
  void send(Map<String, dynamic> message) {
    if (_channel != null) {
      _channel!.sink.add(jsonEncode(message));
    }
  }
  
  Future<void> _reconnect(String token) async {
    await Future.delayed(const Duration(seconds: 5));
    connect(token);
  }
  
  void disconnect() {
    _heartbeatTimer?.cancel();
    _channel?.sink.close();
    _messageController.close();
  }
  
  void _showCelebration(Map<String, dynamic> data) {
    // Trigger celebration animation
    // Implementation depends on your UI framework
  }
}
```

### 3. Task Repository

```dart
// flutter/lib/data/repositories/task_repository.dart
import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final taskRepositoryProvider = Provider<TaskRepository>((ref) {
  return TaskRepository(ref.watch(dioProvider), ref.watch(databaseProvider));
});

class TaskRepository {
  final Dio _dio;
  final AppDatabase _db;
  
  TaskRepository(this._dio, this._db);
  
  Future<List<Task>> getTasks({
    TaskState? state,
    int? limit,
    int? offset,
  }) async {
    try {
      // Try to fetch from server
      final response = await _dio.get('/api/tasks', queryParameters: {
        if (state != null) 'state': state.name,
        if (limit != null) 'limit': limit,
        if (offset != null) 'offset': offset,
      });
      
      final tasks = (response.data as List)
          .map((json) => Task.fromJson(json))
          .toList();
      
      // Update local cache
      await _db.updateTasks(tasks);
      
      return tasks;
    } catch (e) {
      // Fallback to local data
      return _db.getTasks(state: state, limit: limit, offset: offset);
    }
  }
  
  Future<Task> quickCapture(String text) async {
    // Save locally first
    final localTask = await _db.quickCapture(text);
    
    // Sync to server in background
    _syncTask(localTask);
    
    return localTask;
  }
  
  Future<void> _syncTask(Task task) async {
    try {
      final response = await _dio.post('/api/tasks/quick-capture', queryParameters: {
        'text': task.title,
      });
      
      // Update local task with server ID
      await _db.updateTaskId(task.id, response.data['id']);
    } catch (e) {
      // Will retry on next sync cycle
      print('Failed to sync task: $e');
    }
  }
  
  Future<Task> updateTaskState(String taskId, TaskState newState) async {
    // Update locally first
    await _db.updateTaskState(taskId, newState);
    
    // Sync to server
    try {
      await _dio.put('/api/tasks/$taskId', data: {
        'state': newState.name,
      });
    } catch (e) {
      // Mark for sync
      await _db.markForSync(taskId);
    }
    
    return _db.getTask(taskId);
  }
  
  Future<List<Task>> decomposeTask(String taskId) async {
    final response = await _dio.post('/api/tasks/$taskId/decompose');
    
    final subtasks = (response.data['subtasks'] as List)
        .map((json) => Task.fromJson(json))
        .toList();
    
    // Save subtasks locally
    await _db.saveSubtasks(taskId, subtasks);
    
    return subtasks;
  }
}
```

## Docker Deployment

### Docker Compose for Development

```yaml
# docker/docker-compose.yml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: altair
      POSTGRES_PASSWORD: altair
      POSTGRES_DB: altair
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
  
  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
  
  backend:
    build:
      context: ../backend
      dockerfile: ../docker/backend.Dockerfile
    ports:
      - "8000:8000"
    environment:
      DATABASE_URL: postgresql://altair:altair@postgres:5432/altair
      REDIS_URL: redis://redis:6379
      SECRET_KEY: development-secret-key
    depends_on:
      - postgres
      - redis
    volumes:
      - ../backend:/app
    command: uvicorn app.main:app --host 0.0.0.0 --port 8000 --reload
  
  flutter-web:
    build:
      context: ../flutter
      dockerfile: ../docker/flutter.Dockerfile
    ports:
      - "3000:80"
    depends_on:
      - backend

volumes:
  postgres_data:
  redis_data:
```

### Backend Dockerfile

```dockerfile
# docker/backend.Dockerfile
FROM python:3.12-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Install UV
RUN pip install uv

# Copy dependency files
COPY pyproject.toml uv.lock ./

# Install dependencies
RUN uv sync --frozen

# Copy application
COPY . .

# Run migrations and start server
CMD alembic upgrade head && uvicorn app.main:app --host 0.0.0.0 --port 8000
```

### Flutter Web Dockerfile

```dockerfile
# docker/flutter.Dockerfile
FROM dart:stable AS build

WORKDIR /app

# Copy Flutter app
COPY . .

# Get dependencies
RUN dart pub get

# Build for web
RUN dart compile exe bin/server.dart -o bin/server

FROM nginx:alpine

# Copy built web app
COPY --from=build /app/build/web /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
```

## Production Deployment

### Railway Deployment

1. **Backend Deployment**:
```bash
# In backend directory
railway init
railway add
railway up
```

2. **Environment Variables**:
```bash
railway variables add DATABASE_URL=$RAILWAY_DATABASE_URL
railway variables add REDIS_URL=$RAILWAY_REDIS_URL
railway variables add SECRET_KEY=$(openssl rand -hex 32)
```

3. **Flutter Web Deployment**:
```bash
# Build Flutter web
cd flutter
flutter build web --release

# Deploy to Vercel/Netlify
vercel --prod
```

### Mobile Deployment

```bash
# Android
flutter build appbundle --release
# Upload to Google Play Console

# iOS
flutter build ios --release
# Upload via Xcode to App Store Connect
```

## Testing Full Stack

### 1. Backend Tests

```python
# backend/tests/test_integration.py
import pytest
from httpx import AsyncClient
from app.main import app

@pytest.mark.asyncio
async def test_full_task_flow():
    async with AsyncClient(app=app, base_url="http://test") as client:
        # Register user
        register_response = await client.post("/api/auth/register", json={
            "username": "testuser",
            "email": "test@example.com",
            "password": "testpass123"
        })
        assert register_response.status_code == 200
        token = register_response.json()["access_token"]
        
        # Quick capture
        headers = {"Authorization": f"Bearer {token}"}
        capture_response = await client.post(
            "/api/tasks/quick-capture",
            params={"text": "Test task"},
            headers=headers
        )
        assert capture_response.status_code == 200
        task_id = capture_response.json()["id"]
        
        # Update state
        update_response = await client.put(
            f"/api/tasks/{task_id}",
            json={"state": "active"},
            headers=headers
        )
        assert update_response.status_code == 200
        assert update_response.json()["state"] == "active"
```

### 2. Flutter Integration Tests

```dart
// flutter/integration_test/app_test.dart
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:altair/main.dart' as app;

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  
  testWidgets('Complete task flow', (tester) async {
    app.main();
    await tester.pumpAndSettle();
    
    // Login
    await tester.enterText(find.byKey(Key('email_field')), 'test@example.com');
    await tester.enterText(find.byKey(Key('password_field')), 'password');
    await tester.tap(find.text('Login'));
    await tester.pumpAndSettle();
    
    // Quick capture
    await tester.tap(find.byIcon(Icons.add));
    await tester.pumpAndSettle();
    
    await tester.enterText(find.byType(TextField), 'Integration test task');
    await tester.testTextInput.receiveAction(TextInputAction.done);
    await tester.pumpAndSettle();
    
    // Verify task appears
    expect(find.text('Integration test task'), findsOneWidget);
    
    // Complete task
    await tester.drag(
      find.text('Integration test task'),
      const Offset(-300, 0),
    );
    await tester.pumpAndSettle();
    
    await tester.tap(find.text('Done'));
    await tester.pumpAndSettle();
    
    // Verify celebration
    expect(find.byIcon(Icons.celebration), findsOneWidget);
  });
}
```

## Monitoring & Analytics

### Error Tracking (Sentry)

```python
# backend/app/main.py
import sentry_sdk
from sentry_sdk.integrations.asgi import SentryAsgiMiddleware

if settings.SENTRY_DSN:
    sentry_sdk.init(
        dsn=settings.SENTRY_DSN,
        environment=settings.ENVIRONMENT,
    )
    app.add_middleware(SentryAsgiMiddleware)
```

```dart
// flutter/lib/main.dart
import 'package:sentry_flutter/sentry_flutter.dart';

void main() async {
  await SentryFlutter.init(
    (options) {
      options.dsn = const String.fromEnvironment('SENTRY_DSN');
      options.environment = kDebugMode ? 'development' : 'production';
    },
    appRunner: () => runApp(AltairApp()),
  );
}
```

## Development Workflow

### 1. Start Backend
```bash
cd backend
uv run uvicorn app.main:app --reload
```

### 2. Start Flutter
```bash
cd flutter
flutter run -d chrome  # For web
flutter run            # For mobile
```

### 3. Watch for Changes
```bash
# Backend
uv run watchdog

# Flutter
flutter run --hot
```

## Common Issues & Solutions

### CORS Issues
- Ensure backend CORS middleware includes Flutter dev server URL
- Check browser console for specific CORS errors
- Use proxy in Flutter for development

### WebSocket Connection Failed
- Verify WebSocket URL format (ws:// vs wss://)
- Check authentication token is valid
- Ensure backend WebSocket endpoint is running

### State Synchronization
- Implement optimistic updates in Flutter
- Use proper error boundaries
- Add retry logic with exponential backoff

## Next Steps

1. **Week 1**: Set up basic auth flow and task CRUD
2. **Week 2**: Implement quick capture and offline sync
3. **Week 3**: Add focus timer with WebSocket updates
4. **Week 4**: Public profile and dogfooding features
5. **Month 2**: Gamification and social features
6. **Month 3**: AI integration and advanced features

This complete integration ensures your Flutter frontend and Python backend work seamlessly together while maintaining ADHD-friendly patterns throughout the stack!
