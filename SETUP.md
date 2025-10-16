# MailNow Authentication Setup

## Backend API (Rust)

### Prerequisites
- Rust 1.70+
- PostgreSQL 14+
- Database setup completed

### Setup
1. Navigate to API directory:
```bash
cd api
```

2. Install dependencies:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

The API will start on `http://127.0.0.1:3200`

### Authentication Endpoints

- `POST /auth/signup` - User registration
- `POST /auth/login` - User login
- `POST /auth/verify-email` - Send verification email
- `GET /auth/verify-email?token=<token>` - Verify email token

## Frontend (Next.js)

### Prerequisites
- Node.js 18+
- pnpm

### Setup
1. Navigate to frontend directory:
```bash
cd mailing-fro
```

2. Install dependencies:
```bash
pnpm install
```

3. Run development server:
```bash
pnpm dev
```

The frontend will start on `http://localhost:3000`

## Testing Authentication Flow

1. Start both backend and frontend servers
2. Navigate to `http://localhost:3000`
3. Click "Get Started" or "Sign Up"
4. Fill in registration form
5. Check verification email (demo link will be shown)
6. Complete onboarding process
7. Access dashboard

## API Testing

Use the test file at `mailing-fro/test-auth.html` to test API endpoints directly.

## Environment Variables

### Backend (.env)
```
DATABASE_URL=postgresql://user:pass@localhost/mailnow
PORT=3200
DEBUG=1
JWT_SECRET=your-secret-key
```

### Frontend (.env.local)
```
NEXT_PUBLIC_API_URL=http://127.0.0.1:3200
```