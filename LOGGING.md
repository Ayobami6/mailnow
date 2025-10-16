# MailNow API - Error Handling & Logging

## Error Handling

### Centralized Error Types
- `AppError::Database` - Database operation errors
- `AppError::PasswordHash` - Password hashing failures
- `AppError::Jwt` - JWT token errors
- `AppError::Validation` - Input validation errors
- `AppError::Unauthorized` - Authentication failures
- `AppError::UserNotFound` - User lookup failures
- `AppError::UserExists` - Duplicate user registration
- `AppError::Internal` - Generic server errors

### Error Response Format
```json
{
  "status_code": 400,
  "message": "Validation error message",
  "success": false,
  "data": null
}
```

## Logging Levels

### INFO Level
- Server startup/shutdown
- Successful operations
- Request completion
- User authentication success

### WARN Level
- Authentication failures
- Validation errors
- Slow requests (>1s)
- User not found scenarios

### ERROR Level
- Database connection failures
- Password hashing errors
- JWT token errors
- Internal server errors

### DEBUG Level
- Repository operations
- Request details
- Database queries
- Service instantiation

## Log Format Examples

### Request Logging
```
INFO Request started: POST /auth/login from 127.0.0.1 (Anonymous)
INFO Request completed: POST /auth/login -> 200 in 245ms (User:123)
WARN Slow request detected: POST /auth/login took 1.2s
```

### Authentication Events
```
INFO Signup attempt for email: user@example.com
WARN Login failed: invalid password for user: 123
INFO User created successfully with ID: 456
```

### Database Operations
```
DEBUG Creating user with email: user@example.com
INFO User created successfully with ID: 123
ERROR Database error fetching user 456: NotFound
```

## Environment Configuration

### Development
```env
RUST_LOG=actix_web=info,mailnow_api=debug
DEBUG=1
RUST_BACKTRACE=1
```

### Production
```env
RUST_LOG=actix_web=warn,mailnow_api=info
DEBUG=0
```

## Debugging Features

### Request Timer
Automatic timing for operations with warnings for slow queries

### User Context
All logs include user information when available

### Database Tracing
Detailed logging for all database operations

### Error Chain
Full error context with source error information

## Monitoring Integration

Logs are structured for easy integration with:
- ELK Stack (Elasticsearch, Logstash, Kibana)
- Prometheus + Grafana
- CloudWatch Logs
- Datadog

## Performance Monitoring

- Request duration tracking
- Slow query detection
- Database connection pool monitoring
- Memory usage alerts