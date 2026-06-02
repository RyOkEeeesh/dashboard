# 要件定義

## 開発目的
Rustや、linuxアプリケーションの勉強。

## 解決したい課題
- 卓上に時計がなくて不便
- 室温や湿度がわからない
- 予定表などを見ているがあんま見ない
- それぞれの情報を見るためにスマホを見るのが面倒

## ターゲット
- 自分

## やりたいこと
- 現在時刻の表示
- 室温などの表示
- Wether API からの、天気や外気温の取得表示
- Google ログインと Calender API の取得、表示

## MVP
- Googleログイン
- アプリ表示
- アプリ並べ替え

## Googleログイン
```bash
[Raspi (ダッシュボード)]               [Googleのサーバー]               [スマホ / PC]
         │                                    │                               │
         │ 1. 認証リクエスト                   │                               │
         ├───────────────────────────────────►│                               │
         │ 2. コードとURLを返却                │                               │
         ◄────────────────────────────────────┤                               │
         │                                    │                               │
         │ 3. 画面にQRコードとコードを表示       │                               │
         ▼ (画面に表示)                        │                               │
         │                                    │ 4. QRコードをスキャンしてアクセス │
         │                                    │◄──────────────────────────────┤
         │                                    │ 5. 画面にコードを入力＆ログイン   │
         │                                    │◄──────────────────────────────┤
         │                                    │                               │
         │ 6. 「ログイン終わった？」と確認     │                               │
         ├───────────────────────────────────►│                               │
         │    (終わるまで数秒おきにポーリング)   │                               │
         │                                    │                               │
         │ 7. ログイン検知！トークンを返却       │                               │
         ◄────────────────────────────────────┤                               │
         │                                    │                               │
         ▼ 8. 完了！カレンダー表示へ          │                               │
```
```Rust
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, DeviceAuthorizationUrl, TokenUrl};

// 1. クライアントの設定 (Google Cloudで取得したID)
let client = BasicClient::new(
    ClientId::new("YOUR_CLIENT_ID.apps.googleusercontent.com".to_string()),
    None,
    AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
    Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
)
.set_device_authorization_url(
    DeviceAuthorizationUrl::new("https://oauth2.googleapis.com/device/code".to_string()).unwrap(),
);

// 2. Googleに「コードちょうだい」とリクエスト
let details = client
    .exchange_device_code()
    .add_scope(Scope::new("https://www.googleapis.com/auth/calendar.readonly".to_string()))
    .request_async(async_http_client)
    .await?;

// ➔ ここで `details.user_code()` (ABCD-EFGH) と 
//    `details.verification_uri()` (URL) が手に入るので、UI(Slintなど)に渡してQR表示する

// 3. ユーザーがスマホで操作するのを待つ（ポーリング）
let token_result = client
    .exchange_device_access_token(&details)
    .request_async(async_http_client, tokio::time::sleep, None)
    .await;

match token_result {
    Ok(token) => {
        // ➔ ログイン成功！ token.refresh_token() をDBに保存する
    }
    Err(e) => { /* エラー処理 */ }
}
```