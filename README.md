# mock_x_sample

## api-mock
mkdir apps
cd apps
npm create hono@latest
api-mock
bun
cd api-mock
npm run dev

## sample
cargo new sample
cd sample

### 外部モック起動
TWAPI_V2_TWITTER_API_PREFIX_API=http://localhost:3000 cargo run