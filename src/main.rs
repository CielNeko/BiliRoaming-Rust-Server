use actix_web::http::header::ContentType;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use async_channel::{Receiver, Sender};
use biliroaming_rust_server::mods::get_bili_res::{
    get_playurl, get_search, get_season, get_subtitle_th,
};
use biliroaming_rust_server::mods::request::{getwebpage, redis_set};
use biliroaming_rust_server::mods::types::{BiliConfig, SendData};
use chrono::Local;
use deadpool_redis::{Config, Runtime};
use serde_json;
use std::fs::{self, File};
use std::sync::Arc;
use std::thread::spawn;

#[get("/")]
async fn hello() -> impl Responder {
    //println!("{:?}",req.headers().get("Host").unwrap());
    match fs::read_to_string("index.html") {
        Ok(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(value);
        }
        Err(_) => {
            return HttpResponse::Ok()
                .body("Rust server is online. Powered by BiliRoaming-Rust-Server")
        }
    }
}

#[get("/pgc/player/api/playurl")]
async fn zhplayurl_app(req: HttpRequest) -> impl Responder {
    get_playurl(&req, true, false).await
}

#[get("/pgc/player/web/playurl")]
async fn zhplayurl_web(req: HttpRequest) -> impl Responder {
    get_playurl(&req, false, false).await
}

#[get("/intl/gateway/v2/ogv/playurl")]
async fn thplayurl_app(req: HttpRequest) -> impl Responder {
    get_playurl(&req, true, true).await
}

#[get("/x/v2/search/type")]
async fn zhsearch_app(req: HttpRequest) -> impl Responder {
    get_search(&req, true, false).await
}

#[get("/x/web-interface/search/type")]
async fn zhsearch_web(req: HttpRequest) -> impl Responder {
    get_search(&req, false, false).await
}

#[get("/intl/gateway/v2/app/search/type")]
async fn thsearch_app(req: HttpRequest) -> impl Responder {
    get_search(&req, true, true).await //emmmm 油猴脚本也用的这个
}

#[get("/intl/gateway/v2/ogv/view/app/season")]
async fn thseason_app(req: HttpRequest) -> impl Responder {
    get_season(&req, true, true).await
}

#[get("/intl/gateway/v2/app/subtitle")]
async fn thsubtitle_web(req: HttpRequest) -> impl Responder {
    get_subtitle_th(&req, false, true).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("你好喵~");
    let config_file: File;
    match File::open("config.json") {
        Ok(value) => config_file = value,
        Err(_) => {
            println!("缺少配置文件喵");
            std::process::exit(78);
        }
    }
    let config: BiliConfig = serde_json::from_reader(config_file).unwrap();
    let anti_speedtest_cfg = config.clone();
    let woker_num = config.woker_num;
    let port = config.port.clone();

    let (s, r): (Sender<SendData>, Receiver<SendData>) = async_channel::unbounded();
    let bilisender = Arc::new(s);
    let anti_speedtest_redis_cfg = Config::from_url(&config.redis);
    spawn(move || async move{
        let pool = anti_speedtest_redis_cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        loop {
            if let Ok(receive_data) = r.recv().await {
                let dt = Local::now();
                let ts = dt.timestamp_millis() as u64;
                let body_data = match getwebpage(
                    &receive_data.url,
                    &receive_data.proxy_open,
                    &receive_data.proxy_url,
                    &receive_data.user_agent,
                ) {
                    Ok(data) => data,
                    Err(_) => {
                        return HttpResponse::Ok()
                            .content_type(ContentType::plaintext())
                            .body("{\"code\":-6404,\"message\":\"获取播放地址失败喵\"}");
                    }
                };
                let body_data_json: serde_json::Value = serde_json::from_str(&body_data).unwrap();
                let expire_time = match anti_speedtest_cfg.cache.get(&body_data_json["code"].as_i64().unwrap().to_string()) {
                    Some(value) => value,
                    None => anti_speedtest_cfg.cache.get("other").unwrap(),
                };
                let value = format!("{}{body_data}", ts + expire_time * 1000);
                let _: () = redis_set(&pool, &receive_data.key, &value, *expire_time)
                    .await
                    .unwrap_or_default();
            }
        }   
    });
    HttpServer::new(move || {
        let rediscfg = Config::from_url(&config.redis);
        let pool = rediscfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        App::new()
            .app_data((pool, config.clone(), bilisender.clone()))
            .service(hello)
            .service(zhplayurl_app)
            .service(zhplayurl_web)
            .service(thplayurl_app)
            .service(zhsearch_app)
            .service(zhsearch_web)
            .service(thsearch_app)
            .service(thseason_app)
            .service(thsubtitle_web)
    })
    .bind(("0.0.0.0", port))?
    .workers(woker_num)
    .keep_alive(None)
    .run()
    .await
}
