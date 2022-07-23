use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct BiliConfig {
    pub redis : String,
    pub woker_num : usize,
    pub port : u16,
    pub cn_app_playurl_api : String,
    pub tw_app_playurl_api : String,
    pub hk_app_playurl_api : String,
    pub th_app_playurl_api : String,
    pub cn_web_playurl_api : String,
    pub tw_web_playurl_api : String,
    pub hk_web_playurl_api : String,
    pub th_web_playurl_api : String,
    pub cn_app_search_api : String,
    pub tw_app_search_api : String,
    pub hk_app_search_api : String,
    pub th_app_search_api : String,
    pub cn_web_search_api : String,
    pub tw_web_search_api : String,
    pub hk_web_search_api : String,
    pub th_web_search_api : String,
    pub th_app_season_api : String,
    pub th_app_season_sub_api : String,
    pub th_app_season_sub_open : bool,
    pub cn_proxy_url : String,
    pub tw_proxy_url : String,
    pub hk_proxy_url : String,
    pub th_proxy_url : String,
    pub cn_proxy_open : bool,
    pub tw_proxy_open : bool,
    pub hk_proxy_open : bool,
    pub th_proxy_open : bool,
    pub local_wblist : HashMap<String, (bool, bool)>,
    pub search_remake : HashMap<String, String>,
}

pub struct UserCerinfo {
    pub uid: u64,
    pub black:bool,
    pub white:bool,
    pub status_expire_time: u64,
}

impl UserCerinfo {
    pub fn to_json(&self) -> String {
        format!("{{\"uid\":{},\"black\":{},\"white\":{},\"status_expire_time\":{}}}", self.uid,self.black,self.white,self.status_expire_time).to_string()
    }
}

pub struct UserInfo {
    pub access_key: String,
    pub uid: u64,
    pub vip_expire_time: u64,
    pub expire_time: u64,
}

impl UserInfo {
    pub fn to_json(&self) -> String {
        format!("{{\"access_key\":\"{}\",\"uid\":{},\"vip_expire_time\":{},\"expire_time\":{}}}", self.access_key,self.uid,self.vip_expire_time,self.expire_time)
    }
}