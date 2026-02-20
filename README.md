# Cognito-RS
Playing around with Amazon Cognito and [axum](https://crates.io/crates/axum).

## Setup
### AWS
1. Create an SPA app client in Amazon Cognito
2. Add `https://localhost:5173/signin-callback` to redirect URLs
3. Add `https://localhost:5173/signout-callback` to sign-out URLs

### Local Development
1. Update environment variables in `.env.sample` and rename to `.env`
2. Generate dev cert for HTTPS in `~/.dev-certs/`:
```shell
mkdir ~/.dev-certs
openssl req -x509 -newkey rsa:4096 -nodes -out ~/.dev-certs/cert.pem -keyout ~/.dev-certs/key.pem -days 3650
```

## Run Development
### Backend
```shell
cargo run --release
```

### Frontend
```shell
npm run dev --prefix frontend
```

## Run Production
Set the `VITE_API_PORT` to the same as frontend port. Might need to update the cert code in `vite.config.ts`.
```shell
npm run build --prefix frontend
cargo run --release
```

## Testing
```shell
curl -k https://localhost:9999/api -H "Authorization: Bearer <token>"
```
