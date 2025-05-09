use crate::{transform::*, PluginConfig};
use swc_core::ecma::{
  parser,
  visit::{as_folder, Fold},
};
use std::env;

mod attributes;
mod children;
mod condition;
mod entry;
mod harmony;
mod looping;
mod shake;
mod wxs;
mod skyline;

pub fn tr() -> impl Fold {
  let config = serde_json::from_str::<PluginConfig>(
    r#"
        {
            "tmpl_prefix": "f0",
            "components": {
                "block": {},
                "script": {},
                "image": {
                    "src": "i.p3",
                    "mode": "xs.b(i.p1,'scaleToFill')",
                    "lazy-load": "xs.b(i.p0,!1)",
                    "binderror": "eh",
                    "bindload": "eh",
                    "bindtouchstart": "eh",
                    "bindtouchmove": "eh",
                    "bindtouchend": "eh",
                    "bindtouchcancel": "eh",
                    "bindlongpress": "eh",
                    "webp": "xs.b(i.p4,false)",
                    "show-menu-by-longpress": "xs.b(i.p2,false)",
                    "style": "i.st",
                    "class": "i.cl",
                    "bindtap": "eh"
                },
                "view": {
                    "hover-class": "xs.b(i.p1,'none')",
                    "hover-stop-propagation": "xs.b(i.p4,!1)",
                    "hover-start-time": "xs.b(i.p2,50)",
                    "hover-stay-time": "xs.b(i.p3,400)",
                    "bindtouchstart": "eh",
                    "bindtouchmove": "eh",
                    "bindtouchend": "eh",
                    "bindtouchcancel": "eh",
                    "bindlongpress": "eh",
                    "animation": "i.p0",
                    "bindanimationstart": "eh",
                    "bindanimationiteration": "eh",
                    "bindanimationend": "eh",
                    "bindtransitionend": "eh",
                    "style": "i.st",
                    "class": "i.cl",
                    "bindtap": "eh"
                },
                "text": {
                    "selectable": "xs.b(i.p1,!1)",
                    "space": "i.p2",
                    "decode": "xs.b(i.p0,!1)",
                    "user-select": "xs.b(i.p3,false)",
                    "style": "i.st",
                    "class": "i.cl",
                    "bindtap": "eh"
                },
                "movable-area": {
                    "scale-area": "xs.b(i.p0,!1)",
                    "style": "i.st",
                    "class": "i.cl",
                    "bindtap": "eh"
                }
            },
            "adapter": {
                "if": "wx:if",
                "else": "wx:else",
                "elseif": "wx:elif",
                "for": "wx:for",
                "forItem": "wx:for-item",
                "forIndex": "wx:for-index",
                "key": "wx:key",
                "xs": "wxs",
                "type": "weapp"
            }
        }"#,
  )
  .unwrap();
  let visitor = TransformVisitor::new(config);
  as_folder(visitor)
}

pub fn get_syntax_config() -> parser::Syntax {
  // 获取当前工作目录
  let manifest_dir = env::current_dir().expect("Failed to get current directory");
  // 设置 CARGO_MANIFEST_DIR 环境变量
  env::set_var("CARGO_MANIFEST_DIR", manifest_dir.to_str().unwrap());
  parser::Syntax::Es(parser::EsConfig {
    jsx: true,
    ..Default::default()
  })
}
