mod util;

use nanoid::nanoid;
use util::UrlExt;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
  let router = Router::new();

  router
    .post_async("/create", |mut req, ctx| async move {
      let id = nanoid!(12);
      let body = req.bytes().await?;
      let kv = ctx.kv("KVTEST")?;
      // todo expr time
      kv.put_bytes(&id, &body)?.execute().await?;
      Response::from_json(&serde_json::json!({ "id": id }))
    })
    .get_async("/read", |req, ctx| async move {
      let id = req
        .url()?
        .param("id")
        .ok_or("Missing id param")?
        .to_string();
      let view = req
        .url()?
        .param("view")
        .unwrap_or("false".into())
        .eq("true");
      let kv = ctx.kv("KVTEST")?;
      let content = kv.get(&id).bytes().await?.ok_or("Paste Not exist")?;
      if view {
        Response::ok(String::from_utf8_lossy(&content).to_string())
      } else {
        Response::from_bytes(content)
      }
    })
    .run(req, env)
    .await
}
